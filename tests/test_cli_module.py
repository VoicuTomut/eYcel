"""
Unit tests for the eYcel CLI module (src/eYcel/cli.py).

These tests import the module directly and mock external dependencies,
ensuring the CLI logic itself is covered by coverage.
"""
import sys
import io
import argparse
from unittest.mock import Mock, patch, MagicMock
from pathlib import Path

import pytest

# Import the CLI module functions
from src.eYcel.cli import (
    cmd_encrypt,
    cmd_decrypt,
    cmd_validate,
    build_parser,
    main,
)


# ---------------------------------------------------------------------------
# Fixtures & helpers
# ---------------------------------------------------------------------------

@pytest.fixture
def mock_encrypt():
    """Mock encrypt_excel to return a dummy rules path."""
    with patch("src.eYcel.cli.encrypt_excel") as mock:
        mock.return_value = "/tmp/test_rules.yaml"
        yield mock


@pytest.fixture
def mock_decrypt():
    """Mock decrypt_excel."""
    with patch("src.eYcel.cli.decrypt_excel") as mock:
        yield mock


@pytest.fixture
def mock_load_rules():
    """Mock load_rules."""
    with patch("src.eYcel.cli.load_rules") as mock:
        yield mock


@pytest.fixture
def mock_validate_rules():
    """Mock validate_rules."""
    with patch("src.eYcel.cli.validate_rules") as mock:
        yield mock


def make_args(**kwargs):
    """Create an argparse.Namespace with the given attributes."""
    return argparse.Namespace(**kwargs)


# ---------------------------------------------------------------------------
# cmd_encrypt tests
# ---------------------------------------------------------------------------

class TestCmdEncrypt:
    """Test cmd_encrypt directly."""

    def test_success(self, mock_encrypt, tmp_path):
        """Normal encryption success."""
        input_file = tmp_path / "input.xlsx"
        input_file.touch()
        output_file = tmp_path / "output.xlsx"
        args = make_args(
            input=str(input_file),
            output=str(output_file),
            quiet=False,
        )
        ret = cmd_encrypt(args)
        assert ret == 0
        mock_encrypt.assert_called_once_with(
            input_path=str(input_file),
            output_path=str(output_file),
            rules=None,
        )
        # Output printed to stdout (captured by capsys later)

    def test_input_not_found(self, capsys):
        """Missing input file."""
        args = make_args(input="/nonexistent.xlsx", output="/tmp/out.xlsx", quiet=False)
        ret = cmd_encrypt(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err

    def test_input_not_excel(self, tmp_path, capsys):
        """Input file with wrong extension."""
        input_file = tmp_path / "input.txt"
        input_file.write_text("not excel")
        args = make_args(input=str(input_file), output="/tmp/out.xlsx", quiet=False)
        ret = cmd_encrypt(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err

    def test_output_dir_missing(self, tmp_path, capsys):
        """Output directory does not exist."""
        input_file = tmp_path / "input.xlsx"
        input_file.touch()
        args = make_args(
            input=str(input_file),
            output="/nonexistent/dir/out.xlsx",
            quiet=False,
        )
        ret = cmd_encrypt(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err

    def test_encrypt_exception(self, mock_encrypt, tmp_path, capsys):
        """encrypt_excel raises an exception."""
        input_file = tmp_path / "input.xlsx"
        input_file.touch()
        output_file = tmp_path / "output.xlsx"
        mock_encrypt.side_effect = Exception("Mock encryption error")
        args = make_args(input=str(input_file), output=str(output_file), quiet=False)
        ret = cmd_encrypt(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err
        assert "Mock encryption error" in captured.err

    def test_quiet_mode(self, mock_encrypt, tmp_path, capsys):
        """No output when quiet=True."""
        input_file = tmp_path / "input.xlsx"
        input_file.touch()
        output_file = tmp_path / "output.xlsx"
        args = make_args(
            input=str(input_file),
            output=str(output_file),
            quiet=True,
        )
        ret = cmd_encrypt(args)
        assert ret == 0
        captured = capsys.readouterr()
        assert captured.out == ""
        assert captured.err == ""


# ---------------------------------------------------------------------------
# cmd_decrypt tests
# ---------------------------------------------------------------------------

class TestCmdDecrypt:
    """Test cmd_decrypt directly."""

    def test_success(self, mock_decrypt, tmp_path):
        """Normal decryption success."""
        enc_file = tmp_path / "encrypted.xlsx"
        enc_file.touch()
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        output_file = tmp_path / "decrypted.xlsx"
        args = make_args(
            input=str(enc_file),
            rules=str(rules_file),
            output=str(output_file),
            quiet=False,
        )
        ret = cmd_decrypt(args)
        assert ret == 0
        mock_decrypt.assert_called_once_with(
            encrypted_path=str(enc_file),
            rules_path=str(rules_file),
            output_path=str(output_file),
        )

    def test_missing_encrypted_file(self, tmp_path, capsys):
        """Encrypted file not found."""
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        args = make_args(
            input=str(tmp_path / "ghost.xlsx"),
            rules=str(rules_file),
            output=str(tmp_path / "out.xlsx"),
            quiet=False,
        )
        ret = cmd_decrypt(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err

    def test_missing_rules_file(self, tmp_path, capsys):
        """Rules file not found."""
        enc_file = tmp_path / "encrypted.xlsx"
        enc_file.touch()
        args = make_args(
            input=str(enc_file),
            rules=str(tmp_path / "ghost.yaml"),
            output=str(tmp_path / "out.xlsx"),
            quiet=False,
        )
        ret = cmd_decrypt(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err

    def test_output_dir_missing(self, tmp_path, capsys):
        """Output directory does not exist."""
        enc_file = tmp_path / "encrypted.xlsx"
        enc_file.touch()
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        args = make_args(
            input=str(enc_file),
            rules=str(rules_file),
            output="/nonexistent/dir/out.xlsx",
            quiet=False,
        )
        ret = cmd_decrypt(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err

    def test_decrypt_exception(self, mock_decrypt, tmp_path, capsys):
        """decrypt_excel raises an exception."""
        enc_file = tmp_path / "encrypted.xlsx"
        enc_file.touch()
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        output_file = tmp_path / "decrypted.xlsx"
        mock_decrypt.side_effect = Exception("Mock decryption error")
        args = make_args(
            input=str(enc_file),
            rules=str(rules_file),
            output=str(output_file),
            quiet=False,
        )
        ret = cmd_decrypt(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err
        assert "Mock decryption error" in captured.err

    def test_quiet_mode(self, mock_decrypt, tmp_path, capsys):
        """No output when quiet=True."""
        enc_file = tmp_path / "encrypted.xlsx"
        enc_file.touch()
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        output_file = tmp_path / "decrypted.xlsx"
        args = make_args(
            input=str(enc_file),
            rules=str(rules_file),
            output=str(output_file),
            quiet=True,
        )
        ret = cmd_decrypt(args)
        assert ret == 0
        captured = capsys.readouterr()
        assert captured.out == ""
        assert captured.err == ""


# ---------------------------------------------------------------------------
# cmd_validate tests
# ---------------------------------------------------------------------------

class TestCmdValidate:
    """Test cmd_validate directly."""

    def test_success_valid(self, mock_load_rules, mock_validate_rules, tmp_path, capsys):
        """Validate a valid rules file."""
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        mock_load_rules.return_value = {"metadata": {}, "columns": {}}
        mock_validate_rules.return_value = (True, [])
        args = make_args(rules=str(rules_file), quiet=False)
        ret = cmd_validate(args)
        assert ret == 0
        captured = capsys.readouterr()
        assert "valid" in captured.out.lower()
        mock_load_rules.assert_called_once_with(str(rules_file))
        mock_validate_rules.assert_called_once_with({"metadata": {}, "columns": {}})

    def test_success_invalid(self, mock_load_rules, mock_validate_rules, tmp_path, capsys):
        """Validate an invalid rules file."""
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        mock_load_rules.return_value = {"metadata": {}, "columns": {}}
        mock_validate_rules.return_value = (False, ["missing columns"])
        args = make_args(rules=str(rules_file), quiet=False)
        ret = cmd_validate(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "invalid" in captured.err.lower()
        assert "missing columns" in captured.err

    def test_missing_rules_file(self, tmp_path, capsys):
        """Rules file not found."""
        args = make_args(rules=str(tmp_path / "ghost.yaml"), quiet=False)
        ret = cmd_validate(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err

    def test_load_rules_exception(self, mock_load_rules, tmp_path, capsys):
        """load_rules raises an exception."""
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        mock_load_rules.side_effect = Exception("Mock load error")
        args = make_args(rules=str(rules_file), quiet=False)
        ret = cmd_validate(args)
        assert ret == 1
        captured = capsys.readouterr()
        assert "ERROR" in captured.err
        assert "Mock load error" in captured.err

    def test_quiet_mode_valid(self, mock_load_rules, mock_validate_rules, tmp_path, capsys):
        """No output when quiet=True and rules are valid."""
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        mock_load_rules.return_value = {"metadata": {}, "columns": {}}
        mock_validate_rules.return_value = (True, [])
        args = make_args(rules=str(rules_file), quiet=True)
        ret = cmd_validate(args)
        assert ret == 0
        captured = capsys.readouterr()
        assert captured.out == ""
        assert captured.err == ""

    def test_quiet_mode_invalid(self, mock_load_rules, mock_validate_rules, tmp_path, capsys):
        """Invalid rules still output to stderr even with quiet=True."""
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        mock_load_rules.return_value = {"metadata": {}, "columns": {}}
        mock_validate_rules.return_value = (False, ["error"])
        args = make_args(rules=str(rules_file), quiet=True)
        ret = cmd_validate(args)
        assert ret == 1
        captured = capsys.readouterr()
        # quiet only suppresses success messages, errors go to stderr
        assert "invalid" in captured.err.lower()


# ---------------------------------------------------------------------------
# build_parser tests
# ---------------------------------------------------------------------------
    def test_success_valid_with_columns(self, mock_load_rules, mock_validate_rules, tmp_path, capsys):
        """Validate a valid rules file with columns."""
        rules_file = tmp_path / 'rules.yaml'
        rules_file.write_text('')
        mock_load_rules.return_value = {
            'metadata': {'original_filename': 'test.xlsx'},
            'columns': {
                'A': {'transform': 'hash', 'salt': 'salt123'},
                'B': {'transform': 'scale', 'factor': 0.5},
            }
        }
        mock_validate_rules.return_value = (True, [])
        args = make_args(rules=str(rules_file), quiet=False)
        ret = cmd_validate(args)
        assert ret == 0
        captured = capsys.readouterr()
        assert 'valid' in captured.out.lower()
        assert 'A:' in captured.out
        assert 'B:' in captured.out
        mock_load_rules.assert_called_once_with(str(rules_file))
        mock_validate_rules.assert_called_once_with(mock_load_rules.return_value)


class TestBuildParser:
    """Test the argument parser builder."""

    def test_parser_creation(self):
        """Parser is created with correct prog and description."""
        parser = build_parser()
        assert parser.prog == "eYcel"
        assert "Excel Data Anonymization" in parser.description

    def test_subparsers_added(self):
        """All three subcommands are present."""
        parser = build_parser()
        subparsers_actions = [
            action for action in parser._actions
            if isinstance(action, argparse._SubParsersAction)
        ]
        assert len(subparsers_actions) == 1
        subparsers = subparsers_actions[0].choices
        assert set(subparsers.keys()) == {"encrypt", "decrypt", "validate"}

    def test_encrypt_subcommand(self):
        """Encrypt subcommand has required arguments."""
        parser = build_parser()
        # Use parse_args to ensure required flags work
        args = parser.parse_args(["encrypt", "-i", "in.xlsx", "-o", "out.xlsx"])
        assert args.command == "encrypt"
        assert args.input == "in.xlsx"
        assert args.output == "out.xlsx"

    def test_decrypt_subcommand(self):
        """Decrypt subcommand has required arguments."""
        parser = build_parser()
        args = parser.parse_args([
            "decrypt", "-i", "enc.xlsx", "-r", "rules.yaml", "-o", "dec.xlsx"
        ])
        assert args.command == "decrypt"
        assert args.input == "enc.xlsx"
        assert args.rules == "rules.yaml"
        assert args.output == "dec.xlsx"

    def test_validate_subcommand(self):
        """Validate subcommand has required arguments."""
        parser = build_parser()
        args = parser.parse_args(["validate", "-r", "rules.yaml"])
        assert args.command == "validate"
        assert args.rules == "rules.yaml"

    def test_no_command_shows_help(self, capsys):
        """Calling with no command prints help and exits with 0."""
        parser = build_parser()
        # Simulate parse_args being called with no command (already handled by main)
        # We'll test that parser.print_help is called when args.command is None
        # That's covered by main tests.
        pass


# ---------------------------------------------------------------------------
# main tests
# ---------------------------------------------------------------------------

class TestMain:
    """Test the main entry point."""

    def test_main_encrypt(self, mock_encrypt, tmp_path, capsys):
        """Main with encrypt command."""
        input_file = tmp_path / "input.xlsx"
        input_file.touch()
        output_file = tmp_path / "output.xlsx"
        with patch("sys.argv", ["eYcel", "encrypt", "-i", str(input_file), "-o", str(output_file)]):
            with pytest.raises(SystemExit) as exc_info:
                main()
        assert exc_info.value.code == 0
        mock_encrypt.assert_called_once()
        captured = capsys.readouterr()
        # Success messages printed
        assert "Encrypted" in captured.out or "ERROR" not in captured.err

    def test_main_decrypt(self, mock_decrypt, tmp_path, capsys):
        """Main with decrypt command."""
        enc_file = tmp_path / "encrypted.xlsx"
        enc_file.touch()
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        output_file = tmp_path / "decrypted.xlsx"
        with patch("sys.argv", ["eYcel", "decrypt",
                                "-i", str(enc_file),
                                "-r", str(rules_file),
                                "-o", str(output_file)]):
            with pytest.raises(SystemExit) as exc_info:
                main()
        assert exc_info.value.code == 0
        mock_decrypt.assert_called_once()

    def test_main_validate(self, mock_load_rules, mock_validate_rules, tmp_path, capsys):
        """Main with validate command."""
        rules_file = tmp_path / "rules.yaml"
        rules_file.write_text("")
        mock_load_rules.return_value = {"metadata": {}, "columns": {}}
        mock_validate_rules.return_value = (True, [])
        with patch("sys.argv", ["eYcel", "validate", "-r", str(rules_file)]):
            with pytest.raises(SystemExit) as exc_info:
                main()
        assert exc_info.value.code == 0
        mock_load_rules.assert_called_once()
        mock_validate_rules.assert_called_once()

    def test_main_no_command_prints_help(self, capsys):
        """No command triggers help."""
        with patch("sys.argv", ["eYcel"]):
            with pytest.raises(SystemExit) as exc_info:
                main()
        assert exc_info.value.code == 0
        captured = capsys.readouterr()
        assert "usage:" in captured.out.lower()

    def test_main_invalid_command_exits(self, capsys):
        """Invalid command leads to SystemExit (argparse default)."""
        # argparse will call sys.exit(2) with invalid command.
        # We'll catch SystemExit.
        with patch("sys.argv", ["eYcel", "invalid"]):
            with pytest.raises(SystemExit):
                main()