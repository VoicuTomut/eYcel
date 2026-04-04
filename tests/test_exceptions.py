"""
Tests for the custom exceptions in src/eYcel/exceptions.py.
"""
import pytest

from src.eYcel.exceptions import (
    EYcelError,
    EncryptionError,
    DecryptionError,
    RulesValidationError,
    FormulaError,
    MemoryLimitError,
)


def test_eycel_error_is_base():
    """EYcelError is the base for all custom exceptions."""
    assert issubclass(EncryptionError, EYcelError)
    assert issubclass(DecryptionError, EYcelError)
    assert issubclass(RulesValidationError, EYcelError)
    assert issubclass(FormulaError, EYcelError)
    assert issubclass(MemoryLimitError, EYcelError)


def test_exception_instantiation():
    """Each exception can be instantiated with a message."""
    exc = EYcelError("test")
    assert str(exc) == "test"

    exc = EncryptionError("encryption failed")
    assert str(exc) == "encryption failed"

    exc = DecryptionError("decryption failed")
    assert str(exc) == "decryption failed"

    exc = RulesValidationError("rules invalid")
    assert str(exc) == "rules invalid"

    exc = FormulaError("formula error")
    assert str(exc) == "formula error"

    exc = MemoryLimitError("memory limit")
    assert str(exc) == "memory limit"


def test_exception_chaining():
    """Exceptions can be raised and caught."""
    with pytest.raises(EncryptionError) as exc_info:
        raise EncryptionError("specific error")
    assert "specific error" in str(exc_info.value)

    with pytest.raises(DecryptionError):
        raise DecryptionError("decrypt")

    with pytest.raises(RulesValidationError):
        raise RulesValidationError("validate")

    with pytest.raises(FormulaError):
        raise FormulaError("formula")

    with pytest.raises(MemoryLimitError):
        raise MemoryLimitError("memory")


def test_exception_inheritance():
    """Catching base class catches derived exceptions."""
    try:
        raise EncryptionError("test")
    except EYcelError:
        pass  # expected
    else:
        pytest.fail("Base exception did not catch derived")

    try:
        raise MemoryLimitError("test")
    except EYcelError:
        pass  # expected
    else:
        pytest.fail("Base exception did not catch derived")