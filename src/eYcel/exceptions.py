"""
Custom exceptions for eYcel.
"""


class EYcelError(Exception):
    """Base exception for all eYcel errors."""
    pass


class EncryptionError(EYcelError):
    """Raised when encryption fails."""
    pass


class DecryptionError(EYcelError):
    """Raised when decryption fails."""
    pass


class RulesValidationError(EYcelError):
    """Raised when rules file validation fails."""
    pass


class FormulaError(EYcelError):
    """Raised when formula handling fails."""
    pass


class MemoryLimitError(EYcelError):
    """Raised when memory limit is exceeded."""
    pass
