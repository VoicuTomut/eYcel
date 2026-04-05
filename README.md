# eYcel

**Encrypt your Excel & CSV data before sharing with AI.**

[![Python 3.9+](https://img.shields.io/badge/python-3.9+-blue.svg)](https://www.python.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tests](https://github.com/VoicuTomut/eYcel/actions/workflows/ci.yml/badge.svg)](https://github.com/VoicuTomut/eYcel/actions)

---

## The Problem

You want ChatGPT to help with your Excel formulas, analysis, or data logic — but your spreadsheet contains **confidential data** (financial records, customer info, HR data).

## The Solution

eYcel replaces all text with consistent fake words while keeping numbers and formulas intact. Send the encrypted file to the AI, get the formulas back, apply them to your real data.

**Numbers stay real (AI needs them for formulas). Text gets replaced consistently. Your data never leaves your machine.**

---

## Try It Online

**No installation needed.** The web app runs entirely in your browser via WebAssembly — your data never touches a server.

[**Open eYcel Web App**](https://voicutomut.github.io/eYcel/)

Supports `.xlsx`, `.xls`, and `.csv` files.

---

## How It Works

```
Original file                    Encrypted file
┌────────────┬────────┐          ┌────────────┬────────┐
│ Name       │ Amount │          │ Bako       │ Amount │
├────────────┼────────┤   ───►   ├────────────┼────────┤
│ Alice      │    100 │          │ Delu       │    100 │  ← numbers unchanged
│ Bob        │    250 │          │ Fimo       │    250 │
│ Alice      │    150 │          │ Delu       │    150 │  ← same word = same fake
│ =SUM(B2:B4)         │          │ =SUM(B2:B4)         │  ← formulas preserved
└────────────┴────────┘          └────────────┴────────┘

                    + rules.yaml (decryption key — keep secret!)
```

1. Text is replaced with readable fake words (Alice → Delu, everywhere)
2. Numbers stay unchanged so AI can create correct formulas
3. Formulas are preserved exactly (including text references inside them)
4. Send encrypted file to AI → get formulas → decrypt → formulas work on real data

---

## Quick Start

### Option 1: Web App (no install)

Open the [web app](https://voicutomut.github.io/eYcel/), drop your file, download the encrypted version.

### Option 2: Python CLI

```bash
git clone https://github.com/VoicuTomut/eYcel.git
cd eYcel
pip install -e .
```

**Encrypt:**
```bash
eYcel encrypt --input sales_data.xlsx
```

Produces:
- `sales_data_encrypted.xlsx` — text substituted, numbers and formulas intact
- `sales_data_rules.yaml` — decryption key (keep this secret!)

**Decrypt:**
```bash
eYcel decrypt --input sales_data_encrypted.xlsx --rules sales_data_rules.yaml
```

**Validate:**
```bash
eYcel validate --rules sales_data_rules.yaml
```

### Option 3: Python API

```python
from eYcel import encrypt_excel, decrypt_excel

# Encrypt (numbers kept by default)
rules_path = encrypt_excel("sales_data.xlsx", "encrypted.xlsx")

# Encrypt with number scrambling
rules_path = encrypt_excel("sales_data.xlsx", "encrypted.xlsx", scramble_numbers=True)

# Decrypt
decrypt_excel("encrypted.xlsx", "sales_data_rules.yaml", "restored.xlsx")
```

---

## What Gets Encrypted

| Data type | Default behavior | Why |
|-----------|-----------------|-----|
| Text / names | Consistent substitution (Alice → Delu everywhere) | Hides identity, AI sees consistent structure |
| Column headers | Also substituted | Hides what the data means |
| Numbers | **Kept unchanged** | AI needs real numbers for formulas |
| Dates | **Kept unchanged** | AI may need date logic |
| Formulas | Preserved exactly | Core guarantee |
| Text in formulas | Also substituted (`=COUNTIF(A:A,"Sales")` → `=COUNTIF(A:A,"Bako")`) | Formulas stay consistent with data |

**Options:**
- `--scramble-numbers` — also scale numbers by a random factor (reversible)
- `--scramble-dates` — also shift dates by random days (reversible)

---

## Supported Formats

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| `.xlsx` | Yes | Yes | Full formula support |
| `.xls` | Yes | Output as .xlsx | Old Excel format |
| `.csv` | Yes | Yes | Auto-detects types |

---

## Formula Preservation

This is the core feature:

1. All formulas are extracted before any transformation
2. Only data cells are transformed
3. Text literals inside formulas are updated to match substitutions
4. Formulas are reinjected in their exact original positions

`=SUM(B2:B100)`, `=VLOOKUP(...)`, `=COUNTIF(A:A,"Sales")` — all survive and stay correct.

---

## Project Structure

```
eYcel/
├── src/eYcel/                    # Python library
│   ├── cli.py                    # CLI interface
│   ├── encrypt.py                # Encryption pipeline
│   ├── decrypt.py                # Decryption pipeline
│   ├── formula_handler.py        # Formula preservation
│   ├── column_analyzer.py        # Data type detection
│   ├── transformations.py        # All transform functions
│   ├── yaml_handler.py           # Rules file handling
│   └── memory_utils.py           # Memory-efficient processing
├── eYcel-wasm/                   # Rust + WebAssembly version
│   ├── src/                      # Rust source (same logic as Python)
│   ├── web/                      # Web frontend (HTML/JS/CSS)
│   ├── tests/                    # 49 Rust tests
│   └── Cargo.toml
├── tests/                        # 198 Python tests
├── gui/                          # Streamlit GUI
└── pyproject.toml
```

---

## Tests

**Python:**
```bash
pip install -e ".[dev]"
pytest tests/ -v
```
198 tests, 89% coverage.

**Rust/WASM:**
```bash
cd eYcel-wasm
cargo test
```
49 tests covering all formats, transforms, and formula preservation.

---

## Deploy the Web App

The web app is a static site (HTML + JS + WASM). No server needed.

**GitHub Pages (automatic):**

Push to `main` — GitHub Actions builds the WASM and deploys to GitHub Pages.

**Manual / self-hosted:**
```bash
cd eYcel-wasm
./build.sh
# Serve eYcel-wasm/web/ with any static file server
```

---

## Requirements

**Python CLI:** Python 3.9+, openpyxl, PyYAML

**Web app:** Any modern browser (Chrome, Firefox, Safari, Edge)

No pandas. No heavy dependencies. No server.

---

## License

MIT

---

*eYcel: Your data keeps its structure. Its secrets stay yours.*
