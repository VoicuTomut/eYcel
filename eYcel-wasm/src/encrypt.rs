use std::collections::HashMap;

use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::cell_value::CellValue;
use crate::errors::EYcelError;
use crate::formula_handler::{clear_formula_cells, extract_formulas};
use crate::transformations::*;
use crate::xlsx_io::{col_index_to_letter, read_file, write_xlsx, SheetData};
use crate::yaml_handler::{ColumnConfig, Metadata, Rules};

/// Options controlling encryption behavior.
#[derive(Debug, Clone)]
pub struct EncryptOptions {
    /// If true, numeric columns are scaled by a random factor (reversible).
    /// If false (default), numbers are kept unchanged so AI can work with them.
    pub scramble_numbers: bool,
    /// If true, dates are offset by random days (reversible).
    /// If false (default), dates are kept unchanged.
    pub scramble_dates: bool,
}

impl Default for EncryptOptions {
    fn default() -> Self {
        Self {
            scramble_numbers: false,
            scramble_dates: false,
        }
    }
}

/// Apply the correct forward transform to a single cell value.
fn transform_cell(
    value: CellValue,
    config: &ColumnConfig,
    global_text_map: &HashMap<String, String>,
    rng: &mut SmallRng,
) -> CellValue {
    // Text cells are ALWAYS substituted via the global map, regardless of column transform.
    // This ensures headers, titles, and text in numeric columns all get hidden.
    if let CellValue::String(ref s) = value {
        if let Some(fake) = global_text_map.get(s) {
            return CellValue::String(fake.clone());
        }
        return value;
    }

    match config.transform.as_str() {
        "offset" => {
            if value.has_year() {
                match value {
                    CellValue::Date(d) => {
                        CellValue::Date(transform_offset_date(d, config.offset_days.unwrap_or(0)))
                    }
                    CellValue::DateTime(dt) => CellValue::DateTime(transform_offset_datetime(
                        dt,
                        config.offset_days.unwrap_or(0),
                    )),
                    _ => value,
                }
            } else if let Some(f) = value.as_f64() {
                CellValue::Float(transform_offset_number(f, config.offset.unwrap_or(0.0)))
            } else {
                value
            }
        }
        "scale" => {
            if let Some(f) = value.as_f64() {
                match transform_scale(f, config.factor.unwrap_or(1.0)) {
                    Ok(v) => CellValue::Float(v),
                    Err(_) => value,
                }
            } else {
                value
            }
        }
        "shuffle" => {
            if let Some(mapping) = &config.mapping {
                CellValue::String(transform_shuffle(&value.to_string_repr(), mapping))
            } else {
                value
            }
        }
        "anonymize" => {
            let col_type = config.col_type.as_deref().unwrap_or("string");
            transform_anonymize(&value, col_type, rng)
        }
        _ => value, // "keep" or unknown
    }
}

/// Replace text literals inside a formula string using the global text map.
/// E.g. `=COUNTIF(A:A, "Sales")` → `=COUNTIF(A:A, "Bako")` if "Sales"→"Bako".
fn substitute_text_in_formula(formula: &str, text_map: &HashMap<String, String>) -> String {
    let mut result = String::with_capacity(formula.len());
    let chars: Vec<char> = formula.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '"' {
            // Found start of a string literal — extract it
            let start = i + 1;
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                i += 1;
            }
            let end = i;
            let literal: String = chars[start..end].iter().collect();

            // Check if this literal is in our text map
            if let Some(replacement) = text_map.get(&literal) {
                result.push('"');
                result.push_str(replacement);
                result.push('"');
            } else {
                result.push('"');
                result.push_str(&literal);
                result.push('"');
            }

            if i < chars.len() {
                i += 1; // skip closing quote
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// Encrypt a spreadsheet file (xlsx, xls, or csv).
///
/// **Default behavior:**
/// - ALL text in ALL cells (including headers, titles) → consistent substitution
/// - Text inside formulas (e.g. "Sales" in `=COUNTIF(A:A,"Sales")`) → also substituted
/// - Numbers → kept unchanged (AI needs real numbers for formulas)
/// - Dates → kept unchanged
/// - Formulas → structure preserved, text literals inside them updated
///
/// Use `options` to optionally scramble numbers and dates too.
pub fn encrypt_excel(
    input_bytes: &[u8],
    filename: &str,
    custom_rules: Option<&Rules>,
) -> Result<(Vec<u8>, String), EYcelError> {
    encrypt_excel_with_options(input_bytes, filename, custom_rules, &EncryptOptions::default())
}

pub fn encrypt_excel_with_options(
    input_bytes: &[u8],
    filename: &str,
    custom_rules: Option<&Rules>,
    options: &EncryptOptions,
) -> Result<(Vec<u8>, String), EYcelError> {
    let mut rng = SmallRng::from_entropy();

    // 1. Read the file
    let mut sheets = read_file(input_bytes, filename)?;

    // 2. First pass: collect ALL unique text values across ALL cells in ALL sheets
    //    (including headers, titles, and text literals inside formulas)
    let global_text_map = build_global_map_from_sheets(&sheets);

    let mut all_column_configs: HashMap<String, ColumnConfig> = HashMap::new();
    let mut sheet_names: Vec<String> = Vec::new();

    for sheet in &mut sheets {
        sheet_names.push(sheet.name.clone());

        if sheet.grid.is_empty() {
            continue;
        }

        let num_cols = sheet.num_cols();

        // 3. Build per-column transform configs
        //    Analyze ALL data in the column (all rows) to detect type
        let mut col_configs: Vec<(usize, ColumnConfig)> = Vec::new();

        for col_idx in 0..num_cols {
            let col_letter = col_index_to_letter(col_idx);
            let col_key = format!("{}!{}", sheet.name, col_letter);

            // Collect all cells in this column (all rows)
            let col_data: Vec<CellValue> = sheet
                .grid
                .iter()
                .map(|row| row.get(col_idx).cloned().unwrap_or(CellValue::Empty))
                .collect();

            let cfg = if let Some(rules) = custom_rules {
                if let Some(c) = rules.columns.get(&col_key) {
                    c.clone()
                } else if let Some(c) = rules.columns.get(&col_letter) {
                    c.clone()
                } else {
                    auto_config_for_column(&col_data, options, &mut rng)
                }
            } else {
                auto_config_for_column(&col_data, options, &mut rng)
            };

            all_column_configs.insert(col_key, cfg.clone());
            col_configs.push((col_idx, cfg));
        }

        // 4. Extract formulas BEFORE any transformation
        let formula_map = extract_formulas(&sheet.grid);

        // 5. Clear formula cells so transforms don't touch them
        clear_formula_cells(&mut sheet.grid, &formula_map);

        // 6. Transform ALL cells in ALL rows (no row is special)
        for row_idx in 0..sheet.grid.len() {
            for (col_idx, cfg) in &col_configs {
                if *col_idx < sheet.grid[row_idx].len() {
                    let cell = &sheet.grid[row_idx][*col_idx];
                    if cell.is_empty() {
                        continue;
                    }
                    let transformed =
                        transform_cell(cell.clone(), cfg, &global_text_map, &mut rng);
                    sheet.grid[row_idx][*col_idx] = transformed;
                }
            }
        }

        // 7. Reinject formulas, but with text substitutions applied inside them
        for (&(row, col), formula) in &formula_map {
            if row < sheet.grid.len() && col < sheet.grid[row].len() {
                let updated_formula = substitute_text_in_formula(formula, &global_text_map);
                sheet.grid[row][col] = CellValue::Formula(updated_formula);
            }
        }
    }

    // 8. Write encrypted workbook
    let encrypted_bytes = write_xlsx(&sheets)?;

    // 9. Store the global text map in rules (inverse: fake→original) for decryption
    let inverse_map: HashMap<String, String> = global_text_map
        .iter()
        .map(|(orig, fake)| (fake.clone(), orig.clone()))
        .collect();
    all_column_configs.insert(
        "__global_text_map".to_string(),
        ColumnConfig {
            transform: "substitute".to_string(),
            salt: None,
            factor: None,
            offset: None,
            offset_days: None,
            mapping: Some(inverse_map),
            col_type: None,
        },
    );

    // 10. Generate rules YAML
    let timestamp = chrono::Utc::now().to_rfc3339();
    let rules = Rules {
        metadata: Metadata {
            original_filename: filename.to_string(),
            timestamp,
            sheets: sheet_names,
        },
        columns: all_column_configs,
    };

    let rules_yaml = crate::yaml_handler::rules_to_yaml(&rules)
        .map_err(|e| EYcelError::Encryption(e))?;

    Ok((encrypted_bytes, rules_yaml))
}

/// Collect ALL unique text values from ALL cells in ALL sheets.
/// Also extracts text literals from inside formulas.
fn build_global_map_from_sheets(sheets: &[SheetData]) -> HashMap<String, String> {
    let mut unique_texts: Vec<String> = Vec::new();

    for sheet in sheets {
        for row in &sheet.grid {
            for cell in row {
                match cell {
                    CellValue::String(s) if !s.is_empty() => {
                        if !unique_texts.contains(s) {
                            unique_texts.push(s.clone());
                        }
                    }
                    CellValue::Formula(f) => {
                        // Extract text literals from formulas (strings inside quotes)
                        for literal in extract_formula_literals(f) {
                            if !literal.is_empty() && !unique_texts.contains(&literal) {
                                unique_texts.push(literal);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    build_global_text_map(&unique_texts)
}

/// Extract all string literals (text between double quotes) from a formula.
fn extract_formula_literals(formula: &str) -> Vec<String> {
    let mut literals = Vec::new();
    let chars: Vec<char> = formula.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '"' {
            i += 1;
            let start = i;
            while i < chars.len() && chars[i] != '"' {
                i += 1;
            }
            let literal: String = chars[start..i].iter().collect();
            if !literal.is_empty() {
                literals.push(literal);
            }
            i += 1; // skip closing quote
        } else {
            i += 1;
        }
    }

    literals
}

/// Auto-detect the best transform for a column.
fn auto_config_for_column(
    col_data: &[CellValue],
    options: &EncryptOptions,
    rng: &mut SmallRng,
) -> ColumnConfig {
    // Detect dominant type
    let mut type_counts: HashMap<&str, usize> = HashMap::new();
    for cell in col_data {
        if !cell.is_empty() && !cell.is_formula() {
            *type_counts.entry(cell.type_name()).or_insert(0) += 1;
        }
    }

    let dominant = type_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(t, _)| *t)
        .unwrap_or("empty");

    match dominant {
        "string" => ColumnConfig {
            transform: "substitute".to_string(),
            salt: None,
            factor: None,
            offset: None,
            offset_days: None,
            mapping: None,
            col_type: None,
        },
        "int" | "float" => {
            if options.scramble_numbers {
                ColumnConfig {
                    transform: "scale".to_string(),
                    salt: None,
                    factor: Some(random_factor(rng)),
                    offset: None,
                    offset_days: None,
                    mapping: None,
                    col_type: None,
                }
            } else {
                ColumnConfig {
                    transform: "keep".to_string(),
                    salt: None,
                    factor: None,
                    offset: None,
                    offset_days: None,
                    mapping: None,
                    col_type: None,
                }
            }
        }
        "date" => {
            if options.scramble_dates {
                ColumnConfig {
                    transform: "offset".to_string(),
                    salt: None,
                    factor: None,
                    offset: Some(random_offset(rng)),
                    offset_days: Some(random_day_offset(rng)),
                    mapping: None,
                    col_type: None,
                }
            } else {
                ColumnConfig {
                    transform: "keep".to_string(),
                    salt: None,
                    factor: None,
                    offset: None,
                    offset_days: None,
                    mapping: None,
                    col_type: None,
                }
            }
        }
        _ => ColumnConfig {
            transform: "keep".to_string(),
            salt: None,
            factor: None,
            offset: None,
            offset_days: None,
            mapping: None,
            col_type: None,
        },
    }
}
