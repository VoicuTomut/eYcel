//! Tests to verify that xlsx, xls, and csv formats are all properly supported
//! through the Rust core and ready for the web app.

use eycel_wasm::cell_value::CellValue;
use eycel_wasm::decrypt::{decrypt_excel, decrypt_excel_as_csv};
use eycel_wasm::encrypt::encrypt_excel;
use eycel_wasm::formula_handler::extract_formulas;
use eycel_wasm::xlsx_io::{read_csv, read_excel, read_file, write_csv, write_xlsx, InputFormat, SheetData};

// ===========================================================================
// 1. Format detection
// ===========================================================================

#[test]
fn test_format_detection_xlsx() {
    assert_eq!(InputFormat::from_filename("data.xlsx"), InputFormat::Xlsx);
    assert_eq!(InputFormat::from_filename("DATA.XLSX"), InputFormat::Xlsx);
    assert_eq!(InputFormat::from_filename("path/to/file.xlsx"), InputFormat::Xlsx);
}

#[test]
fn test_format_detection_xls() {
    assert_eq!(InputFormat::from_filename("data.xls"), InputFormat::Xls);
    assert_eq!(InputFormat::from_filename("DATA.XLS"), InputFormat::Xls);
    assert_eq!(InputFormat::from_filename("path/to/old_file.xls"), InputFormat::Xls);
}

#[test]
fn test_format_detection_csv() {
    assert_eq!(InputFormat::from_filename("data.csv"), InputFormat::Csv);
    assert_eq!(InputFormat::from_filename("DATA.CSV"), InputFormat::Csv);
    assert_eq!(InputFormat::from_filename("path/to/export.csv"), InputFormat::Csv);
}

#[test]
fn test_format_detection_unknown_defaults_to_xlsx() {
    assert_eq!(InputFormat::from_filename("data.txt"), InputFormat::Xlsx);
    assert_eq!(InputFormat::from_filename("data"), InputFormat::Xlsx);
    assert_eq!(InputFormat::from_filename(""), InputFormat::Xlsx);
}

// ===========================================================================
// 2. read_file dispatcher routes correctly
// ===========================================================================

#[test]
fn test_read_file_routes_csv() {
    let csv = b"Name,Value\nAlice,10\n";
    let sheets = read_file(csv, "test.csv").expect("read_file should route CSV");
    assert_eq!(sheets.len(), 1);
    assert_eq!(sheets[0].grid.len(), 2);
    assert_eq!(sheets[0].grid[0][0], CellValue::String("Name".into()));
}

#[test]
fn test_read_file_routes_xlsx() {
    // Create an xlsx in memory, then read it back through read_file
    let sheet = SheetData {
        name: "S1".to_string(),
        grid: vec![
            vec![CellValue::String("H".into())],
            vec![CellValue::Int(42)],
        ],
    };
    let bytes = write_xlsx(&[sheet]).unwrap();
    let sheets = read_file(&bytes, "test.xlsx").expect("read_file should route xlsx");
    assert_eq!(sheets.len(), 1);
}

#[test]
fn test_read_file_routes_xls_extension_to_excel_reader() {
    // We can't create a real .xls file easily, but we CAN verify that passing
    // an xlsx-format file with a .xls extension goes through read_excel
    // (calamine auto-detects the actual binary format).
    let sheet = SheetData {
        name: "S1".to_string(),
        grid: vec![
            vec![CellValue::String("Col".into())],
            vec![CellValue::Float(1.0)],
        ],
    };
    let bytes = write_xlsx(&[sheet]).unwrap();
    // Even though extension says .xls, calamine auto-detects it as xlsx
    let result = read_file(&bytes, "legacy.xls");
    assert!(result.is_ok(), "Should handle xlsx bytes even with .xls extension");
}

// ===========================================================================
// 3. CSV parsing — type detection
// ===========================================================================

#[test]
fn test_csv_detects_integers() {
    let csv = b"Val\n0\n-5\n999999\n";
    let sheets = read_csv(csv, "ints.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(grid[1][0], CellValue::Int(0));
    assert_eq!(grid[2][0], CellValue::Int(-5));
    assert_eq!(grid[3][0], CellValue::Int(999999));
}

#[test]
fn test_csv_detects_floats() {
    let csv = b"Val\n3.14\n-0.5\n1e3\n";
    let sheets = read_csv(csv, "floats.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(grid[1][0], CellValue::Float(3.14));
    assert_eq!(grid[2][0], CellValue::Float(-0.5));
    assert_eq!(grid[3][0], CellValue::Float(1000.0)); // 1e3
}

#[test]
fn test_csv_detects_booleans() {
    let csv = b"Val\ntrue\nfalse\nTRUE\nFalse\n";
    let sheets = read_csv(csv, "bools.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(grid[1][0], CellValue::Bool(true));
    assert_eq!(grid[2][0], CellValue::Bool(false));
    assert_eq!(grid[3][0], CellValue::Bool(true));
    assert_eq!(grid[4][0], CellValue::Bool(false));
}

#[test]
fn test_csv_detects_dates_iso() {
    let csv = b"Val\n2024-01-15\n2023-12-31\n";
    let sheets = read_csv(csv, "dates.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(
        grid[1][0],
        CellValue::Date(chrono::NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
    );
    assert_eq!(
        grid[2][0],
        CellValue::Date(chrono::NaiveDate::from_ymd_opt(2023, 12, 31).unwrap())
    );
}

#[test]
fn test_csv_detects_datetimes() {
    let csv = b"Val\n2024-01-15 10:30:00\n2024-01-15T10:30:00\n";
    let sheets = read_csv(csv, "datetimes.csv").unwrap();
    let grid = &sheets[0].grid;
    let expected = chrono::NaiveDateTime::parse_from_str("2024-01-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(grid[1][0], CellValue::DateTime(expected));
    assert_eq!(grid[2][0], CellValue::DateTime(expected));
}

#[test]
fn test_csv_detects_strings() {
    let csv = b"Val\nhello world\nfoo bar\n";
    let sheets = read_csv(csv, "strings.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(grid[1][0], CellValue::String("hello world".into()));
    assert_eq!(grid[2][0], CellValue::String("foo bar".into()));
}

// ===========================================================================
// 4. CSV edge cases
// ===========================================================================

#[test]
fn test_csv_empty_cells() {
    let csv = b"A,B,C\n1,,3\n,2,\n";
    let sheets = read_csv(csv, "empty.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(grid[1][0], CellValue::Int(1));
    assert_eq!(grid[1][1], CellValue::Empty);
    assert_eq!(grid[1][2], CellValue::Int(3));
    assert_eq!(grid[2][0], CellValue::Empty);
    assert_eq!(grid[2][1], CellValue::Int(2));
    assert_eq!(grid[2][2], CellValue::Empty);
}

#[test]
fn test_csv_quoted_fields_with_commas() {
    let csv = b"Name,City\n\"Smith, John\",\"New York, NY\"\n";
    let sheets = read_csv(csv, "quoted.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(grid[1][0], CellValue::String("Smith, John".into()));
    assert_eq!(grid[1][1], CellValue::String("New York, NY".into()));
}

#[test]
fn test_csv_unicode() {
    let csv = "Name,City\nJösé,München\n".as_bytes();
    let sheets = read_csv(csv, "unicode.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(grid[1][0], CellValue::String("Jösé".into()));
    assert_eq!(grid[1][1], CellValue::String("München".into()));
}

#[test]
fn test_csv_formulas_preserved() {
    let csv = b"A,B,Total\n10,20,=A2+B2\n30,40,=A3+B3\n";
    let sheets = read_csv(csv, "formulas.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(grid[1][2], CellValue::Formula("=A2+B2".into()));
    assert_eq!(grid[2][2], CellValue::Formula("=A3+B3".into()));
}

#[test]
fn test_csv_single_column() {
    let csv = b"ID\n1\n2\n3\n";
    let sheets = read_csv(csv, "single.csv").unwrap();
    assert_eq!(sheets[0].grid.len(), 4);
    assert_eq!(sheets[0].num_cols(), 1);
}

#[test]
fn test_csv_header_only() {
    let csv = b"A,B,C\n";
    let sheets = read_csv(csv, "headers_only.csv").unwrap();
    assert_eq!(sheets[0].grid.len(), 1); // just the header row
}

#[test]
fn test_csv_empty_file_errors() {
    let csv = b"";
    let result = read_csv(csv, "empty.csv");
    assert!(result.is_err(), "Empty CSV should return error");
}

#[test]
fn test_csv_sheet_name_from_filename() {
    let csv = b"A\n1\n";
    let sheets = read_csv(csv, "my_report.csv").unwrap();
    assert_eq!(sheets[0].name, "my_report");

    let sheets2 = read_csv(csv, "path/to/data_export.csv").unwrap();
    assert_eq!(sheets2[0].name, "data_export");
}

#[test]
fn test_csv_mixed_types_in_column() {
    // A column with ints and strings — should not crash
    let csv = b"Val\n42\nhello\n99\n";
    let sheets = read_csv(csv, "mixed.csv").unwrap();
    let grid = &sheets[0].grid;
    assert_eq!(grid[1][0], CellValue::Int(42));
    assert_eq!(grid[2][0], CellValue::String("hello".into()));
    assert_eq!(grid[3][0], CellValue::Int(99));
}

// ===========================================================================
// 5. CSV write/read roundtrip
// ===========================================================================

#[test]
fn test_csv_write_read_roundtrip() {
    let sheet = SheetData {
        name: "Test".to_string(),
        grid: vec![
            vec![
                CellValue::String("Name".into()),
                CellValue::String("Amount".into()),
                CellValue::String("Date".into()),
                CellValue::String("Active".into()),
            ],
            vec![
                CellValue::String("Alice".into()),
                CellValue::Float(100.5),
                CellValue::Date(chrono::NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()),
                CellValue::Bool(true),
            ],
            vec![
                CellValue::String("Bob".into()),
                CellValue::Int(200),
                CellValue::Date(chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
                CellValue::Bool(false),
            ],
        ],
    };

    // Write to CSV
    let csv_bytes = write_csv(&[sheet]).expect("write_csv");
    let csv_str = String::from_utf8(csv_bytes.clone()).unwrap();

    // Verify CSV content
    assert!(csv_str.contains("Name,Amount,Date,Active"));
    assert!(csv_str.contains("Alice"));
    assert!(csv_str.contains("100.5"));
    assert!(csv_str.contains("2024-06-15"));
    assert!(csv_str.contains("true"));

    // Read back
    let sheets = read_csv(&csv_bytes, "roundtrip.csv").expect("read_csv");
    let grid = &sheets[0].grid;
    assert_eq!(grid.len(), 3); // header + 2 rows
    assert_eq!(grid[0][0], CellValue::String("Name".into()));
    assert_eq!(grid[1][0], CellValue::String("Alice".into()));
}

// ===========================================================================
// 6. CSV full encrypt → decrypt pipeline
// ===========================================================================

#[test]
fn test_csv_encrypt_all_column_types() {
    // CSV with strings (→ hash), numbers (→ scale), categories (→ shuffle), dates (→ offset)
    let csv = b"Name,Score,Department,JoinDate\n\
        Alice,95.5,Engineering,2022-03-15\n\
        Bob,87.0,Engineering,2021-06-01\n\
        Carol,92.3,Sales,2023-01-10\n\
        Dave,78.1,Sales,2020-11-20\n\
        Eve,88.9,Engineering,2022-08-05\n";

    let (encrypted_bytes, rules_yaml) =
        encrypt_excel(csv, "team.csv", None).expect("encrypt CSV");

    // Verify rules contain all columns
    let rules = eycel_wasm::yaml_handler::rules_from_yaml(&rules_yaml).unwrap();
    assert_eq!(rules.metadata.original_filename, "team.csv");
    // 4 column configs + 1 __global_text_map entry
    let real_cols = rules.columns.len() - 1; // subtract __global_text_map
    assert_eq!(real_cols, 4, "Should have 4 columns, got keys: {:?}", rules.columns.keys().collect::<Vec<_>>());

    // Verify encrypted data is different from original
    let enc_sheets = read_excel(&encrypted_bytes).unwrap();
    let enc = &enc_sheets[0].grid;
    assert_ne!(enc[1][0].to_string_repr(), "Alice", "Name should be hashed");

    // Decrypt to xlsx
    let dec_bytes = decrypt_excel(&encrypted_bytes, &rules_yaml, "encrypted.xlsx").unwrap();
    let dec_sheets = read_excel(&dec_bytes).unwrap();
    let dec = &dec_sheets[0].grid;

    // Score should be restored (scale is reversible)
    let score = dec[1][1].as_f64().unwrap();
    assert!(
        (score - 95.5).abs() < 0.01,
        "Score should be ~95.5, got {}",
        score
    );

    // Decrypt to CSV
    let csv_out = decrypt_excel_as_csv(&encrypted_bytes, &rules_yaml, "encrypted.xlsx").unwrap();
    let csv_str = String::from_utf8(csv_out).unwrap();
    let lines: Vec<&str> = csv_str.trim().lines().collect();
    assert_eq!(lines.len(), 6, "Should have header + 5 data rows");
    assert!(lines[0].contains("Name"), "First line should be headers");
}

#[test]
fn test_csv_encrypt_preserves_row_count() {
    let csv = b"X,Y\n1,2\n3,4\n5,6\n7,8\n9,10\n";

    let (enc, rules) = encrypt_excel(csv, "nums.csv", None).unwrap();
    let enc_sheets = read_excel(&enc).unwrap();
    assert_eq!(enc_sheets[0].grid.len(), 6, "Header + 5 data rows");

    let dec = decrypt_excel(&enc, &rules, "encrypted.xlsx").unwrap();
    let dec_sheets = read_excel(&dec).unwrap();
    assert_eq!(dec_sheets[0].grid.len(), 6, "Header + 5 data rows after decrypt");
}

// ===========================================================================
// 7. xlsx through read_file (ensure old path still works)
// ===========================================================================

#[test]
fn test_xlsx_through_read_file_with_formulas() {
    let sheet = SheetData {
        name: "Sheet1".to_string(),
        grid: vec![
            vec![
                CellValue::String("A".into()),
                CellValue::String("B".into()),
                CellValue::String("Sum".into()),
            ],
            vec![
                CellValue::Float(10.0),
                CellValue::Float(20.0),
                CellValue::Formula("=A2+B2".into()),
            ],
        ],
    };

    let bytes = write_xlsx(&[sheet]).unwrap();

    // Read through dispatcher with .xlsx extension
    let sheets = read_file(&bytes, "test.xlsx").unwrap();
    let formulas = extract_formulas(&sheets[0].grid);
    assert_eq!(formulas.len(), 1, "Formula should survive read_file for xlsx");

    // Encrypt through the full pipeline
    let (enc, rules) = encrypt_excel(&bytes, "test.xlsx", None).unwrap();
    let enc_sheets = read_excel(&enc).unwrap();
    let enc_formulas = extract_formulas(&enc_sheets[0].grid);
    assert_eq!(enc_formulas.len(), 1, "Formula should survive encryption");

    // Decrypt
    let dec = decrypt_excel(&enc, &rules, "encrypted.xlsx").unwrap();
    let dec_sheets = read_excel(&dec).unwrap();
    let dec_formulas = extract_formulas(&dec_sheets[0].grid);
    assert_eq!(dec_formulas.len(), 1, "Formula should survive decryption");
    assert!(
        dec_formulas.values().any(|f| f == "=A2+B2"),
        "Formula text should be exactly =A2+B2, got {:?}",
        dec_formulas
    );
}

// ===========================================================================
// 8. xlsx multi-sheet support still works through read_excel
// ===========================================================================

#[test]
fn test_xlsx_multi_sheet_through_read_excel() {
    let sheets = vec![
        SheetData {
            name: "Totals".to_string(),
            grid: vec![
                vec![CellValue::String("Total".into())],
                vec![CellValue::Formula("=Data!A2+Data!A3".into())],
            ],
        },
        SheetData {
            name: "Data".to_string(),
            grid: vec![
                vec![CellValue::String("Value".into())],
                vec![CellValue::Float(100.0)],
                vec![CellValue::Float(200.0)],
            ],
        },
    ];

    let bytes = write_xlsx(&sheets).unwrap();
    let read_sheets = read_excel(&bytes).unwrap();
    assert_eq!(read_sheets.len(), 2, "Both sheets should be read");

    // Encrypt multi-sheet workbook
    let (enc, _rules) = encrypt_excel(&bytes, "multi.xlsx", None).unwrap();
    let enc_sheets = read_excel(&enc).unwrap();
    assert_eq!(enc_sheets.len(), 2, "Both sheets should survive encryption");

    // Check formula preservation across sheets
    let totals_formulas = extract_formulas(&enc_sheets[0].grid);
    assert!(
        totals_formulas.values().any(|f| f.contains("Data!")),
        "Cross-sheet formula should be preserved: {:?}",
        totals_formulas
    );
}

// ===========================================================================
// 9. CSV → encrypt → decrypt → CSV roundtrip preserves data integrity
// ===========================================================================

#[test]
fn test_csv_full_roundtrip_data_integrity() {
    let csv = b"City,Population,Country\n\
        Tokyo,13960000,Japan\n\
        Delhi,11030000,India\n\
        Shanghai,24870000,China\n\
        Tokyo,13960000,Japan\n";

    let (enc, rules) = encrypt_excel(csv, "cities.csv", None).unwrap();

    // Check rules
    let parsed_rules = eycel_wasm::yaml_handler::rules_from_yaml(&rules).unwrap();

    // With new defaults: text→substitute, numbers→keep (no scramble)
    for (key, cfg) in &parsed_rules.columns {
        assert!(
            ["hash", "scale", "shuffle", "offset", "keep", "anonymize", "substitute"].contains(&cfg.transform.as_str()),
            "Column '{}' has invalid transform '{}'",
            key, cfg.transform
        );
    }

    // Decrypt and verify population numbers are restored
    let dec = decrypt_excel(&enc, &rules, "encrypted.xlsx").unwrap();
    let dec_sheets = read_excel(&dec).unwrap();
    let grid = &dec_sheets[0].grid;

    let pop1 = grid[1][1].as_f64().unwrap();
    assert!(
        (pop1 - 13960000.0).abs() < 1.0,
        "Population should be ~13960000, got {}",
        pop1
    );

    let pop3 = grid[3][1].as_f64().unwrap();
    assert!(
        (pop3 - 24870000.0).abs() < 1.0,
        "Population should be ~24870000, got {}",
        pop3
    );

    // CSV export should have correct row count
    let csv_out = decrypt_excel_as_csv(&enc, &rules, "encrypted.xlsx").unwrap();
    let csv_str = String::from_utf8(csv_out).unwrap();
    let lines: Vec<&str> = csv_str.trim().lines().collect();
    assert_eq!(lines.len(), 5, "Header + 4 data rows");
}

// ===========================================================================
// 10. Write CSV handles formulas and empty cells
// ===========================================================================

#[test]
fn test_write_csv_with_formulas_and_empties() {
    let sheet = SheetData {
        name: "Test".to_string(),
        grid: vec![
            vec![
                CellValue::String("A".into()),
                CellValue::String("B".into()),
                CellValue::String("Sum".into()),
            ],
            vec![
                CellValue::Int(10),
                CellValue::Empty,
                CellValue::Formula("=A2+B2".into()),
            ],
        ],
    };

    let csv_bytes = write_csv(&[sheet]).unwrap();
    let csv_str = String::from_utf8(csv_bytes).unwrap();

    // Verify formulas are written as-is
    assert!(csv_str.contains("=A2+B2"), "Formula should appear in CSV");

    // Verify empty cell produces empty field
    let lines: Vec<&str> = csv_str.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    // Second line should have empty middle field: "10,,=A2+B2"
    assert!(
        lines[1].contains("10,,=A2+B2") || lines[1].contains("10,,\"=A2+B2\""),
        "Line should be '10,,=A2+B2', got '{}'",
        lines[1]
    );
}

#[test]
fn test_write_csv_no_sheets_errors() {
    let result = write_csv(&[]);
    assert!(result.is_err(), "Writing CSV with no sheets should error");
}
