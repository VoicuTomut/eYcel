use chrono::{NaiveDate, NaiveDateTime};

/// Unified cell value type used throughout eYcel.
/// Bridges between calamine's read types and rust_xlsxwriter's write types.
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Empty,
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Formula(String),
}

impl CellValue {
    /// Check if this cell contains a formula (starts with '=').
    pub fn is_formula(&self) -> bool {
        matches!(self, CellValue::Formula(_))
    }

    /// Check if the cell is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, CellValue::Empty)
    }

    /// Convert to string representation for hashing/shuffle lookups.
    pub fn to_string_repr(&self) -> String {
        match self {
            CellValue::Empty => String::new(),
            CellValue::String(s) => s.clone(),
            CellValue::Int(i) => i.to_string(),
            CellValue::Float(f) => f.to_string(),
            CellValue::Bool(b) => b.to_string(),
            CellValue::Date(d) => d.to_string(),
            CellValue::DateTime(dt) => dt.to_string(),
            CellValue::Formula(f) => f.clone(),
        }
    }

    /// Detect the type name (matching Python's detect_cell_type).
    pub fn type_name(&self) -> &'static str {
        match self {
            CellValue::Empty => "empty",
            CellValue::String(_) => "string",
            CellValue::Int(_) => "int",
            CellValue::Float(_) => "float",
            CellValue::Bool(_) => "boolean",
            CellValue::Date(_) | CellValue::DateTime(_) => "date",
            CellValue::Formula(_) => "formula",
        }
    }

    /// Check if this value has a year (is a date type) — matches Python's hasattr(value, "year").
    pub fn has_year(&self) -> bool {
        matches!(self, CellValue::Date(_) | CellValue::DateTime(_))
    }

    /// Try to extract as f64 for numeric transforms.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            CellValue::Int(i) => Some(*i as f64),
            CellValue::Float(f) => Some(*f),
            _ => None,
        }
    }
}

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}
