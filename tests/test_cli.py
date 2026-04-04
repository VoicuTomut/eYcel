"""
Tests for CLI module.
"""
import pytest
import subprocess
import sys


def test_cli_help():
    """Test CLI help output."""
    result = subprocess.run(
        [sys.executable, 'eYcel_cli.py', '--help'],
        capture_output=True,
        text=True
    )
    assert result.returncode == 0
    assert 'eYcel' in result.stdout


def test_encrypt_command():
    """Test encrypt command."""
    # TODO: Implement test
    pass


def test_decrypt_command():
    """Test decrypt command."""
    # TODO: Implement test
    pass
