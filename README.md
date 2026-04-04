# 🔐 eYcel

[![Tests](https://github.com/VoicuTomut/eYcel/actions/workflows/ci.yml/badge.svg)](https://github.com/VoicuTomut/eYcel/actions)
[![Python 3.9+](https://img.shields.io/badge/python-3.9+-blue.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PyPI](https://img.shields.io/badge/pypi-eYcel-brightgreen.svg)](https://pypi.org/project/eYcel/)

> **Excel Data Anonymization & Encryption Tool**

Preserve formulas while anonymizing sensitive data in Excel spreadsheets. Perfect for sharing datasets while protecting privacy and maintaining analytical integrity.

---

## ✨ Features

- 🔒 **Formula-Preserving Encryption** — All Excel formulas stay intact, only data values are anonymized
- 🎯 **Smart Type Detection** — Automatically detects dates, numbers, categoricals, and text
- 🔄 **Reversible Transformations** — Scale, offset, shuffle, and hash with full decryption support
- 📋 **Rules-Based Decryption** — Separate rules file ensures original data never leaks
- 🚀 **CLI & Python API** — Use from command line or integrate into your Python workflows
- 💾 **Memory Efficient** — Process large files without loading everything into memory
- 🧪 **Fully Tested** — 119 tests, 80%+ coverage

---


## 📦 Installation

```bash
pip install eYcel
```

> **Note:** PyPI release (`pip install eYcel`) is coming soon. For now, install from source:

### Development Install

```bash
git clone https://github.com/VoicuTomut/eYcel.git
cd eYcel
pip install -e .

# With development dependencies
pip install -e ".[dev]"
```



```bash
pip install eYcel
```

### Development Install

```bash
git clone https://github.com/VoicuTomut/eYcel.git
cd eYcel
pip install -e ".[dev]"
```

> **Note:** PyPI release (`pip install eYcel`) is coming soon.

---

## 🚀 Quick Start

### CLI Usage

```bash
# Encrypt
eYcel encrypt -i sales_data.xlsx -o encrypted.xlsx
# → produces encrypted.xlsx + sales_data_rules.yaml

# Decrypt
eYcel decrypt -i encrypted.xlsx -r sales_data_rules.yaml -o restored.xlsx

# Validate rules file
eYcel validate -r sales_data_rules.yaml

# Verbose output
eYcel -v encrypt -i sales_data.xlsx -o encrypted.xlsx

# Supply a pre-existing rules file during encryption (batch mode)
eYcel encrypt -i sales_data.xlsx -o encrypted.xlsx -r existing_rules.yaml
```

### Python API

```python
from eYcel import encrypt_excel, decrypt_excel

# Encrypt — returns the path to the generated rules file
rules_path = encrypt_excel("sales_data.xlsx", "encrypted.xlsx")
# → produces encrypted.xlsx + sales_data_rules.yaml

# Decrypt
decrypt_excel("encrypted.xlsx", rules_path, "restored.xlsx")
```

### Streamlit GUI
```bash
pip install streamlit
streamlit run gui/app.py
```

---

## 📖 How It Works

### Encryption Process

1. **Extract Formulas** — All formulas are identified and temporarily removed
2. **Analyze Columns** — Each column is analyzed to detect data types
3. **Apply Transformations** — Based on type, appropriate anonymization is applied:
   - **Dates** → Offset by random days
   - **Numbers** → Scale by random factor or offset
   - **Categoricals** → Shuffle values within category
   - **Strings** → Deterministic hash
4. **Store Rules** — Transformation parameters saved to YAML file
5. **Re-inject Formulas** — Formulas are restored to their original cells

### Decryption Process

1. **Load Rules** — Read transformation parameters from YAML
2. **Reverse Transform** — Apply inverse operations to restore original values
3. **Preserve Structure** — Sheet layout, headers, and formulas unchanged

### Rules File Security

The rules file contains **zero original data** — only transformation parameters:

```yaml
metadata:
  version: "0.2.0"
  created: "2024-01-15T10:30:00"
  source_file: "data.xlsx"

columns:
  A:
    header: "Date"
    transform: offset_date
    params:
      days: 42
  B:
    header: "Revenue"
    transform: scale
    params:
      factor: 1.337
```

---

## 🔧 CLI Commands

### `encrypt`

Encrypt an Excel file, creating an anonymized version and a rules file.

```bash
eyecel encrypt -i input.xlsx -o encrypted.xlsx [-r rules.yaml] [-q]
```

**Options:**
- `-i, --input` — Input Excel file (required)
- `-o, --output` — Output encrypted file (required)
- `-r, --rules` — Custom rules filename (optional, defaults to `<input>_rules.yaml`)
- `-q, --quiet` — Suppress output
- `-v, --verbose` — Verbose output

**Example:**
```bash
eyecel encrypt -i sales_data.xlsx -o sales_anonymized.xlsx -r sales_rules.yaml
```

### `decrypt`

Restore an encrypted file using its rules file.

```bash
eyecel decrypt -i encrypted.xlsx -r rules.yaml -o restored.xlsx [-q]
```

**Options:**
- `-i, --input` — Input encrypted file (required)
- `-r, --rules` — Rules file from encryption (required)
- `-o, --output` — Output restored file (required)
- `-q, --quiet` — Suppress output

**Example:**
```bash
eyecel decrypt -i sales_anonymized.xlsx -r sales_rules.yaml -o sales_restored.xlsx
```

### `validate`

Validate a rules file without decrypting.

```bash
eyecel validate -r rules.yaml
```

**Options:**
- `-r, --rules` — Rules file to validate (required)

**Example:**
```bash
eyecel validate -r sales_rules.yaml
# Output: Rules file valid ✓
```

---

## 🐍 Python API Examples

### Basic Encryption/Decryption

```python
from eYcel import encrypt_excel, decrypt_excel

# Simple encrypt → decrypt round-trip
encrypt_excel("original.xlsx", "encrypted.xlsx")
decrypt_excel("encrypted.xlsx", "original_rules.yaml", "restored.xlsx")
```

### Custom Transformations

```python
from eYcel import encrypt_excel
from eYcel.transformations import scale, offset_date

# Define custom rules for specific columns
custom_rules = {
    'A': {'transform': 'offset_date', 'params': {'days': 365}},
    'B': {'transform': 'scale', 'params': {'factor': 2.0}},
    'C': {'transform': 'hash'},  # One-way hashing
    'D': {'transform': 'keep'},  # Leave unchanged
}

encrypt_excel("data.xlsx", "encrypted.xlsx", column_rules=custom_rules)
```

### Processing with Progress Callback

```python
from eYcel import encrypt_excel

def on_progress(current, total, message):
    pct = (current / total) * 100
    print(f"{message}: {pct:.1f}%")

encrypt_excel(
    "large_file.xlsx",
    "encrypted.xlsx",
    progress_callback=on_progress
)
```

### Formula Verification

```python
from eYcel.formula_handler import verify_formulas_preserved

# Ensure formulas survived the round-trip
is_preserved = verify_formulas_preserved("original.xlsx", "restored.xlsx")
print(f"Formulas preserved: {is_preserved}")
```

---

## 🔐 Security Model

### What Gets Protected

| Data Type | Transformation | Reversible? |
|-----------|---------------|-------------|
| Dates | Random offset (± years) | ✅ Yes |
| Numbers | Scale or offset | ✅ Yes |
| Categoricals | Shuffle mapping | ✅ Yes |
| Strings | Deterministic hash | ❌ No (one-way) |
| Formulas | Preserved exactly | ✅ N/A |

### Security Guarantees

- **No data in rules file** — Only transformation parameters, never original values
- **Deterministic hashing** — Same input always produces same hash (for consistency)
- **Formula integrity** — Calculated values may change (due to data changes), but formulas remain identical
- **Memory safe** — Large files processed in chunks, sensitive data not persisted

### Best Practices

1. **Keep rules files secure** — They're the "keys" to your encrypted data
2. **Use hashing for PII** — Names, emails, IDs should use `hash` transform (one-way)
3. **Test round-trips** — Always verify your workflow before production use
4. **Version control** — Rules files are safe to commit (contain no data)

---

## 🏗️ Project Structure

```
eYcel/
├── src/eYcel/
│   ├── __init__.py           # Package init & public API
│   ├── cli.py                # CLI entry point (eYcel command)
│   ├── encrypt.py            # Encryption pipeline
│   ├── decrypt.py            # Decryption pipeline
│   ├── formula_handler.py    # Formula preservation logic
│   ├── column_analyzer.py    # Data type detection
│   ├── transformations.py    # Hash, offset, scale, shuffle transforms
│   ├── yaml_handler.py       # Rules file read/write/validation
│   ├── memory_utils.py       # Memory-efficient processing
│   └── exceptions.py         # Custom exception classes
├── tests/
│   ├── conftest.py           # Shared fixtures
│   ├── test_cli.py
│   ├── test_column_analyzer.py
│   ├── test_decrypt.py
│   ├── test_encrypt.py
│   ├── test_formula_handler.py
│   ├── test_integration.py
│   ├── test_memory_utils.py
│   ├── test_transformations.py
│   └── test_yaml_handler.py
├── examples/
│   ├── generate_sample_data.py     # Script to create sample data
│   ├── process_sample.py           # End-to-end example script
│   ├── sample_data.xlsx            # Example input file
│   ├── sample_data_encrypted.xlsx  # Example encrypted output
│   ├── sample_data_restored.xlsx   # Example decrypted output
│   └── sample_data_rules.yaml     # Example rules file
├── gui/
│   └── app.py                # Streamlit web interface
├── pyproject.toml            # Build configuration & metadata
├── setup.py                  # Legacy setup (setuptools)
├── requirements.txt
├── LICENSE
├── MANIFEST.in
├── .github/                  # CI/CD workflows
└── ProjectBuildingPlan.md
```

---

## 🤝 Contributing

We welcome contributions! Please follow these guidelines:

1. **Fork the repo** and create a feature branch
2. **Install dev dependencies**: `pip install -e ".[dev]"`
3. **Run tests before committing**: `pytest tests/`
4. **Maintain coverage**: New code must keep coverage ≥ 80%
5. **Follow style**: `flake8 src/ --max-line-length=100`
6. **Submit a PR** with clear description

### Development Setup

```bash
git clone https://github.com/VoicuTomut/eYcel.git
cd eYcel
python -m venv .venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate
pip install -e ".[dev]"
pytest tests/  # Verify all tests pass
```

### Running Tests

```bash
# Run all tests
pytest tests/

# With coverage report
pytest tests/ --cov=src/eYcel --cov-report=term-missing

# Specific test file
pytest tests/test_transformations.py -v
```

---

## 📋 Requirements

- Python 3.9, 3.10, 3.11, or 3.12
- openpyxl ≥ 3.0
- PyYAML ≥ 6.0

**Optional:**
- `streamlit` — for the web GUI (`gui/app.py`)
- `tqdm` — for CLI progress bars (`pip install eYcel[cli]`)
- `memory-profiler` — for development profiling (`pip install eYcel[dev]`)

No heavy dependencies. No pandas. Runs on any machine.

---

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

---

## 📄 License

MIT License — see [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Built with [openpyxl](https://openpyxl.readthedocs.io/) for Excel handling
- Inspired by data privacy needs in financial and healthcare industries

---

## 💬 Support

- 📧 Issues: [GitHub Issues](https://github.com/VoicuTomut/eYcel/issues)
- 💡 Discussions: [GitHub Discussions](https://github.com/VoicuTomut/eYcel/discussions)

---

<p align="center">
  <b>Made with ❤️ for data privacy</b>
</p>
