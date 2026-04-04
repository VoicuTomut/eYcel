# src/eYcel/memory_utils.py
"""Memory-efficient processing utilities for large Excel workbooks."""

from __future__ import annotations

import sys
from typing import Any, Callable, Generator, List

from openpyxl.worksheet.worksheet import Worksheet


# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

DEFAULT_CHUNK_SIZE: int = 1000
DEFAULT_MAX_MEMORY_MB: float = 50.0


# ---------------------------------------------------------------------------
# Public helpers
# ---------------------------------------------------------------------------

def get_memory_usage_mb() -> float:
    """Return the current process RSS memory usage in megabytes.

    Uses :mod:`sys` only (no psutil dependency).  Falls back to 0.0 on
    platforms that do not support ``sys.getsizeof`` size tracking.

    Returns:
        Memory usage in MB as a float.
    """
    try:
        import resource  # Unix only
        usage_kb = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        # macOS returns bytes, Linux returns kilobytes
        if sys.platform == "darwin":
            return usage_kb / (1024 * 1024)
        return usage_kb / 1024
    except ImportError:
        # Windows — use ctypes if available
        try:
            import ctypes
            import ctypes.wintypes

            class PROCESS_MEMORY_COUNTERS(ctypes.Structure):  # noqa: N801
                _fields_ = [
                    ("cb", ctypes.wintypes.DWORD),
                    ("PageFaultCount", ctypes.wintypes.DWORD),
                    ("PeakWorkingSetSize", ctypes.c_size_t),
                    ("WorkingSetSize", ctypes.c_size_t),
                    ("QuotaPeakPagedPoolUsage", ctypes.c_size_t),
                    ("QuotaPagedPoolUsage", ctypes.c_size_t),
                    ("QuotaPeakNonPagedPoolUsage", ctypes.c_size_t),
                    ("QuotaNonPagedPoolUsage", ctypes.c_size_t),
                    ("PagefileUsage", ctypes.c_size_t),
                    ("PeakPagefileUsage", ctypes.c_size_t),
                ]
            pmc = PROCESS_MEMORY_COUNTERS()
            pmc.cb = ctypes.sizeof(pmc)
            ctypes.windll.psapi.GetProcessMemoryInfo(  # type: ignore[attr-defined]
                ctypes.windll.kernel32.GetCurrentProcess(),  # type: ignore[attr-defined]
                ctypes.byref(pmc), pmc.cb,
            )
            return pmc.WorkingSetSize / (1024 * 1024)
        except Exception:
            return 0.0


def check_memory_limit(
    max_mb: float = DEFAULT_MAX_MEMORY_MB,
    label: str = "",
) -> None:
    """Warn if current memory usage exceeds *max_mb*.

    Does not raise — prints a warning to stderr so processing can continue.

    Args:
        max_mb:  Memory ceiling in megabytes.
        label:   Optional label to include in the warning message.
    """
    import sys as _sys
    current = get_memory_usage_mb()
    if current > max_mb:
        tag = f" [{label}]" if label else ""
        print(
            f"WARNING{tag}: memory usage {current:.1f} MB exceeds limit {max_mb:.1f} MB",
            file=_sys.stderr,
        )


def chunk_iterator(
    worksheet: Worksheet,
    chunk_size: int = DEFAULT_CHUNK_SIZE,
    start_row: int = 2,
) -> Generator[List[Any], None, None]:
    """Yield worksheet rows in chunks of *chunk_size*.

    Skips the header row (row 1) by default.  Each yielded chunk is a
    list of ``openpyxl`` row tuples.

    Args:
        worksheet:  The openpyxl worksheet to iterate.
        chunk_size: Number of rows per chunk.
        start_row:  First data row (default 2, skipping the header).

    Yields:
        List of row tuples, each row being a tuple of Cell objects.
    """
    max_row = worksheet.max_row or 1
    chunk: List[Any] = []

    for row in worksheet.iter_rows(min_row=start_row, max_row=max_row):
        chunk.append(row)
        if len(chunk) >= chunk_size:
            yield chunk
            chunk = []
            # Encourage GC to reclaim memory between chunks
            import gc
            gc.collect()

    if chunk:
        yield chunk


def process_column_in_chunks(
    worksheet: Worksheet,
    column_index: int,
    processor_func: Callable[[Any], Any],
    chunk_size: int = DEFAULT_CHUNK_SIZE,
    start_row: int = 2,
) -> int:
    """Apply *processor_func* to every cell in *column_index*, in chunks.

    Modifies the worksheet in-place.

    Args:
        worksheet:      Target openpyxl worksheet.
        column_index:   1-based column index.
        processor_func: Callable ``(cell_value) -> new_value``.
                        Called only for non-None, non-formula values.
        chunk_size:     Rows per processing chunk.
        start_row:      First data row (default 2).

    Returns:
        Total number of cells processed.
    """
    processed = 0

    for chunk in chunk_iterator(worksheet, chunk_size=chunk_size, start_row=start_row):
        for row_tuple in chunk:
            # row_tuple is a tuple of Cell objects
            if column_index - 1 >= len(row_tuple):
                continue
            cell = row_tuple[column_index - 1]
            if cell.value is None:
                continue
            if isinstance(cell.value, str) and cell.value.startswith("="):
                continue  # never transform formula cells
            cell.value = processor_func(cell.value)
            processed += 1

        del chunk  # release memory after each chunk

    return processed
