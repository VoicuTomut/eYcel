"""
Tests for the decrypt module.
"""
import pytest
from pathlib import Path

from src.eYcel.decrypt import (
    decrypt_excel,
    load_and_validate_rules,
    apply_reverse_transform,
)


class TestLoadAndValidateRules:
    def test_loads_valid_rules(self, temp_dir, sample_rules):
        from src.eYcel.yaml_handler import save_rules
        path = str(temp_dir / "rules.yaml")
        save_rules(sample_rules, path)
        loaded = load_and_validate_rules(path)
        assert loaded["metadata"]["original_filename"] == "test.xlsx"

    def test_missing_file_raises(self, temp_dir):
        with pytest.raises(FileNotFoundError):
            load_and_validate_rules(str(temp_dir / "ghost.yaml"))

    def test_invalid_rules_raises(self, temp_dir):
        path = temp_dir / "bad.yaml"
        path.write_text("columns: [not, a, dict]")
        with pytest.raises(ValueError):
            load_and_validate_rules(str(path))


class TestApplyReverseTransform:
    def test_scale_reversed(self):
        data = [100.0, 200.0]
        restored = apply_reverse_transform(data, {"transform": "scale", "factor": 0.5})
        assert abs(restored[0] - 200.0) < 1e-9
        assert abs(restored[1] - 400.0) < 1e-9

    def test_offset_number_reversed(self):
        data = [150.0, 250.0]
        restored = apply_reverse_transform(data, {"transform": "offset", "offset": 50.0})
        assert abs(restored[0] - 100.0) < 1e-9

    def test_shuffle_reversed(self):
        mapping = {"Electronics": "Cat_0", "Clothing": "Cat_1"}
        data = ["Cat_0", "Cat_1"]
        restored = apply_reverse_transform(data, {"transform": "shuffle", "mapping": mapping})
        assert restored[0] == "Electronics"
        assert restored[1] == "Clothing"

    def test_none_values_preserved(self):
        data = [None, 100.0, None]
        restored = apply_reverse_transform(data, {"transform": "scale", "factor": 2.0})
        assert restored[0] is None
        assert restored[2] is None

    def test_hash_not_reversed(self):
        # hash is one-way — value stays as-is
        data = ["abc123def456"]
        restored = apply_reverse_transform(data, {"transform": "hash", "salt": "x"})
        assert restored[0] == "abc123def456"


class TestDecryptExcel:
    def test_creates_output(self, sample_excel_path, temp_dir):
        from src.eYcel.encrypt import encrypt_excel
        enc = str(temp_dir / "enc.xlsx")
        rules_path = encrypt_excel(str(sample_excel_path), enc)
        out = str(temp_dir / "restored.xlsx")
        decrypt_excel(enc, rules_path, out)
        assert Path(out).exists()

    def test_formulas_preserved_after_decrypt(self, sample_excel_path, temp_dir):
        import openpyxl
        from src.eYcel.encrypt import encrypt_excel
        enc = str(temp_dir / "enc.xlsx")
        rules_path = encrypt_excel(str(sample_excel_path), enc)
        out = str(temp_dir / "restored.xlsx")
        decrypt_excel(enc, rules_path, out)
        wb = openpyxl.load_workbook(out, data_only=False)
        ws = wb.active
        for row in range(2, 7):
            val = ws.cell(row=row, column=5).value
            assert val is not None and str(val).startswith("="), \
                f"Formula missing at row {row}"

    def test_missing_encrypted_file_raises(self, temp_dir, sample_rules):
        from src.eYcel.yaml_handler import save_rules
        rules_path = str(temp_dir / "rules.yaml")
        save_rules(sample_rules, rules_path)
        with pytest.raises(FileNotFoundError):
            decrypt_excel(str(temp_dir / "ghost.xlsx"), rules_path, str(temp_dir / "out.xlsx"))
