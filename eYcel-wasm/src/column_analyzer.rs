use std::collections::HashMap;

use crate::cell_value::CellValue;

/// Metadata about a single column, used to decide which transform to apply.
#[derive(Debug, Clone)]
pub struct ColumnMetadata {
    pub col_index: usize,
    pub header: String,
    pub dominant_type: String,
    pub is_categorical: bool,
    pub formula_count: usize,
    pub data_count: usize,
}

/// Detect whether a list of values is categorical (low cardinality).
/// Threshold: unique_count / total_count <= 0.35
pub fn detect_categorical(values: &[String], threshold: f64) -> bool {
    let non_null: Vec<&String> = values.iter().filter(|v| !v.is_empty()).collect();
    if non_null.is_empty() {
        return false;
    }
    let mut unique = std::collections::HashSet::new();
    for v in &non_null {
        unique.insert(v.as_str());
    }
    (unique.len() as f64 / non_null.len() as f64) <= threshold
}

/// Analyze a column of CellValues and return metadata.
pub fn analyze_column(
    col_index: usize,
    header: &str,
    cells: &[CellValue],
) -> ColumnMetadata {
    let mut type_counts: HashMap<&str, usize> = HashMap::new();
    let mut formula_count = 0;
    let mut raw_values: Vec<String> = Vec::new();
    let mut data_count = 0;

    for cell in cells {
        if cell.is_formula() {
            formula_count += 1;
            continue;
        }
        if cell.is_empty() {
            continue;
        }
        let t = cell.type_name();
        *type_counts.entry(t).or_insert(0) += 1;
        raw_values.push(cell.to_string_repr());
        data_count += 1;
    }

    let dominant_type = type_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(t, _)| t.to_string())
        .unwrap_or_else(|| "empty".to_string());

    let mut is_categorical = false;
    let mut final_type = dominant_type;

    if final_type == "string" {
        is_categorical = detect_categorical(&raw_values, 0.35);
        if is_categorical {
            final_type = "categorical".to_string();
        }
    }

    ColumnMetadata {
        col_index,
        header: header.to_string(),
        dominant_type: final_type,
        is_categorical,
        formula_count,
        data_count,
    }
}

/// Auto-detect the best transform for a column based on its metadata.
/// Matches Python's auto_detect_transform exactly.
pub fn auto_detect_transform(meta: &ColumnMetadata) -> &'static str {
    match meta.dominant_type.as_str() {
        "date" => "offset",
        "int" | "float" => "scale",
        "categorical" => "shuffle",
        "string" => "hash",
        _ => "keep",
    }
}
