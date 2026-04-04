"""
Tests for the eYcel CLI (eYcel_cli.py).

Runs the CLI as a subprocess so the full argument-parsing and dispatch
paths are exercised, exactly as a real user would call them.
"""
import subprocess
import sys
from pathlib import Path

import openpyxl
import pytest
import yaml

# Path to the CLI script under test
CLI = str(Path(__file__).parent.parent / "eYcel_cli.py")


def run(args: list[str]) -> subprocess.CompletedProcess:
    """Helper: run the CLI with *args* and capture output."""
    return subprocess.run(
        [sys.executable, CLI] + args,
        capture_output=True,
        text=True,
    )


# ---------------------------------------------------------------------------
# --help / --version
# ---------------------------------------------------------------------------

class TestCLIHelp:
    def test_no_args_shows_help(self):
        r = run([])
        assert r.returncode == 0
        assert "eYcel" in r.stdout

    def test_help_flag(self):
        r = run(["--help"])
        assert r.returncode == 0
        assert "encrypt" in r.stdout
        assert "decrypt" in r.stdout
        assert "validate" in r.stdout

    def test_version_flag(self):
        r = run(["--version"])
        assert r.returncode == 0
        assert "eYcel" in r.stdout

    def test_encrypt_subcommand_help(self):
        r = run(["encrypt", "--help"])
        assert r.returncode == 0
        assert "--input" in r.stdout
        assert "--output" in r.stdout

    def test_decrypt_subcommand_help(self):
        r = run(["decrypt", "--help"])
        assert r.returncode == 0
        assert "--input" in r.stdout
        assert "--rules" in r.stdout
        assert "--output" in r.stdout

    def test_validate_subcommand_help(self):
        r = run(["validate", "--help"])
        assert r.returncode == 0
        assert "--rules" in r.stdout


# ---------------------------------------------------------------------------
# encrypt command
# ---------------------------------------------------------------------------

class TestEncryptCommand:
    def test_encrypt_produces_output_and_rules(self, sample_excel_path, tmp_path):
        out = tmp_path / "enc.xlsx"
        r = run(["encrypt", "-i", str(sample_excel_path), "-o", str(out)])
        assert r.returncode == 0, r.stderr
        assert out.exists()
        # Rules file should be beside the output
        rules = tmp_path / "enc_rules.yaml"
        assert rules.exists()

    def test_encrypt_stdout_contains_paths(self, sample_excel_path, tmp_path):
        out = tmp_path / "enc.xlsx"
        r = run(["encrypt", "-i", str(sample_excel_path), "-o", str(out)])
        assert r.returncode == 0, r.stderr
        assert "enc.xlsx" in r.stdout
        assert "rules.yaml" in r.stdout

    def test_encrypt_quiet_suppresses_output(self, sample_excel_path, tmp_path):
        out = tmp_path / "enc.xlsx"
        r = run(["-q", "encrypt", "-i", str(sample_excel_path), "-o", str(out)])
        assert r.returncode == 0, r.stderr
        assert r.stdout.strip() == ""

    def test_encrypt_missing_input_returns_error(self, tmp_path):
        out = tmp_path / "enc.xlsx"
        r = run(["encrypt", "-i", str(tmp_path / "nonexistent.xlsx"), "-o", str(out)])
        assert r.returncode == 1
        assert "ERROR" in r.stderr

    def test_encrypt_non_excel_input_returns_error(self, tmp_path):
        fake = tmp_path / "data.txt"
        fake.write_text("not excel")
        out = tmp_path / "enc.xlsx"
        r = run(["encrypt", "-i", str(fake), "-o", str(out)])
        assert r.returncode == 1
        assert "ERROR" in r.stderr

    def test_encrypt_bad_output_dir_returns_error(self, sample_excel_path):
        r = run(["encrypt", "-i", str(sample_excel_path), "-o", "/nonexistent/dir/enc.xlsx"])
        assert r.returncode == 1
        assert "ERROR" in r.stderr


# ---------------------------------------------------------------------------
# decrypt command
# ---------------------------------------------------------------------------

class TestDecryptCommand:
    def test_decrypt_restores_file(self, sample_excel_path, tmp_path):
        enc = tmp_path / "enc.xlsx"
        dec = tmp_path / "dec.xlsx"
        rules = tmp_path / "enc_rules.yaml"

        # First encrypt
        r = run(["encrypt", "-i", str(sample_excel_path), "-o", str(enc)])
        assert r.returncode == 0, r.stderr

        # Then decrypt
        r = run(["decrypt", "-i", str(enc), "-r", str(rules), "-o", str(dec)])
        assert r.returncode == 0, r.stderr
        assert dec.exists()

    def test_decrypt_stdout_contains_path(self, sample_excel_path, tmp_path):
        enc = tmp_path / "enc.xlsx"
        dec = tmp_path / "dec.xlsx"
        rules = tmp_path / "enc_rules.yaml"
        run(["encrypt", "-i", str(sample_excel_path), "-o", str(enc)])
        r = run(["decrypt", "-i", str(enc), "-r", str(rules), "-o", str(dec)])
        assert r.returncode == 0, r.stderr
        assert "dec.xlsx" in r.stdout

    def test_decrypt_missing_encrypted_file_returns_error(self, tmp_path):
        rules = tmp_path / "rules.yaml"
        rules.write_text("metadata: {}\ncolumns: {}\n")
        r = run(["decrypt", "-i", str(tmp_path / "ghost.xlsx"),
                 "-r", str(rules), "-o", str(tmp_path / "out.xlsx")])
        assert r.returncode == 1
        assert "ERROR" in r.stderr

    def test_decrypt_missing_rules_file_returns_error(self, sample_excel_path, tmp_path):
        r = run(["decrypt", "-i", str(sample_excel_path),
                 "-r", str(tmp_path / "ghost.yaml"),
                 "-o", str(tmp_path / "out.xlsx")])
        assert r.returncode == 1
        assert "ERROR" in r.stderr

    def test_decrypt_quiet_suppresses_output(self, sample_excel_path, tmp_path):
        enc = tmp_path / "enc.xlsx"
        dec = tmp_path / "dec.xlsx"
        rules = tmp_path / "enc_rules.yaml"
        run(["encrypt", "-i", str(sample_excel_path), "-o", str(enc)])
        r = run(["-q", "decrypt", "-i", str(enc), "-r", str(rules), "-o", str(dec)])
        assert r.returncode == 0, r.stderr
        assert r.stdout.strip() == ""


# ---------------------------------------------------------------------------
# validate command
# ---------------------------------------------------------------------------

class TestValidateCommand:
    def test_validate_valid_rules(self, sample_excel_path, tmp_path):
        enc = tmp_path / "enc.xlsx"
        rules = tmp_path / "enc_rules.yaml"
        run(["encrypt", "-i", str(sample_excel_path), "-o", str(enc)])
        r = run(["validate", "-r", str(rules)])
        assert r.returncode == 0, r.stderr
        assert "valid" in r.stdout.lower()

    def test_validate_missing_file_returns_error(self, tmp_path):
        r = run(["validate", "-r", str(tmp_path / "ghost.yaml")])
        assert r.returncode == 1
        assert "ERROR" in r.stderr

    def test_validate_invalid_yaml_returns_error(self, tmp_path):
        bad = tmp_path / "bad.yaml"
        bad.write_text("not: valid: yaml: [[\n")
        r = run(["validate", "-r", str(bad)])
        assert r.returncode == 1

    def test_validate_shows_column_list(self, sample_excel_path, tmp_path):
        enc = tmp_path / "enc.xlsx"
        rules = tmp_path / "enc_rules.yaml"
        run(["encrypt", "-i", str(sample_excel_path), "-o", str(enc)])
        r = run(["validate", "-r", str(rules)])
        assert r.returncode == 0
        # Should list at least one column transform
        assert ":" in r.stdout
