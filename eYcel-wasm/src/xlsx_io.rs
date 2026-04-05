use std::io::Cursor;

use calamine::{open_workbook_auto_from_rs, Data, Reader};
use chrono::{NaiveDate, NaiveDateTime, Timelike};
use rust_xlsxwriter::{Format, Workbook};

use crate::cell_value::CellValue;
use crate::errors::EYcelError;

/// Detected input format, used to decide output behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Xlsx,
    Xls,
    Csv,
}

impl InputFormat {
    /// Detect format from filename extension.
    pub fn from_filename(filename: &str) -> Self {
        let lower = filename.to_lowercase();
        if lower.ends_with(".csv") {
            InputFormat::Csv
        } else if lower.ends_with(".xls") {
            InputFormat::Xls
        } else {
            InputFormat::Xlsx
        }
    }
}

/// A parsed sheet: name, headers (row 0), and data grid (all rows including headers).
#[derive(Debug, Clone)]
pub struct SheetData {
    pub name: String,
    pub grid: Vec<Vec<CellValue>>,
}

impl SheetData {
    pub fn headers(&self) -> Vec<String> {
        if self.grid.is_empty() {
            return vec![];
        }
        self.grid[0]
            .iter()
            .map(|c| c.to_string_repr())
            .collect()
    }

    pub fn num_cols(&self) -> usize {
        self.grid.first().map(|r| r.len()).unwrap_or(0)
    }

    pub fn column_data(&self, col: usize) -> Vec<CellValue> {
        self.grid
            .iter()
            .skip(1)
            .map(|row| row.get(col).cloned().unwrap_or(CellValue::Empty))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Unified reader — dispatches by filename extension
// ---------------------------------------------------------------------------

/// Read a spreadsheet file from bytes, auto-detecting format from filename.
/// Supports: .xlsx, .xls, .xlsb, .ods (via calamine) and .csv.
/// Formulas are extracted and preserved for Excel formats.
pub fn read_file(bytes: &[u8], filename: &str) -> Result<Vec<SheetData>, EYcelError> {
    let format = InputFormat::from_filename(filename);
    match format {
        InputFormat::Csv => read_csv(bytes, filename),
        InputFormat::Xlsx | InputFormat::Xls => read_excel(bytes),
    }
}

// ---------------------------------------------------------------------------
// Excel reader (xlsx, xls, xlsb, ods) via calamine auto-detect
// ---------------------------------------------------------------------------

/// Read any Excel format from bytes using calamine's auto-detection.
/// Handles xlsx, xls, xlsb, and ods.
pub fn read_excel(bytes: &[u8]) -> Result<Vec<SheetData>, EYcelError> {
    // Own the data so the Cursor has a 'static lifetime — needed for WASM
    let owned = bytes.to_vec();
    let cursor = Cursor::new(owned);
    let mut workbook = open_workbook_auto_from_rs(cursor)
        .map_err(|e| EYcelError::ExcelParse(format!("Failed to open workbook: {}", e)))?;

    let sheet_names: Vec<String> = workbook.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err(EYcelError::ExcelParse("Workbook has no sheets".to_string()));
    }

    let mut sheets = Vec::new();

    for name in &sheet_names {
        // Try to get formulas — not all formats support this, so ignore errors
        let formula_range = workbook.worksheet_formula(name).ok();

        let data_range = match workbook.worksheet_range(name) {
            Ok(r) => r,
            Err(e) => {
                // Skip sheets that can't be read (e.g. chart sheets)
                eprintln!("Skipping sheet '{}': {}", name, e);
                continue;
            }
        };

        let (total_rows, cols) = data_range.get_size();
        if total_rows == 0 || cols == 0 {
            // Empty sheet — keep it but with empty grid
            sheets.push(SheetData {
                name: name.clone(),
                grid: vec![],
            });
            continue;
        }

        let mut grid: Vec<Vec<CellValue>> = Vec::with_capacity(total_rows);

        for (row_idx, data_row) in data_range.rows().enumerate() {
            let mut row_cells = Vec::with_capacity(cols);

            for (col_idx, data_cell) in data_row.iter().enumerate() {
                // Check if this cell has a formula
                let has_formula: Option<String> =
                    formula_range.as_ref().and_then(|fr| {
                        fr.get((row_idx, col_idx)).and_then(|f: &String| {
                            if f.is_empty() {
                                None
                            } else {
                                Some(f.clone())
                            }
                        })
                    });

                let cell_value = if let Some(formula) = has_formula {
                    if formula.starts_with('=') {
                        CellValue::Formula(formula)
                    } else {
                        CellValue::Formula(format!("={}", formula))
                    }
                } else {
                    convert_calamine_cell(data_cell)
                };

                row_cells.push(cell_value);
            }

            grid.push(row_cells);
        }

        sheets.push(SheetData {
            name: name.clone(),
            grid,
        });
    }

    if sheets.is_empty() {
        return Err(EYcelError::ExcelParse("No readable sheets in workbook".to_string()));
    }

    Ok(sheets)
}

// ---------------------------------------------------------------------------
// CSV reader
// ---------------------------------------------------------------------------

/// Read a CSV file from bytes. Returns a single sheet named after the file.
/// CSV has no formulas, so all cells are data.
/// Values are auto-detected as int, float, bool, or string.
pub fn read_csv(bytes: &[u8], filename: &str) -> Result<Vec<SheetData>, EYcelError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)   // We treat row 0 as headers ourselves
        .flexible(true)       // Allow rows with different column counts
        .from_reader(bytes);

    let mut grid: Vec<Vec<CellValue>> = Vec::new();

    for result in reader.records() {
        let record = result
            .map_err(|e| EYcelError::ExcelParse(format!("CSV parse error: {}", e)))?;

        let row: Vec<CellValue> = record
            .iter()
            .map(|field| parse_csv_field(field))
            .collect();

        grid.push(row);
    }

    if grid.is_empty() {
        return Err(EYcelError::ExcelParse("CSV file is empty".to_string()));
    }

    // Use filename stem as sheet name
    let sheet_name = filename
        .rsplit('/')
        .next()
        .unwrap_or(filename)
        .trim_end_matches(".csv")
        .trim_end_matches(".CSV")
        .to_string();

    Ok(vec![SheetData {
        name: if sheet_name.is_empty() {
            "Sheet1".to_string()
        } else {
            sheet_name
        },
        grid,
    }])
}

/// Auto-detect the type of a CSV field value.
fn parse_csv_field(field: &str) -> CellValue {
    let trimmed = field.trim();

    if trimmed.is_empty() {
        return CellValue::Empty;
    }

    // Formula
    if trimmed.starts_with('=') {
        return CellValue::Formula(trimmed.to_string());
    }

    // Boolean
    match trimmed.to_lowercase().as_str() {
        "true" => return CellValue::Bool(true),
        "false" => return CellValue::Bool(false),
        _ => {}
    }

    // Integer
    if let Ok(i) = trimmed.parse::<i64>() {
        return CellValue::Int(i);
    }

    // Float
    if let Ok(f) = trimmed.parse::<f64>() {
        return CellValue::Float(f);
    }

    // Date (common formats)
    if let Ok(d) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        return CellValue::Date(d);
    }
    if let Ok(d) = NaiveDate::parse_from_str(trimmed, "%m/%d/%Y") {
        return CellValue::Date(d);
    }
    if let Ok(d) = NaiveDate::parse_from_str(trimmed, "%d/%m/%Y") {
        return CellValue::Date(d);
    }

    // DateTime
    if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S") {
        return CellValue::DateTime(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S") {
        return CellValue::DateTime(dt);
    }

    // Default: string
    CellValue::String(trimmed.to_string())
}

// ---------------------------------------------------------------------------
// Calamine cell converter
// ---------------------------------------------------------------------------

/// Convert a calamine Data cell to our CellValue.
fn convert_calamine_cell(cell: &Data) -> CellValue {
    match cell {
        Data::Empty => CellValue::Empty,
        Data::String(s) => {
            if s.starts_with('=') {
                CellValue::Formula(s.clone())
            } else {
                CellValue::String(s.clone())
            }
        }
        Data::Int(i) => CellValue::Int(*i),
        Data::Float(f) => CellValue::Float(*f),
        Data::Bool(b) => CellValue::Bool(*b),
        Data::DateTime(excel_dt) => {
            if let Some(ndt) = excel_dt.as_datetime() {
                if ndt.time() == chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap() {
                    CellValue::Date(ndt.date())
                } else {
                    CellValue::DateTime(ndt)
                }
            } else {
                CellValue::Float(excel_dt.as_f64())
            }
        }
        Data::DateTimeIso(s) => {
            if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
                CellValue::DateTime(ndt)
            } else if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                CellValue::Date(nd)
            } else {
                CellValue::String(s.clone())
            }
        }
        Data::DurationIso(s) => CellValue::String(s.clone()),
        Data::Error(_) => CellValue::Empty,
    }
}

// ---------------------------------------------------------------------------
// Column letter utilities
// ---------------------------------------------------------------------------

/// Convert 0-based column index to Excel column letter (0→A, 1→B, ..., 25→Z, 26→AA, ...).
pub fn col_index_to_letter(idx: usize) -> String {
    let mut result = String::new();
    let mut n = idx;
    loop {
        result.insert(0, (b'A' + (n % 26) as u8) as char);
        if n < 26 {
            break;
        }
        n = n / 26 - 1;
    }
    result
}

// ---------------------------------------------------------------------------
// Excel date helpers
// ---------------------------------------------------------------------------

/// Convert NaiveDate to Excel serial date number.
fn naive_date_to_excel(date: NaiveDate) -> f64 {
    let base = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap();
    let diff = date.signed_duration_since(base).num_days();
    let adjusted = if diff >= 59 { diff + 1 } else { diff };
    adjusted as f64
}

/// Convert NaiveDateTime to Excel serial date number.
fn naive_datetime_to_excel(dt: NaiveDateTime) -> f64 {
    let date_part = naive_date_to_excel(dt.date());
    let time = dt.time();
    let secs = time.num_seconds_from_midnight();
    let frac = (secs as f64) / 86400.0;
    date_part + frac
}

// ---------------------------------------------------------------------------
// Writers
// ---------------------------------------------------------------------------

/// Write sheets to xlsx format, returning the bytes.
/// Formulas in CellValue::Formula are written as formulas — preserving them exactly.
pub fn write_xlsx(sheets: &[SheetData]) -> Result<Vec<u8>, EYcelError> {
    let mut workbook = Workbook::new();
    let date_format = Format::new().set_num_format("yyyy-mm-dd");
    let datetime_format = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss");

    for sheet in sheets {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name(&sheet.name)
            .map_err(|e| EYcelError::Io(format!("Failed to set sheet name: {}", e)))?;

        for (row_idx, row) in sheet.grid.iter().enumerate() {
            let r = row_idx as u32;
            for (col_idx, cell) in row.iter().enumerate() {
                let c = col_idx as u16;
                match cell {
                    CellValue::Empty => {}
                    CellValue::String(s) => {
                        let _ = worksheet.write_string(r, c, s);
                    }
                    CellValue::Int(i) => {
                        let _ = worksheet.write_number(r, c, *i as f64);
                    }
                    CellValue::Float(f) => {
                        let _ = worksheet.write_number(r, c, *f);
                    }
                    CellValue::Bool(b) => {
                        let _ = worksheet.write_boolean(r, c, *b);
                    }
                    CellValue::Date(d) => {
                        let excel_date = naive_date_to_excel(*d);
                        let _ =
                            worksheet.write_number_with_format(r, c, excel_date, &date_format);
                    }
                    CellValue::DateTime(dt) => {
                        let excel_dt = naive_datetime_to_excel(*dt);
                        let _ = worksheet.write_number_with_format(
                            r,
                            c,
                            excel_dt,
                            &datetime_format,
                        );
                    }
                    CellValue::Formula(f) => {
                        // Strip leading '=' — rust_xlsxwriter adds it automatically
                        let formula_str = if f.starts_with('=') { &f[1..] } else { f.as_str() };
                        let _ = worksheet.write_formula(r, c, formula_str);
                    }
                }
            }
        }
    }

    let buf = workbook
        .save_to_buffer()
        .map_err(|e| EYcelError::Io(format!("Failed to write xlsx: {}", e)))?;

    Ok(buf)
}

/// Write the first sheet to CSV format, returning UTF-8 bytes.
/// Formulas are written as-is (e.g. "=SUM(A1:A5)").
pub fn write_csv(sheets: &[SheetData]) -> Result<Vec<u8>, EYcelError> {
    let sheet = sheets
        .first()
        .ok_or_else(|| EYcelError::Io("No sheets to write".to_string()))?;

    let mut wtr = csv::Writer::from_writer(Vec::new());

    for row in &sheet.grid {
        let fields: Vec<String> = row
            .iter()
            .map(|cell| match cell {
                CellValue::Empty => String::new(),
                CellValue::String(s) => s.clone(),
                CellValue::Int(i) => i.to_string(),
                CellValue::Float(f) => f.to_string(),
                CellValue::Bool(b) => b.to_string(),
                CellValue::Date(d) => d.format("%Y-%m-%d").to_string(),
                CellValue::DateTime(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                CellValue::Formula(f) => f.clone(),
            })
            .collect();

        wtr.write_record(&fields)
            .map_err(|e| EYcelError::Io(format!("CSV write error: {}", e)))?;
    }

    wtr.into_inner()
        .map_err(|e| EYcelError::Io(format!("CSV flush error: {}", e)))
}
