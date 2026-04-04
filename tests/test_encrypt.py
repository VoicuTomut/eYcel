"""
Tests for the encrypt module.
"""
import pytest
from pathlib import Path
import openpyxl

from src.eYcel.encrypt import (
    encrypt_excel,
    auto_detect_transform,
    generate_output_paths,
)


class TestGenerateOutputPaths:
    def test_encrypted_suffix(self):
        enc, _ = generate_output_paths("/tmp/sales.xlsx")
        assert enc.endswith("_encrypted.xlsx")

    def test_rules_suffix(self):
        _, rules = generate_output_paths("/tmp/sales.xlsx")
        assert rules.endswith("_rules.yaml")

    def test_same_directory(self):
        enc, rules = generate_output_paths("/data/myfile.xlsx")
        assert Path(enc).parent == Path(rules).parent


class TestAutoDetectTransform:
    def test_date_column(self):
        meta = {"dominant_type": "date"}
        assert auto_detect_transform(meta) == "offset"

    def test_float_column(self):
        meta = {"dominant_type": "float"}
        assert auto_detect_transform(meta) == "scale"

    def test_int_column(self):
        meta = {"dominant_type": "int"}
        assert auto_detect_transform(meta) == "scale"

    def test_categorical_column(self):
        meta = {"dominant_type": "categorical"}
        assert auto_detect_transform(meta) == "shuffle"

    def test_string_column(self):
        meta = {"dominant_type": "string"}
        assert auto_detect_transform(meta) == "hash"

    def test_unknown_type_keeps(self):
        meta = {"dominant_type": "empty"}
        assert auto_detect_transform(meta) == "keep"


class TestEncryptExcel:
    def test_creates_output_file(self, sample_excel_path, temp_dir):
        out = str(temp_dir / "enc.xlsx")
        encrypt_excel(str(sample_excel_path), out)
        assert Path(out).exists()

    def test_creates_rules_file(self, sample_excel_path, temp_dir):
        out = str(temp_dir / "enc.xlsx")
        rules_path = encrypt_excel(str(sample_excel_path), out)
        assert Path(rules_path).exists()

    def test_formulas_preserved(self, sample_excel_path, temp_dir):
        out = str(temp_dir / "enc.xlsx")
        encrypt_excel(str(sample_excel_path), out)
        wb_orig = openpyxl.load_workbook(str(sample_excel_path), data_only=False)
        wb_enc = openpyxl.load_workbook(out, data_only=False)
        # Check formula cells in column E
        for row in range(2, 7):
            orig_val = wb_orig.active.cell(row=row, column=5).value
            enc_val = wb_enc.active.cell(row=row, column=5).value
            if orig_val and str(orig_val).startswith("="):
                assert enc_val == orig_val, f"Formula at row {row} changed!"

    def test_data_values_changed(self, sample_excel_path, temp_dir):
        out = str(temp_dir / "enc.xlsx")
        encrypt_excel(str(sample_excel_path), out)
        wb_orig = openpyxl.load_workbook(str(sample_excel_path), data_only=False)
        wb_enc = openpyxl.load_workbook(out, data_only=False)
        # At least one data value in col B (amount) should differ
        orig_vals = [wb_orig.active.cell(row=r, column=2).value for r in range(2, 7)]
        enc_vals  = [wb_enc.active.cell(row=r, column=2).value  for r in range(2, 7)]
        assert orig_vals != enc_vals

    def test_missing_input_raises(self, temp_dir):
        with pytest.raises(FileNotFoundError):
            encrypt_excel(str(temp_dir / "no_such_file.xlsx"), str(temp_dir / "out.xlsx"))

    def test_headers_preserved(self, sample_excel_path, temp_dir):
        out = str(temp_dir / "enc.xlsx")
        encrypt_excel(str(sample_excel_path), out)
        wb = openpyxl.load_workbook(out)
        ws = wb.active
        assert ws["A1"].value == "customer_id"
        assert ws["B1"].value == "amount"
