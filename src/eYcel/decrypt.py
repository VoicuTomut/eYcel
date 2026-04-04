"""
Excel decryption pipeline.

Workflow
--------
1. Load and validate rules.yaml.
2. Open the encrypted workbook (data_only=False).
3. For each sheet, extract any formula cells (preserve them).
4. Reverse-transform data cells column by column.
5. Re-inject formulas.
6. Save the restored workbook.
"""
from pathlib import Path
from typing import Any, Dict, List, Optional

import openpyxl
from openpyxl.utils import column_index_from_string, get_column_letter

from .formula_handler import extract_formulas, clear_formula_cells, reinject_formulas
from .transformations import (
    reverse_offset_date,
    reverse_offset_number,
    reverse_scale,
    reverse_shuffle,
    transform_keep,
)
from .yaml_handler import load_rules, validate_rules


# ---------------------------------------------------------------------------
# Reverse transform dispatcher
# ---------------------------------------------------------------------------

def _reverse_cell(value: Any, config: Dict[str, Any]) -> Any:
    """Apply the correct reverse transform to a single cell value."""
    t = config.get("transform", "keep")

    if t == "hash":
        # Hash is one-way — cannot reverse; leave the hashed value
        return value

    if t == "offset":
        if hasattr(value, "year"):          # date / datetime
            return reverse_offset_date(value, config.get("offset_days", 0))
        return reverse_offset_number(float(value), config.get("offset", 0.0))

    if t == "scale":
        return reverse_scale(float(value), config["factor"])

    if t == "shuffle":
        return reverse_shuffle(str(value), config.get("mapping", {}))

    if t in ("anonymize", "keep"):
        return transform_keep(value)

    return value


# ---------------------------------------------------------------------------
# Column header → column index lookup
# ---------------------------------------------------------------------------

def _build_header_map(worksheet) -> Dict[str, int]:
    """
    Read row 1 and return a dict mapping header string → column index.

    Args:
        worksheet: openpyxl Worksheet.

    Returns:
        {header_string: col_index, …}
    """
    mapping: Dict[str, int] = {}
    for cell in worksheet[1]:
        if cell.value is not None:
            mapping[str(cell.value)] = cell.column
    return mapping


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------

def load_and_validate_rules(rules_path: str) -> Dict[str, Any]:
    """
    Load a rules YAML file and raise on invalid content.

    Args:
        rules_path: Path to the rules .yaml file.

    Returns:
        Validated rules dictionary.

    Raises:
        FileNotFoundError: If the file does not exist.
        ValueError:        If the schema is invalid.
    """
    return load_rules(rules_path)   # load_rules already validates


def apply_reverse_transform(
    column_data: List[Any],
    transform_config: Dict[str, Any],
) -> List[Any]:
    """
    Apply a reverse transform to an entire column's data list.

    Args:
        column_data:      List of encrypted cell values.
        transform_config: Transform config dict from rules.

    Returns:
        List of restored values (same length, same order).
    """
    return [
        _reverse_cell(v, transform_config) if v is not None else v
        for v in column_data
    ]


def decrypt_excel(
    encrypted_path: str,
    rules_path: str,
    output_path: str,
) -> None:
    """
    Restore an encrypted Excel file to its original values.

    Args:
        encrypted_path: Path to the encrypted .xlsx file.
        rules_path:     Path to the matching rules .yaml file.
        output_path:    Destination path for the restored file.

    Raises:
        FileNotFoundError: If either input file does not exist.
        ValueError:        If rules validation fails.
    """
    # ── 1. Load & validate rules ─────────────────────────────────────────────
    rules = load_and_validate_rules(rules_path)
    column_configs: Dict[str, Dict[str, Any]] = rules.get("columns", {})

    # ── 2. Open encrypted workbook ───────────────────────────────────────────
    src = Path(encrypted_path)
    if not src.exists():
        raise FileNotFoundError(f"Encrypted file not found: {encrypted_path}")

    wb = openpyxl.load_workbook(src, data_only=False)

    for sheet_name in wb.sheetnames:
        ws = wb[sheet_name]
        max_row = ws.max_row or 1
        if max_row < 2:
            continue

        # Map column headers to column indices
        header_map = _build_header_map(ws)

        # ── 3. Extract formulas ──────────────────────────────────────────────
        formula_map = extract_formulas(ws)
        clear_formula_cells(ws, formula_map)

        # ── 4. Reverse-transform data cells ──────────────────────────────────
        for header, cfg in column_configs.items():
            col_idx = header_map.get(header)
            if col_idx is None:
                continue  # column not in this sheet — skip silently

            for row in range(2, max_row + 1):
                cell = ws.cell(row=row, column=col_idx)
                if cell.value is None:
                    continue
                try:
                    cell.value = _reverse_cell(cell.value, cfg)
                except Exception:
                    pass  # Leave value as-is on failure

        # ── 5. Re-inject formulas ────────────────────────────────────────────
        reinject_formulas(ws, formula_map)

    # ── 6. Save restored workbook ────────────────────────────────────────────
    Path(output_path).parent.mkdir(parents=True, exist_ok=True)
    wb.save(output_path)
