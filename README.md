# eYcel 🔐📊

![Python](https://img.shields.io/badge/Python-3.9+-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)
![Status](https://img.shields.io/badge/Status-In%20Development-orange.svg)

> **eYcel** — Excel + Anonymize + Encrypt.  
> Transform sensitive Excel data into anonymized versions that preserve formulas, structure, and intelligence — while making the actual values unintelligible.

---

## 🎯 What is eYcel?

eYcel is a lightweight Python tool that lets you safely share or process sensitive Excel files with external systems (like LLMs/GPT) by anonymizing the data while keeping all formulas intact.

It produces two outputs:
- `encrypted.xlsx` — anonymized data with all formulas preserved
- `rules.yaml` — the decryption key (transformation rules), containing **zero original data**

---

## 🔄 Core Workflow

```
┌─────────────────┐     eYcel encrypt     ┌──────────────────────┐
│  Clean Excel    │ ──────────────────►   │  encrypted.xlsx      │
│  (sensitive)    │                       │  rules.yaml          │
└─────────────────┘                       └──────────┬───────────┘
                                                     │
                                          Send encrypted.xlsx only
                                                     │
                                                     ▼
                                          ┌──────────────────────┐
                                          │   LLM / GPT          │
                                          │  (generates formula  │
                                          │   or algorithm)      │
                                          └──────────┬───────────┘
                                                     │
                                          Apply result to clean data
                                                     │
                                                     ▼
┌─────────────────┐     eYcel decrypt     ┌──────────────────────┐
│  Restored Excel │ ◄──────────────────   │  encrypted.xlsx      │
│  (original data)│                       │  rules.yaml          │
└─────────────────┘                       └──────────────────────┘
```

---

## ✨ Features

### 🔒 Encryption Module
- Auto-detects column data types: `date`, `float`, `int`, `%`, `string`, `formula`
- Per-column transformation rules:
  - **Keep** — non-sensitive columns pass through unchanged
  - **Hash** — one-way deterministic transformation for IDs
  - **Offset** — shift dates or numbers by a random delta
  - **Anonymize** — replace values with type-consistent random data
  - **Shuffle** — reorder categorical values
  - **Rename** — systematically rename categories
- Preserves ALL Excel formulas exactly
- Generates `rules.yaml` — the decryption key

### 🔓 Decryption Module
- Ingests `encrypted.xlsx` + `rules.yaml`
- Reverses all transformations
- Restores original data perfectly
- Verifies formula integrity after restoration

### ⚡ Performance
- Memory usage: **< 50MB for 100k rows**
- Chunk processing for large files
- No pandas dependency in core engine
- Streaming / generator-based architecture

---

## 🚀 Installation

### Option 1 — pipx (recommended, isolated)
```bash
pipx install eYcel
```

### Option 2 — pip
```bash
pip install eYcel
```

### Option 3 — from source
```bash
git clone https://github.com/VoicuTomut/eYcel.git
cd eYcel
pip install -e .
```

---

## 📖 Quick Start

### Encrypt a file
```bash
eYcel encrypt input.xlsx
# Output: input_encrypted.xlsx + input_rules.yaml
```

### Decrypt a file
```bash
eYcel decrypt input_encrypted.xlsx input_rules.yaml
# Output: input_restored.xlsx
```

### Validate a rules file
```bash
eYcel validate input_rules.yaml
```

### Interactive mode (choose rules per column)
```bash
eYcel encrypt input.xlsx --interactive
```

### Batch mode (supply rules via config)
```bash
eYcel encrypt input.xlsx --rules my_config.yaml --quiet
```

---

## 🐍 Python API

```python
from eYcel import encrypt_excel, decrypt_excel

# Encrypt
encrypt_excel(
    input_path="sales_data.xlsx",
    output_path="sales_encrypted.xlsx",
    rules_path="sales_rules.yaml"
)

# Decrypt
decrypt_excel(
    input_path="sales_encrypted.xlsx",
    rules_path="sales_rules.yaml",
    output_path="sales_restored.xlsx"
)
```

---

## 📁 Project Structure

```
eYcel/
├── src/
│   └── eYcel/
│       ├── __init__.py
│       ├── cli.py               # CLI entry point (click)
│       ├── encrypt.py           # Encryption module
│       ├── decrypt.py           # Decryption module
│       ├── column_analyzer.py   # Data type detection
│       ├── transformations.py   # Transform logic
│       ├── formula_handler.py   # Formula preservation
│       └── yaml_handler.py      # Rules file I/O
├── tests/
│   ├── test_encrypt.py
│   ├── test_decrypt.py
│   ├── test_formulas.py
│   ├── test_transformations.py
│   └── test_integration.py
├── examples/
│   └── sample_data.xlsx
├── .env.example
├── .gitignore
├── pyproject.toml
├── requirements.txt
├── ProjectBuildingPlan.md
└── README.md
```

---

## 🔐 rules.yaml — What it looks like

```yaml
metadata:
  original_filename: sales_data.xlsx
  encryption_timestamp: "2024-01-15T14:30:00Z"
  eycel_version: "1.0.0"

columns:
  customer_id:
    transformation: hash
    algorithm: sha256
    salt: "a1b2c3d4"

  transaction_date:
    transformation: offset
    offset_days: -45

  amount:
    transformation: anonymize
    method: random_range
    min: 100
    max: 10000

  category:
    transformation: shuffle
    mapping:
      Electronics: Cat_A
      Clothing: Cat_B

  profit_formula:
    transformation: keep
    is_formula: true
```

> ⚠️ The rules file contains **zero original data values** — only transformation metadata.

---

## 🎯 Use Cases

| Use Case | Description |
|----------|-------------|
| 🤖 **LLM / GPT Data Prep** | Send anonymized data to AI models safely |
| 🔏 **GDPR / HIPAA Compliance** | Share data without exposing PII |
| 🧪 **Test Environments** | Use realistic but anonymous data in dev/test |
| 🤝 **Third-party Sharing** | Share business data with partners safely |
| 📊 **Formula Preservation** | AI generates formulas on anonymized data — apply to real data |

---

## 🛠️ Requirements

- Python 3.9+
- openpyxl >= 3.1.0
- PyYAML >= 6.0
- click >= 8.0

---

## 🤝 Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Run tests: `pytest tests/`
4. Submit a pull request

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

*eYcel: Where Excel data keeps its structure but loses its secrets.* 🔐
