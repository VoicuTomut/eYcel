"""
Pytest fixtures and configuration.
"""
import pytest
import tempfile
from pathlib import Path


@pytest.fixture
def temp_dir():
    """Provide a temporary directory."""
    with tempfile.TemporaryDirectory() as tmpdir:
        yield Path(tmpdir)


@pytest.fixture
def sample_excel_path(temp_dir):
    """Create a sample Excel file for testing."""
    # TODO: Create sample Excel file using openpyxl
    pass


@pytest.fixture
def sample_rules():
    """Provide sample rules dictionary."""
    return {
        "metadata": {
            "original_filename": "test.xlsx",
            "timestamp": "2024-01-15T10:00:00Z"
        },
        "columns": {
            "id": {"transform": "hash", "salt": "abc123"},
            "amount": {"transform": "scale", "factor": 0.5}
        }
    }
