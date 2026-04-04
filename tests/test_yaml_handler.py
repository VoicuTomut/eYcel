"""
Tests for yaml_handler module.
"""
import pytest
from pathlib import Path

from src.eYcel.yaml_handler import (
    generate_rules,
    save_rules,
    load_rules,
    validate_rules,
    sanitize_rules,
)


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _minimal_rules():
    return {
        "metadata": {
            "original_filename": "test.xlsx",
            "timestamp": "2024-01-01T00:00:00+00:00",
        },
        "columns": {
            "amount": {"transform": "scale", "factor": 0.5},
            "category": {"transform": "shuffle", "mapping": {"A": "Cat_0"}},
        },
    }


# ---------------------------------------------------------------------------
# generate_rules
# ---------------------------------------------------------------------------

class TestGenerateRules:
    def test_has_metadata_and_columns(self):
        rules = generate_rules(
            metadata={"original_filename": "x.xlsx"},
            columns={"col1": {"transform": "keep"}},
        )
        assert "metadata" in rules
        assert "columns" in rules

    def test_timestamp_added_automatically(self):
        rules = generate_rules(
            metadata={"original_filename": "x.xlsx"},
            columns={},
        )
        assert "timestamp" in rules["metadata"]

    def test_caller_dict_not_mutated(self):
        meta = {"original_filename": "x.xlsx"}
        original_keys = set(meta.keys())
        generate_rules(meta, {})
        assert set(meta.keys()) == original_keys


# ---------------------------------------------------------------------------
# save_rules / load_rules round-trip
# ---------------------------------------------------------------------------

class TestSaveAndLoad:
    def test_round_trip(self, temp_dir):
        rules = _minimal_rules()
        path = str(temp_dir / "test_rules.yaml")
        save_rules(rules, path)
        loaded = load_rules(path)
        assert loaded["columns"]["amount"]["factor"] == 0.5

    def test_file_created(self, temp_dir):
        path = str(temp_dir / "out.yaml")
        save_rules(_minimal_rules(), path)
        assert Path(path).exists()

    def test_missing_file_raises(self, temp_dir):
        with pytest.raises(FileNotFoundError):
            load_rules(str(temp_dir / "nonexistent.yaml"))

    def test_invalid_yaml_raises_on_load(self, temp_dir):
        path = temp_dir / "bad.yaml"
        path.write_text("columns: [this, is, a, list, not, a, mapping]")
        with pytest.raises(ValueError):
            load_rules(str(path))


# ---------------------------------------------------------------------------
# validate_rules
# ---------------------------------------------------------------------------

class TestValidateRules:
    def test_valid_rules_pass(self):
        ok, errors = validate_rules(_minimal_rules())
        assert ok is True
        assert errors == []

    def test_missing_metadata_fails(self):
        bad = {"columns": {"c": {"transform": "keep"}}}
        ok, errors = validate_rules(bad)
        assert ok is False
        assert any("metadata" in e for e in errors)

    def test_missing_columns_fails(self):
        bad = {"metadata": {"original_filename": "x.xlsx"}}
        ok, errors = validate_rules(bad)
        assert ok is False
        assert any("columns" in e for e in errors)

    def test_invalid_transform_name_fails(self):
        rules = _minimal_rules()
        rules["columns"]["bad_col"] = {"transform": "INVALID"}
        ok, errors = validate_rules(rules)
        assert ok is False
        assert any("INVALID" in e for e in errors)

    def test_missing_transform_key_fails(self):
        rules = _minimal_rules()
        rules["columns"]["bad_col"] = {"factor": 0.5}   # no transform key
        ok, errors = validate_rules(rules)
        assert ok is False

    def test_non_dict_input_fails(self):
        ok, errors = validate_rules("not a dict")
        assert ok is False

    def test_all_valid_transforms_accepted(self):
        for t in ("hash", "offset", "scale", "shuffle", "keep", "anonymize"):
            rules = {
                "metadata": {"original_filename": "x.xlsx"},
                "columns": {"col": {"transform": t}},
            }
            ok, _ = validate_rules(rules)
            assert ok is True, f"Transform '{t}' should be valid"


# ---------------------------------------------------------------------------
# sanitize_rules
# ---------------------------------------------------------------------------

class TestSanitizeRules:
    def test_removes_original_values(self):
        rules = _minimal_rules()
        rules["columns"]["amount"]["original_values"] = [1, 2, 3]
        sanitized = sanitize_rules(rules)
        assert "original_values" not in sanitized["columns"]["amount"]

    def test_removes_raw_prefix_keys(self):
        rules = _minimal_rules()
        rules["columns"]["amount"]["raw_backup"] = "secret"
        sanitized = sanitize_rules(rules)
        assert "raw_backup" not in sanitized["columns"]["amount"]

    def test_safe_keys_preserved(self):
        rules = _minimal_rules()
        sanitized = sanitize_rules(rules)
        assert sanitized["columns"]["amount"]["factor"] == 0.5
