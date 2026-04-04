# eYcel 🔐

> **Excel Data Anonymization & Encryption Tool**
> Transform sensitive Excel data into anonymized versions — safe to share with AI models, third parties, or public environments — while preserving all formulas and structure.

[![Python 3.9+](https://img.shields.io/badge/python-3.9+-blue.svg)](https://www.python.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Platform: Win/Mac/Linux](https://img.shields.io/badge/platform-win%20%7C%20mac%20%7C%20linux-lightgrey.svg)]()

---

## 🧠 Why eYcel?

When working with AI tools like ChatGPT, you often need to share your data to get help with formulas, analysis, or logic — but your data may be **confidential** (financial records, customer data, HR data, etc.).

**eYcel solves this:**
1. You encrypt your Excel file → data becomes meaningless to outsiders
2. You send the encrypted file to GPT → GPT builds formulas/algorithms on the anonymized structure
3. You apply GPT's output to your **real** data using the rules file
4. Or you decrypt the result back to original values

All formulas are **preserved exactly** throughout this process.

---

## 🔄 Core Workflow

```
┌─────────────────┐     encrypt      ┌──────────────────────┐
│  original.xlsx  │ ───────────────► │  encrypted.xlsx      │  ← safe to share
│  (sensitive)    │                  │  + rules.yaml        │  ← keep this secret
└─────────────────┘                  └──────────────────────┘
                                              │
                                     share encrypted.xlsx
                                              │
                                              ▼
                                       GPT / Analysis
                                              │
                                     get formulas/results
                                              │
                                              ▼
┌─────────────────┐     decrypt      ┌──────────────────────┐
│  restored.xlsx  │ ◄─────────────── │  result.xlsx         │
│  (original)     │                  │  + rules.yaml        │
└─────────────────┘                  └──────────────────────┘
```

---

## ⚡ Quick Install

```bash
# Option 1 — pip (recommended)
pip install eYcel

# Option 2 — pipx (isolated, any machine)
pipx install eYcel

# Option 3 — from source
git clone https://github.com/VoicuTomut/eYcel.git
cd eYcel
pip install -e .
```

---

## 🚀 Usage

### Command Line
```bash
# Encrypt
eYcel encrypt --input sales_data.xlsx --output encrypted.xlsx

# Decrypt
eYcel decrypt --input encrypted.xlsx --rules rules.yaml --output restored.xlsx

# Validate rules file
eYcel validate --rules rules.yaml
```

### Python API
```python
from eYcel import encrypt_excel, decrypt_excel

# Encrypt
encrypt_excel("sales_data.xlsx", "encrypted.xlsx")
# → produces encrypted.xlsx + rules.yaml

# Decrypt
decrypt_excel("encrypted.xlsx", "rules.yaml", "restored.xlsx")
```

---

## 📦 What Gets Generated

### `encrypted.xlsx`
- All **formulas preserved** exactly as-is
- All **data values** transformed (anonymized)
- Same structure, same sheet names, same column layout

### `rules.yaml`
```yaml
metadata:
  original_filename: sales_data.xlsx
  timestamp: "2024-01-15T14:30:00Z"

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
    transform: keep        # formulas always kept
```

> ⚠️ **The rules file is your decryption key — keep it safe and never share it.**

---

## 🔧 Supported Transformations

| Type | Description | Best For |
|------|-------------|----------|
| `hash` | One-way SHA256 hash | IDs, names |
| `offset` | Shift dates/numbers by fixed amount | Dates, timestamps |
| `scale` | Multiply numbers by a secret factor | Amounts, prices |
| `shuffle` | Rename categories systematically | Dropdowns, labels |
| `keep` | No transformation | Non-sensitive columns |
| `anonymize` | Replace with realistic fake values | Mixed types |

---

## 🗂️ Project Structure

```
eYcel/
├── src/eYcel/
│   ├── __init__.py
│   ├── encrypt.py          # Encryption pipeline
│   ├── decrypt.py          # Decryption pipeline
│   ├── formula_handler.py  # Formula preservation logic
│   ├── column_analyzer.py  # Data type detection
│   └── yaml_handler.py     # Rules file read/write
├── tests/
│   ├── test_encrypt.py
│   ├── test_decrypt.py
│   ├── test_formulas.py
│   └── test_integration.py
├── examples/
│   └── sample_data.xlsx
├── eYcel_cli.py            # CLI entry point
├── setup.py
├── requirements.txt
├── .env.example
└── ProjectBuildingPlan.md
```

---

## 🛠️ Requirements

- Python 3.9+
- openpyxl ≥ 3.1.0
- PyYAML ≥ 6.0

No heavy dependencies. No pandas. Runs on any machine.

---

## 🔐 Security Notes

- The **rules file contains zero original data** — only transformation metadata
- Hashing uses a random salt per session (deterministic within a session)
- Original file is **never modified** — all output goes to new files
- Works fully offline — no data ever leaves your machine

---

## 📋 License

MIT — free to use, modify, and distribute.

---

*eYcel: Your data keeps its structure. Its secrets stay yours.*
