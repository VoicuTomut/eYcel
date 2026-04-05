# eYcel 🔐

**Encrypt your Excel data before sharing with AI.**

[![Python 3.9+](https://img.shields.io/badge/python-3.9+-blue.svg)](https://www.python.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tests](https://github.com/VoicuTomut/eYcel/actions/workflows/ci.yml/badge.svg)](https://github.com/VoicuTomut/eYcel/actions)

---

## The Problem

You want ChatGPT to help with your Excel formulas, analysis, or data logic — but your spreadsheet contains **confidential data** (financial records, customer info, HR data).

## The Solution

eYcel encrypts your data while keeping the structure and formulas intact. Send the encrypted file to GPT, get the formulas back, apply them to your real data.

**Your formulas are preserved exactly. Your data never leaves your machine.**

---

## How It Works

```
┌─────────────────┐     eYcel encrypt     ┌──────────────────────┐
│  sales_data.xlsx │ ──────────────────► │  sales_encrypted.xlsx │  safe to share
│  (confidential)  │                      │  + sales_rules.yaml   │  keep secret
└─────────────────┘                      └──────────────────────┘
                                                   │
                                          share encrypted file
                                                   │
                                                   ▼
                                             ChatGPT / AI
                                          "add a profit formula"
                                                   │
                                          get formula back
                                                   │
┌─────────────────┐     eYcel decrypt     ┌──────────────────────┐
│  restored.xlsx   │ ◄────────────────── │  result + rules.yaml  │
│  (original data) │                      └──────────────────────┘
└─────────────────┘
```

---

## Quick Start

### Install

```bash
git clone https://github.com/VoicuTomut/eYcel.git
cd eYcel
pip install -e .
```

### Encrypt

```bash
eYcel encrypt --input sales_data.xlsx
```

Produces:
- `sales_data_encrypted.xlsx` — anonymized data, formulas intact
- `sales_data_rules.yaml` — decryption key (keep this secret!)

### Decrypt

```bash
eYcel decrypt --input sales_data_encrypted.xlsx --rules sales_data_rules.yaml
```

Restores your original data with any new formulas applied.

### Validate

```bash
eYcel validate --rules sales_data_rules.yaml
```

Checks that a rules file is valid and contains no original data.

---

## Python API

```python
from eYcel import encrypt_excel, decrypt_excel

# Encrypt
rules_path = encrypt_excel("sales_data.xlsx", "encrypted.xlsx")

# Decrypt
decrypt_excel("encrypted.xlsx", "sales_data_rules.yaml", "restored.xlsx")
```

---

## Transformations

Each column gets a transformation rule:

| Type | What it does | Best for | Reversible |
|------|-------------|----------|------------|
| `hash` | SHA-256 with salt | IDs, names, emails | No |
| `offset` | Shift by fixed amount | Dates, numbers | Yes |
| `scale` | Multiply by secret factor | Amounts, prices | Yes |
| `shuffle` | Rename categories | Dropdowns, labels | Yes |
| `keep` | No change | Non-sensitive columns | N/A |
| `anonymize` | Replace with fake values | Mixed data | No |

---

## Formula Preservation

This is the core feature. eYcel:

1. Extracts all formula cells before transformation
2. Transforms only data cells
3. Re-injects formulas in their exact original positions
4. Verifies formulas are identical after the round-trip

`=SUM(B2:B100)`, `=VLOOKUP(...)`, nested `=IF(...)` — all survive untouched.

---

## Rules File

The rules file is your decryption key. It contains only transformation metadata — **never original data values**.

```yaml
metadata:
  original_filename: sales_data.xlsx
  timestamp: "2026-04-04T10:00:00Z"
columns:
  customer_id:
    transform: hash
    salt: "a1b2c3..."
  transaction_date:
    transform: offset
    offset_days: -45
  amount:
    transform: scale
    factor: 0.73
  category:
    transform: shuffle
    mapping:
      Electronics: Cat_A
      Clothing: Cat_B
  profit_formula:
    transform: keep
```

Keep this file secret. Anyone with the rules file can decrypt your data.

---

## Project Structure

```
eYcel/
├── src/eYcel/
│   ├── cli.py                # CLI interface
│   ├── encrypt.py            # Encryption pipeline
│   ├── decrypt.py            # Decryption pipeline
│   ├── formula_handler.py    # Formula preservation
│   ├── column_analyzer.py    # Data type detection
│   ├── transformations.py    # All transform functions
│   ├── yaml_handler.py       # Rules file handling
│   └── memory_utils.py       # Memory-efficient processing
├── tests/                    # 166+ tests
├── gui/                      # Streamlit web interface
├── examples/                 # Usage examples
└── pyproject.toml
```

---

## Tests

```bash
pip install -e ".[dev]"
pytest tests/ -v
```

166 tests: encryption, decryption, formula handling, transformations, CLI, round-trip integration.

---

## Requirements

- Python 3.9+
- openpyxl (Excel I/O with formula support)
- PyYAML (rules files)

No pandas. No heavy dependencies.

---

## Use Cases

- **AI data prep** — encrypt before sending to ChatGPT/Claude
- **Data sharing** — share with third parties safely
- **Testing** — realistic but anonymous test data
- **Compliance** — GDPR/HIPAA data anonymization

---

## License

MIT

---

*eYcel: Your data keeps its structure. Its secrets stay yours.*
