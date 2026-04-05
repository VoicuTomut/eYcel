use eycel_wasm::cell_value::CellValue;
use eycel_wasm::encrypt::encrypt_excel;
use eycel_wasm::decrypt::{decrypt_excel, decrypt_excel_as_csv};
use eycel_wasm::xlsx_io::{read_csv, read_excel, write_xlsx, SheetData};
use eycel_wasm::formula_handler::extract_formulas;

/// Build a sample xlsx file in memory with formulas, encrypt, decrypt,
/// and verify formulas are preserved exactly.
#[test]
fn test_encrypt_decrypt_preserves_formulas() {
    // Build a test workbook with formulas
    let sheet = SheetData {
        name: "Sheet1".to_string(),
        grid: vec![
            // Row 0: headers
            vec![
                CellValue::String("Name".into()),
                CellValue::String("Amount".into()),
                CellValue::String("Tax".into()),
                CellValue::String("Total".into()),
            ],
            // Row 1: data + formula
            vec![
                CellValue::String("Alice".into()),
                CellValue::Float(100.0),
                CellValue::Float(10.0),
                CellValue::Formula("=B2+C2".into()),
            ],
            // Row 2: data + formula
            vec![
                CellValue::String("Bob".into()),
                CellValue::Float(200.0),
                CellValue::Float(20.0),
                CellValue::Formula("=B3+C3".into()),
            ],
            // Row 3: formula row
            vec![
                CellValue::String("Total".into()),
                CellValue::Formula("=SUM(B2:B3)".into()),
                CellValue::Formula("=SUM(C2:C3)".into()),
                CellValue::Formula("=SUM(D2:D3)".into()),
            ],
        ],
    };

    // Write to xlsx bytes
    let input_bytes = write_xlsx(&[sheet]).expect("Failed to write test xlsx");

    // Encrypt
    let (encrypted_bytes, rules_yaml) =
        encrypt_excel(&input_bytes, "test.xlsx", None).expect("Encryption failed");

    // Verify encrypted file is valid xlsx
    let encrypted_sheets = read_excel(&encrypted_bytes).expect("Failed to read encrypted xlsx");
    assert_eq!(encrypted_sheets.len(), 1);
    assert_eq!(encrypted_sheets[0].name, "Sheet1");

    // Extract formulas from encrypted file — they should be preserved
    let encrypted_formulas = extract_formulas(&encrypted_sheets[0].grid);
    assert!(
        encrypted_formulas.values().any(|f| f == "=B2+C2" || f == "=B2+C2"),
        "Formulas should be preserved in encrypted file. Found: {:?}",
        encrypted_formulas
    );
    assert!(
        encrypted_formulas.values().any(|f| f == "=SUM(B2:B3)"),
        "SUM formula should be preserved. Found: {:?}",
        encrypted_formulas
    );

    // Verify data was actually transformed (not the same as original)
    let enc_grid = &encrypted_sheets[0].grid;
    // "Alice" should be hashed (not "Alice" anymore)
    let alice_cell = &enc_grid[1][0];
    assert_ne!(
        alice_cell.to_string_repr(),
        "Alice",
        "Name should be encrypted"
    );

    // Decrypt
    let decrypted_bytes =
        decrypt_excel(&encrypted_bytes, &rules_yaml, "encrypted.xlsx").expect("Decryption failed");

    // Verify decrypted file has formulas preserved
    let decrypted_sheets = read_excel(&decrypted_bytes).expect("Failed to read decrypted xlsx");
    let decrypted_formulas = extract_formulas(&decrypted_sheets[0].grid);

    // All original formulas must be present
    assert!(
        decrypted_formulas.values().any(|f| f == "=B2+C2"),
        "Formula =B2+C2 should survive decrypt. Found: {:?}",
        decrypted_formulas
    );
    assert!(
        decrypted_formulas.values().any(|f| f == "=B3+C3"),
        "Formula =B3+C3 should survive decrypt. Found: {:?}",
        decrypted_formulas
    );
    assert!(
        decrypted_formulas.values().any(|f| f == "=SUM(B2:B3)"),
        "Formula =SUM(B2:B3) should survive decrypt. Found: {:?}",
        decrypted_formulas
    );
    assert!(
        decrypted_formulas.values().any(|f| f == "=SUM(C2:C3)"),
        "Formula =SUM(C2:C3) should survive decrypt. Found: {:?}",
        decrypted_formulas
    );
    assert!(
        decrypted_formulas.values().any(|f| f == "=SUM(D2:D3)"),
        "Formula =SUM(D2:D3) should survive decrypt. Found: {:?}",
        decrypted_formulas
    );

    // Count: we had 5 formulas total, all should survive
    assert_eq!(
        decrypted_formulas.len(),
        5,
        "All 5 formulas should be preserved"
    );
}

#[test]
fn test_numeric_roundtrip_reversible() {
    let sheet = SheetData {
        name: "Sheet1".to_string(),
        grid: vec![
            vec![
                CellValue::String("Value".into()),
            ],
            vec![CellValue::Float(42.5)],
            vec![CellValue::Float(100.0)],
            vec![CellValue::Float(-7.3)],
        ],
    };

    let input_bytes = write_xlsx(&[sheet]).expect("write");
    let (encrypted_bytes, rules_yaml) =
        encrypt_excel(&input_bytes, "nums.xlsx", None).expect("encrypt");
    let decrypted_bytes = decrypt_excel(&encrypted_bytes, &rules_yaml, "encrypted.xlsx").expect("decrypt");

    let decrypted = read_excel(&decrypted_bytes).expect("read");
    let grid = &decrypted[0].grid;

    // Scale transform is reversible — values should be restored
    let v1 = grid[1][0].as_f64().expect("should be float");
    let v2 = grid[2][0].as_f64().expect("should be float");
    let v3 = grid[3][0].as_f64().expect("should be float");

    assert!((v1 - 42.5).abs() < 0.01, "Expected ~42.5, got {}", v1);
    assert!((v2 - 100.0).abs() < 0.01, "Expected ~100.0, got {}", v2);
    assert!((v3 - (-7.3)).abs() < 0.01, "Expected ~-7.3, got {}", v3);
}

#[test]
fn test_rules_yaml_valid() {
    let sheet = SheetData {
        name: "Sheet1".to_string(),
        grid: vec![
            vec![
                CellValue::String("Name".into()),
                CellValue::String("Age".into()),
            ],
            vec![
                CellValue::String("Alice".into()),
                CellValue::Float(30.0),
            ],
        ],
    };

    let input_bytes = write_xlsx(&[sheet]).expect("write");
    let (_, rules_yaml) = encrypt_excel(&input_bytes, "test.xlsx", None).expect("encrypt");

    // Rules YAML should be valid and parseable
    let rules = eycel_wasm::yaml_handler::rules_from_yaml(&rules_yaml).expect("parse rules");
    assert_eq!(rules.metadata.original_filename, "test.xlsx");
    assert!(!rules.columns.is_empty());
}

#[test]
fn test_csv_encrypt_decrypt_roundtrip() {
    // Create a CSV file in memory
    let csv_data = b"Name,Amount,Category\nAlice,100.5,Sales\nBob,200.0,Engineering\nCarol,150.75,Sales\n";

    // Encrypt (filename tells the reader it's CSV)
    let (encrypted_bytes, rules_yaml) =
        encrypt_excel(csv_data, "data.csv", None).expect("CSV encryption failed");

    // Encrypted output is xlsx
    let encrypted_sheets = read_excel(&encrypted_bytes).expect("Failed to read encrypted xlsx");
    assert_eq!(encrypted_sheets.len(), 1);
    assert_eq!(encrypted_sheets[0].grid.len(), 4); // header + 3 rows

    // Verify data was transformed
    let enc_grid = &encrypted_sheets[0].grid;
    assert_ne!(
        enc_grid[1][0].to_string_repr(),
        "Alice",
        "Name should be encrypted"
    );

    // Decrypt back to xlsx
    let decrypted_bytes = decrypt_excel(&encrypted_bytes, &rules_yaml, "encrypted.xlsx").expect("Decrypt failed");
    let decrypted_sheets = read_excel(&decrypted_bytes).expect("read decrypted");
    let grid = &decrypted_sheets[0].grid;

    // Numeric values should be restored (scale is reversible)
    let v1 = grid[1][1].as_f64().expect("should be float");
    assert!((v1 - 100.5).abs() < 0.01, "Expected ~100.5, got {}", v1);

    // Decrypt to CSV
    let csv_bytes = decrypt_excel_as_csv(&encrypted_bytes, &rules_yaml, "encrypted.xlsx").expect("CSV decrypt");
    let csv_str = String::from_utf8(csv_bytes).expect("valid UTF-8");
    assert!(csv_str.contains("Name"), "CSV should have headers");
    // CSV should have 4 lines (header + 3 data rows)
    let lines: Vec<&str> = csv_str.trim().lines().collect();
    assert_eq!(lines.len(), 4, "CSV should have 4 lines, got {}", lines.len());
}

#[test]
fn test_csv_parse_types() {
    let csv_data = b"Num,Date,Bool,Text\n42,2024-06-15,true,hello\n100,2023-01-01,false,world\n";

    let sheets = read_csv(csv_data, "types.csv").expect("parse CSV");
    assert_eq!(sheets.len(), 1);
    assert_eq!(sheets[0].name, "types");

    let grid = &sheets[0].grid;
    // Row 0: headers (all strings)
    assert_eq!(grid[0][0], CellValue::String("Num".into()));

    // Row 1: typed values
    assert_eq!(grid[1][0], CellValue::Int(42));
    assert_eq!(
        grid[1][1],
        CellValue::Date(chrono::NaiveDate::from_ymd_opt(2024, 6, 15).unwrap())
    );
    assert_eq!(grid[1][2], CellValue::Bool(true));
    assert_eq!(grid[1][3], CellValue::String("hello".into()));
}
