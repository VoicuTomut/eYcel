"""
eYcel - Excel Data Anonymization & Encryption Tool

Transform sensitive Excel data into anonymized versions while
preserving formulas.
"""

__version__ = "0.2.0"
__author__ = "Voicu Tomut"
__license__ = "MIT"

from .encrypt import encrypt_excel
from .decrypt import decrypt_excel

__all__ = ["encrypt_excel", "decrypt_excel", "__version__"]
