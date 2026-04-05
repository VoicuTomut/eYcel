use crate::cell_value::CellValue;
use crate::errors::EYcelError;
use crate::formula_handler::{clear_formula_cells, extract_formulas};
use crate::transformations::*;
use crate::xlsx_io::{col_index_to_letter, read_file, write_csv, write_xlsx};
use crate::yaml_handler::{rules_from_yaml, ColumnConfig, Rules};

/// Apply the correct reverse transform to a single cell value.
/// `inverse_text_map` is the global fake→original mapping for "substitute" transforms.
fn reverse_cell(
    value: CellValue,
    config: &ColumnConfig,
    inverse_text_map: &std::collections::HashMap<String, String>,
) -> CellValue {
    // Text cells that exist in the global inverse map are always reversed.
    if let CellValue::String(ref s) = value {
        if let Some(original) = inverse_text_map.get(s) {
            return CellValue::String(original.clone());
        }
        // Fall through to column-specific transform (e.g. shuffle)
    }

    match config.transform.as_str() {
        "hash" | "anonymize" => {
            // One-way transforms — cannot reverse, return as-is
            value
        }
        "offset" => {
            if value.has_year() {
                match value {
                    CellValue::Date(d) => {
                        CellValue::Date(reverse_offset_date(d, config.offset_days.unwrap_or(0)))
                    }
                    CellValue::DateTime(dt) => CellValue::DateTime(reverse_offset_datetime(
                        dt,
                        config.offset_days.unwrap_or(0),
                    )),
                    _ => value,
                }
            } else if let Some(f) = value.as_f64() {
                CellValue::Float(reverse_offset_number(f, config.offset.unwrap_or(0.0)))
            } else {
                value
            }
        }
        "scale" => {
            if let Some(f) = value.as_f64() {
                match reverse_scale(f, config.factor.unwrap_or(1.0)) {
                    Ok(v) => CellValue::Float(v),
                    Err(_) => value,
                }
            } else {
                value
            }
        }
        "shuffle" => {
            if let Some(mapping) = &config.mapping {
                CellValue::String(reverse_shuffle(&value.to_string_repr(), mapping))
            } else {
                value
            }
        }
        _ => value, // "keep" or unknown
    }
}

/// Build a column-letter → column index map for a given sheet.
/// Rules use "SheetName!ColLetter" as keys, so we build both forms.
fn build_col_key_map(sheet_name: &str, num_cols: usize) -> std::collections::HashMap<String, usize> {
    let mut map = std::collections::HashMap::new();
    for idx in 0..num_cols {
        let letter = col_index_to_letter(idx);
        // Match "Sheet1!A" format (primary)
        map.insert(format!("{}!{}", sheet_name, letter), idx);
        // Also match bare "A" format (fallback for single-sheet files)
        map.insert(letter, idx);
    }
    map
}

/// Decrypt an encrypted file using rules.
///
/// Accepts xlsx, xls, or csv input (detected from filename).
/// Returns decrypted xlsx bytes.
///
/// **Formula guarantee:** all formulas are extracted before reverse-transformation
/// and reinjected after, so they are preserved 100%.
pub fn decrypt_excel(encrypted_bytes: &[u8], rules_yaml: &str, filename: &str) -> Result<Vec<u8>, EYcelError> {
    let sheets = decrypt_to_sheets(encrypted_bytes, rules_yaml, filename)?;
    write_xlsx(&sheets)
}

/// Decrypt and return as CSV bytes (first sheet only).
/// Accepts xlsx, xls, or csv input (detected from filename).
pub fn decrypt_excel_as_csv(encrypted_bytes: &[u8], rules_yaml: &str, filename: &str) -> Result<Vec<u8>, EYcelError> {
    let sheets = decrypt_to_sheets(encrypted_bytes, rules_yaml, filename)?;
    write_csv(&sheets)
}

/// Core decryption logic — returns decrypted SheetData.
fn decrypt_to_sheets(
    encrypted_bytes: &[u8],
    rules_yaml: &str,
    filename: &str,
) -> Result<Vec<crate::xlsx_io::SheetData>, EYcelError> {
    // 1. Parse and validate rules
    let rules: Rules =
        rules_from_yaml(rules_yaml).map_err(|e| EYcelError::Decryption(e))?;

    // 2. Extract the global inverse text map (fake→original) if present
    let inverse_text_map = rules
        .columns
        .get("__global_text_map")
        .and_then(|cfg| cfg.mapping.clone())
        .unwrap_or_default();

    // 3. Read encrypted file — auto-detect format from filename
    let mut sheets = read_file(encrypted_bytes, filename)?;

    for sheet in &mut sheets {
        if sheet.grid.is_empty() {
            continue;
        }

        let col_map = build_col_key_map(&sheet.name, sheet.num_cols());

        // 4. Extract formulas BEFORE any reverse transformation
        let formula_map = extract_formulas(&sheet.grid);

        // 5. Clear formula cells
        clear_formula_cells(&mut sheet.grid, &formula_map);

        // 6. Reverse-transform ALL cells in ALL rows (no row is special)
        for (col_key, cfg) in &rules.columns {
            if col_key == "__global_text_map" {
                continue;
            }
            if let Some(&col_idx) = col_map.get(col_key) {
                for row_idx in 0..sheet.grid.len() {
                    if col_idx < sheet.grid[row_idx].len() {
                        let cell = &sheet.grid[row_idx][col_idx];
                        if cell.is_empty() {
                            continue;
                        }
                        let restored = reverse_cell(cell.clone(), cfg, &inverse_text_map);
                        sheet.grid[row_idx][col_idx] = restored;
                    }
                }
            }
        }

        // 7. Reinject formulas with text substitutions reversed
        for (&(row, col), formula) in &formula_map {
            if row < sheet.grid.len() && col < sheet.grid[row].len() {
                let restored_formula = reverse_text_in_formula(formula, &inverse_text_map);
                sheet.grid[row][col] = CellValue::Formula(restored_formula);
            }
        }
    }

    Ok(sheets)
}

/// Replace fake text back to original inside formula strings.
fn reverse_text_in_formula(formula: &str, inverse_map: &std::collections::HashMap<String, String>) -> String {
    let mut result = String::with_capacity(formula.len());
    let chars: Vec<char> = formula.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '"' {
            let start = i + 1;
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                i += 1;
            }
            let literal: String = chars[start..i].iter().collect();

            result.push('"');
            if let Some(original) = inverse_map.get(&literal) {
                result.push_str(original);
            } else {
                result.push_str(&literal);
            }
            result.push('"');

            if i < chars.len() {
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// Validate rules YAML and return structured info.
/// Used by the web UI's "Validate" tab.
pub fn validate_rules_yaml(rules_yaml: &str) -> Result<Rules, String> {
    rules_from_yaml(rules_yaml)
}
