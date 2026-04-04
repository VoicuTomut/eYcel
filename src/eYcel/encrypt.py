"""
Excel encryption pipeline.

Workflow
--------
1. Open the workbook (data_only=False so formulas are readable).
2. Analyse each column to decide the best transformation.
3. Extract and stash all formula cells.
4. Apply transformations to data cells column by column.
5. Re-inject formulas unchanged.
6. Save the encrypted workbook.
7. Generate and save rules.yaml.
"""

import random
import string
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple
from datetime import datetime, timezone

import openpyxl
from openpyxl.utils import column_index_from_string


from .column_analyzer import analyze_workbook_columns
from .formula_handler import extract_formulas, clear_formula_cells, reinject_formulas
from .transformations import (
    transform_hash,
    transform_offset_date,
    transform_offset_number,
    transform_scale,
    transform_shuffle,
    transform_keep,
    transform_anonymize,
)
from .yaml_handler import generate_rules, save_rules


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _random_salt(length: int = 16) -> str:
    """Generate a random alphanumeric salt string."""
    return "".join(random.choices(string.ascii_letters + string.digits, k=length))


def _random_factor() -> float:
    """Return a random scale factor in [0.1, 0.9) ∪ (1.1, 2.0]."""
    factor = random.uniform(0.1, 2.0)
    while 0.9 <= factor <= 1.1:
        factor = random.uniform(0.1, 2.0)
    return round(factor, 6)


def _random_offset() -> float:
    """Return a random numeric offset in [-1000, -1) ∪ (1, 1000]."""
    return round(random.uniform(100, 1000) * random.choice([-1, 1]), 4)


def _random_day_offset() -> int:
    """Return a random day offset between ±365 days."""
    return random.randint(-365, 365)


def _build_shuffle_mapping(unique_values: List[Any]) -> Dict[str, str]:
    """
    Create a deterministic label mapping: each unique value → 'Cat_N'.
    """
    return {str(v): f"Cat_{i}" for i, v in enumerate(sorted(set(str(x) for x in unique_values)))}


def auto_detect_transform(col_meta: Dict[str, Any]) -> str:
    """
    Recommend a transform type based on column metadata.

    Args:
        col_meta: Metadata dict from column_analyzer.analyze_column().

    Returns:
        Transform name string.
    """
    dtype = col_meta.get("dominant_type", "string")
    if dtype == "date":
        return "offset"
    if dtype in ("int", "float"):
        return "scale"
    if dtype == "categorical":
        return "shuffle"
    if dtype in ("string",):
        return "hash"
    return "keep"


def generate_output_paths(input_path: str) -> Tuple[str, str]:
    """
    Generate default output paths based on the input file name.

    Args:
        input_path: Path to the original Excel file.

    Returns:
        (encrypted_path, rules_path) as strings.
    """
    p = Path(input_path)
    stem = p.stem
    parent = p.parent
    encrypted = str(parent / f"{stem}_encrypted.xlsx")
    rules = str(parent / f"{stem}_rules.yaml")
    return encrypted, rules


# ---------------------------------------------------------------------------
# Column-level transform dispatcher
# ---------------------------------------------------------------------------

def _transform_cell(value: Any, config: Dict[str, Any]) -> Any:
    """Apply the correct forward transform to a single cell value."""
    t = config.get("transform", "keep")
    if t == "hash":
        return transform_hash(value, config["salt"])
    if t == "offset":
        if hasattr(value, "year"):          # date / datetime
            return transform_offset_date(value, config["offset_days"])
        return transform_offset_number(value, config["offset"])
    if t == "scale":
        return transform_scale(value, config["factor"])
    if t == "shuffle":
        return transform_shuffle(value, config["mapping"])
    if t == "anonymize":
        return transform_anonymize(value, config.get("col_type", "string"))
    return transform_keep(value)


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------

def encrypt_excel(
    input_path: str,
    output_path: Optional[str] = None,
    rules: Optional[Dict[str, Any]] = None,
) -> str:
    """
    Encrypt an Excel file while preserving all formulas.

    Args:
        input_path:  Path to the source .xlsx file.
        output_path: Destination path for the encrypted file.
                     Defaults to '<stem>_encrypted.xlsx' next to the source.
        rules:       Optional pre-built column config dict mapping column
                     header (str) → transform config (dict).
                     If None, transforms are auto-detected.

    Returns:
        Path to the generated rules YAML file.

    Raises:
        FileNotFoundError: If *input_path* does not exist.
        ValueError:        If the file is not a valid .xlsx file.
    """
    src = Path(input_path)
    if not src.exists():
        raise FileNotFoundError(f"Input file not found: {input_path}")

    default_encrypted, default_rules = generate_output_paths(input_path)
    if output_path is None:
        output_path = default_encrypted
    rules_path = str(
        Path(output_path).parent /
        (Path(output_path).stem.replace("_encrypted", "") + "_rules.yaml")
    )

    # ── 1. Open workbook ────────────────────────────────────────────────────
    wb = openpyxl.load_workbook(src, data_only=False)

    column_configs: Dict[str, Dict[str, Any]] = {}   # header → transform config

    for sheet_name in wb.sheetnames:
        ws = wb[sheet_name]

        max_row = ws.max_row or 1
        if max_row < 2:
            continue

        # ── 2. Analyse columns ───────────────────────────────────────────────
        col_metadata = analyze_workbook_columns(wb, sheet_name)

        # ── 3. Build per-column transform configs ────────────────────────────
        sheet_configs: Dict[int, Dict[str, Any]] = {}   # col_idx → config

        for col_letter, meta in col_metadata.items():
            col_idx = column_index_from_string(col_letter)
            header = str(meta.get("header") or col_letter)

            if rules and header in rules:
                cfg = dict(rules[header])
            else:
                transform = auto_detect_transform(meta)
                cfg = {"transform": transform}

                if transform == "hash":
                    cfg["salt"] = _random_salt()
                elif transform == "offset":
                    cfg["offset_days"] = _random_day_offset()
                    cfg["offset"] = _random_offset()
                elif transform == "scale":
                    cfg["factor"] = _random_factor()
                elif transform == "shuffle":

                    # Collect all unique values for full mapping
                    all_vals = set()
                    for row in range(2, max_row + 1):
                        cell = ws.cell(row=row, column=col_idx)
                        if cell.value is not None and not (
                            isinstance(cell.value, str) and cell.value.startswith("=")
                        ):
                            all_vals.add(cell.value)
                    cfg["mapping"] = _build_shuffle_mapping(list(all_vals))

            column_configs[header] = cfg
            sheet_configs[col_idx] = cfg

        # ── 4. Extract formulas ──────────────────────────────────────────────
        formula_map = extract_formulas(ws)
        clear_formula_cells(ws, formula_map)

        # ── 5. Transform data cells ──────────────────────────────────────────
        for row in range(2, max_row + 1):
            for col_idx, cfg in sheet_configs.items():
                cell = ws.cell(row=row, column=col_idx)
                if cell.value is None:
                    continue
                try:
                    cell.value = _transform_cell(cell.value, cfg)
                except Exception:
                    # If transform fails (e.g., wrong type), keep original
                    pass

        # ── 6. Re-inject formulas ────────────────────────────────────────────
        reinject_formulas(ws, formula_map)

    # ── 7. Save encrypted workbook ───────────────────────────────────────────
    Path(output_path).parent.mkdir(parents=True, exist_ok=True)
    wb.save(output_path)

    # ── 8. Generate and save rules ───────────────────────────────────────────
    metadata = {
        "original_filename": src.name,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "sheets": wb.sheetnames,
    }
    rules_dict = generate_rules(metadata, column_configs)
    save_rules(rules_dict, rules_path)

    return rules_path
