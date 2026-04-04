"""
Formula extraction, storage, and re-injection for openpyxl workbooks.

The key guarantee: formulas are stored as plain strings and re-injected
*after* data transformation, so the encrypted file retains 100 % of the
original formula logic.
"""
from typing import Dict, Tuple


# Type alias: maps (row, col) → formula string
FormulaMap = Dict[Tuple[int, int], str]


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------

def extract_formulas(worksheet) -> FormulaMap:
    """
    Walk every cell in a worksheet and collect formula cells.

    A cell is considered a formula cell when its value is a string
    that starts with '='.

    Args:
        worksheet: openpyxl Worksheet (opened with data_only=False).

    Returns:
        Dict mapping (row_index, col_index) tuples to formula strings,
        e.g. {(2, 3): '=SUM(A2:B2)'}.
    """
    formulas: FormulaMap = {}
    for row in worksheet.iter_rows():
        for cell in row:
            if isinstance(cell.value, str) and cell.value.startswith("="):
                formulas[(cell.row, cell.column)] = cell.value
    return formulas


def clear_formula_cells(worksheet, formula_cells: FormulaMap) -> None:
    """
    Temporarily blank out formula cells so the data-transform step
    won't try to encrypt formula strings.

    Args:
        worksheet:    openpyxl Worksheet (read-write).
        formula_cells: The FormulaMap returned by extract_formulas().
    """
    for (row, col) in formula_cells:
        worksheet.cell(row=row, column=col).value = None


def reinject_formulas(worksheet, formula_dict: FormulaMap) -> None:
    """
    Write formula strings back into their original cell positions.

    Call this *after* all data transformations are complete.

    Args:
        worksheet:    openpyxl Worksheet (read-write).
        formula_dict: The FormulaMap returned by extract_formulas().
    """
    for (row, col), formula in formula_dict.items():
        worksheet.cell(row=row, column=col).value = formula


def verify_formulas_preserved(original_ws, processed_ws) -> bool:
    """
    Confirm that every formula present in *original_ws* also appears
    unchanged in *processed_ws*.

    Args:
        original_ws:  openpyxl Worksheet from the source workbook.
        processed_ws: openpyxl Worksheet from the output workbook.

    Returns:
        True if all original formulas are found intact; False otherwise.
    """
    original_formulas = extract_formulas(original_ws)
    processed_formulas = extract_formulas(processed_ws)

    for coords, formula in original_formulas.items():
        if processed_formulas.get(coords) != formula:
            return False
    return True


def get_formula_summary(worksheet) -> Dict[str, int]:
    """
    Return a quick summary of formula usage in a worksheet.

    Args:
        worksheet: openpyxl Worksheet.

    Returns:
        Dict with keys 'formula_count' and 'data_count'.
    """
    formula_count = 0
    data_count = 0
    for row in worksheet.iter_rows():
        for cell in row:
            if cell.value is None:
                continue
            if isinstance(cell.value, str) and cell.value.startswith("="):
                formula_count += 1
            else:
                data_count += 1
    return {"formula_count": formula_count, "data_count": data_count}
