#!/usr/bin/env python3
"""
Generate sample Excel files for eYcel demos and tests.

Usage:
    python examples/generate_sample_data.py          # creates sample_data.xlsx
    python examples/generate_sample_data.py --large  # creates large_data.xlsx (1000 rows)
"""
from __future__ import annotations

import argparse
import random
import sys
from datetime import date, timedelta
from pathlib import Path

import openpyxl

CATEGORIES = ["Electronics", "Clothing", "Food", "Furniture", "Sports"]
FIRST_NAMES = ["Alice", "Bob", "Carol", "David", "Eve", "Frank", "Grace", "Henry"]
LAST_NAMES = ["Smith", "Jones", "Williams", "Brown", "Taylor", "Wilson"]


def _random_date(start: date, end: date) -> date:
    delta = (end - start).days
    return start + timedelta(days=random.randint(0, delta))


def create_sample_excel(path: Path, n_rows: int = 20) -> None:
    """Create a sample Excel workbook with multiple data types and formulas."""
    wb = openpyxl.Workbook()
    ws = wb.active
    ws.title = "Sales"

    # --- Headers ---
    headers = [
        "customer_id", "first_name", "last_name", "email",
        "amount", "discount", "category", "joined_date", "total",
    ]
    ws.append(headers)

    start = date(2018, 1, 1)
    end = date(2024, 12, 31)

    random.seed(42)  # reproducible
    for i in range(1, n_rows + 1):
        fn = random.choice(FIRST_NAMES)
        ln = random.choice(LAST_NAMES)
        amount = round(random.uniform(10.0, 999.99), 2)
        discount = round(random.uniform(0.0, 0.3), 2)
        row_num = i + 1  # 1-indexed, row 1 = header

        ws.append([
            f"CUST{i:05d}",
            fn,
            ln,
            f"{fn.lower()}.{ln.lower()}@example.com",
            amount,
            discount,
            random.choice(CATEGORIES),
            _random_date(start, end),
            f"=E{row_num}*(1-F{row_num})",   # formula: amount × (1 - discount)
        ])

    # --- Second sheet: summary with cross-sheet formulas ---
    ws2 = wb.create_sheet("Summary")
    ws2.append(["Metric", "Value"])
    ws2.append(["Total Revenue", f"=SUM(Sales!E2:E{n_rows + 1})"])
    ws2.append(["Average Amount", f"=AVERAGE(Sales!E2:E{n_rows + 1})"])
    ws2.append(["Row Count", n_rows])

    wb.save(str(path))
    print(f"✅ Created {path}  ({n_rows} data rows)")


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate sample Excel files for eYcel")
    parser.add_argument("--large", action="store_true", help="Generate 1000-row file")
    parser.add_argument("--rows", type=int, default=20, help="Number of rows (default: 20)")
    parser.add_argument("--out", type=Path, help="Output path override")
    args = parser.parse_args()

    base = Path(__file__).parent

    if args.large:
        n = 1000
        out = args.out or base / "large_data.xlsx"
    else:
        n = args.rows
        out = args.out or base / "sample_data.xlsx"

    create_sample_excel(out, n_rows=n)


if __name__ == "__main__":
    main()
