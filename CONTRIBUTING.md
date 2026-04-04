# Contributing to eYcel 🤝

Thank you for your interest in contributing to **eYcel**! Whether it's a bug fix, new feature, documentation improvement, or test — every contribution matters.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Submitting Pull Requests](#submitting-pull-requests)
- [Development Workflow](#development-workflow)
  - [Branching Strategy](#branching-strategy)
  - [Coding Standards](#coding-standards)
  - [Testing](#testing)
  - [Commit Messages](#commit-messages)
- [Architecture Overview](#architecture-overview)
- [License](#license)

---

## Code of Conduct

Be respectful, constructive, and inclusive. We follow the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). In short:

- Use welcoming and inclusive language
- Respect differing viewpoints and experiences
- Accept constructive criticism gracefully
- Focus on what is best for the project and the community

---

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/<your-username>/eYcel.git
   cd eYcel
   ```
3. **Add upstream** remote:
   ```bash
   git remote add upstream https://github.com/VoicuTomut/eYcel.git
   ```

---

## Development Setup

### Prerequisites

- Python 3.9 or higher
- Git

### Install in Development Mode

```bash
# Create a virtual environment (recommended)
python -m venv .venv
source .venv/bin/activate    # Linux/macOS
# .venv\Scripts\activate     # Windows

# Install the package with dev dependencies
pip install -e ".[dev]"

# Install linting tools
pip install flake8
```

### Verify the Setup

```bash
# Run the test suite
pytest tests/ -v

# Verify CLI works
eYcel --help

# Run linting
flake8 src/ --max-line-length=100 --exclude=__pycache__
```

If all tests pass and the CLI responds, you're ready to contribute.

---

## Project Structure

```
eYcel/
├── src/eYcel/                # Core package
│   ├── __init__.py           # Public API exports
│   ├── cli.py                # CLI entry point (argparse)
│   ├── encrypt.py            # Encryption pipeline
│   ├── decrypt.py            # Decryption pipeline
│   ├── transformations.py    # Transform functions (hash, offset, scale, shuffle, etc.)
│   ├── formula_handler.py    # Formula extraction & restoration
│   ├── column_analyzer.py    # Automatic column type detection
│   ├── yaml_handler.py       # Rules YAML read/write/validation
│   ├── memory_utils.py       # Memory profiling utilities
│   └── exceptions.py         # Custom exception classes
├── tests/                    # Test suite (pytest)
│   ├── conftest.py           # Shared fixtures
│   ├── test_encrypt.py
│   ├── test_decrypt.py
│   ├── test_transformations.py
│   ├── test_formula_handler.py
│   ├── test_column_analyzer.py
│   ├── test_yaml_handler.py
│   ├── test_cli.py
│   ├── test_memory_utils.py
│   └── test_integration.py   # End-to-end encrypt→decrypt tests
├── examples/                 # Example Excel files and usage scripts
├── gui/                      # Streamlit web interface
│   └── streamlit_app.py
├── .github/workflows/        # CI/CD (GitHub Actions)
├── pyproject.toml            # Package metadata & build config
├── setup.py                  # Legacy build support
├── README.md
├── CONTRIBUTING.md           # ← You are here
└── LICENSE                   # MIT License
```

### Key Modules

| Module | Responsibility |
|--------|---------------|
| `encrypt.py` | Orchestrates encryption: analyzes columns → selects transforms → applies them → writes output + rules |
| `decrypt.py` | Reverses encryption using rules YAML: loads rules → applies inverse transforms → restores original data |
| `transformations.py` | Individual transform functions (`hash`, `offset`, `scale`, `shuffle`, `keep`, `anonymize`) |
| `formula_handler.py` | Extracts formulas before encryption and restores them after — the core differentiator of eYcel |
| `column_analyzer.py` | Inspects columns to detect dominant data types and recommend transforms |
| `yaml_handler.py` | Reads, writes, and validates the rules YAML schema |
| `cli.py` | Argument parsing and CLI command dispatch |

---

## How to Contribute

### Reporting Bugs

Open an [issue](https://github.com/VoicuTomut/eYcel/issues) with:

- **Title**: Short, descriptive summary
- **Environment**: Python version, OS, eYcel version (`eYcel --version`)
- **Steps to reproduce**: Minimal commands or code to trigger the bug
- **Expected vs. actual behavior**
- **Sample file** (if possible): A minimal `.xlsx` that demonstrates the issue (make sure it contains no real sensitive data)

### Suggesting Features

Open an issue tagged `enhancement` with:

- **Problem**: What pain point does this solve?
- **Proposed solution**: How you'd like it to work
- **Alternatives considered**: Other approaches you thought about
- **Impact on existing features**: Any backwards-compatibility concerns

### Submitting Pull Requests

1. Create a feature branch from `main` (see [Branching Strategy](#branching-strategy))
2. Make your changes following the [Coding Standards](#coding-standards)
3. Add or update tests to cover your changes
4. Ensure all tests pass: `pytest tests/ -v`
5. Ensure linting passes: `flake8 src/ --max-line-length=100`
6. Push to your fork and open a PR against `main`

**PR checklist:**

- [ ] Tests added/updated and all passing
- [ ] Code follows project style conventions
- [ ] Docstrings added for new public functions/classes
- [ ] No new dependencies added without discussion
- [ ] `eYcel --help` still works correctly
- [ ] Formulas are still preserved end-to-end (run integration tests)

---

## Development Workflow

### Branching Strategy

```
main                ← stable, release-ready
├── feat/xxx        ← new features
├── fix/xxx         ← bug fixes
├── docs/xxx        ← documentation only
└── test/xxx        ← test improvements
```

Branch naming examples:
- `feat/add-csv-support`
- `fix/formula-preservation-multisheet`
- `docs/update-api-examples`
- `test/improve-transform-coverage`

### Coding Standards

- **Style**: PEP 8, max line length 100 characters
- **Linting**: `flake8 src/ --max-line-length=100 --exclude=__pycache__`
- **Type hints**: Use type annotations on all public function signatures
- **Docstrings**: Google-style docstrings on all public functions and classes:

  ```python
  def encrypt_excel(input_path: str, output_path: str) -> str:
      """Encrypt an Excel file, producing an anonymized copy and rules file.

      Args:
          input_path: Path to the source Excel file.
          output_path: Path for the encrypted output file.

      Returns:
          Path to the generated rules YAML file.

      Raises:
          FileNotFoundError: If input_path does not exist.
          EncryptionError: If encryption fails.
      """
  ```

- **Imports**: Standard library → third-party → local, separated by blank lines
- **No wildcard imports**: Always use explicit imports
- **Dependencies**: eYcel aims to stay lightweight. Avoid adding heavy dependencies (no pandas, no numpy). Discuss in an issue before adding any new dependency.

### Testing

We use **pytest** with the following conventions:

```bash
# Run all tests
pytest tests/ -v

# Run a specific test file
pytest tests/test_encrypt.py -v

# Run with coverage report
pytest tests/ --cov=src/eYcel --cov-report=term-missing

# Run only integration tests
pytest tests/test_integration.py -v
```

**Testing guidelines:**

- Every new feature needs tests. Every bug fix needs a regression test.
- Test files follow the naming pattern `tests/test_<module>.py`
- Use fixtures from `conftest.py` for shared setup (temp workbooks, sample data)
- Integration tests should verify the full encrypt → decrypt round-trip
- **Formula preservation is sacred** — always include formula round-trip assertions in integration tests
- Target ≥80% code coverage

**Writing a good test:**

```python
def test_scale_transform_roundtrip(self, sample_workbook):
    """Scale transform should be perfectly reversible."""
    # Arrange
    original_value = 1000.0

    # Act
    encrypted = apply_scale(original_value, factor=0.73)
    decrypted = reverse_scale(encrypted, factor=0.73)

    # Assert
    assert decrypted == pytest.approx(original_value, rel=1e-9)
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <short description>

[optional body — explain WHY, not what]

[optional footer — references issues]
```

**Types:**

| Type | Use for |
|------|---------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation changes only |
| `test` | Adding or updating tests |
| `refactor` | Code restructuring (no behavior change) |
| `chore` | Build, CI, tooling changes |
| `style` | Formatting, whitespace (no logic change) |

**Examples:**

```
feat: add CSV export option for rules file

fix: preserve merged cell formulas during encryption

Formulas in merged cells were being dropped because the handler
only checked the top-left cell. Now scans all cells in merged ranges.

Fixes #42

test: add coverage for multi-sheet formula round-trip

docs: add CLI usage examples to README
```

---

## Architecture Overview

Understanding the data flow helps when making changes:

```
                    ┌─────────────────────┐
                    │   column_analyzer    │  ← inspects data types per column
                    └──────────┬──────────┘
                               │ column metadata
                               ▼
┌───────────┐     ┌─────────────────────┐     ┌──────────────────┐
│  Input     │────►│      encrypt        │────►│  Output          │
│  .xlsx     │     │  (orchestrator)     │     │  _encrypted.xlsx │
└───────────┘     └──────┬──────────────┘     └──────────────────┘
                         │         │
                         │         │
              ┌──────────▼──┐  ┌───▼────────────┐
              │ formula_     │  │ transformations │  ← hash, offset, scale, etc.
              │ handler      │  └───┬────────────┘
              │ (extract)    │      │
              └──────────────┘      │
                                    ▼
                            ┌───────────────┐
                            │ yaml_handler   │  ← writes rules.yaml
                            └───────────────┘

Decryption is the reverse:
  rules.yaml → load_and_validate_rules → reverse transforms → restore formulas → output.xlsx
```

**Key invariant:** Formulas are **never** transformed. They are extracted before encryption and restored after. This is the core design principle of eYcel.

---

## Adding a New Transformation

If you want to add a new transformation type (e.g., `mask`):

1. **Define the transform** in `src/eYcel/transformations.py`:
   - Forward function: `apply_mask(value, **params) -> transformed_value`
   - Reverse function: `reverse_mask(value, **params) -> original_value`

2. **Register it** in the transform dispatcher in `encrypt.py` and `decrypt.py`

3. **Add auto-detection logic** in `column_analyzer.py` (if applicable)

4. **Update the YAML schema** in `yaml_handler.py` to accept the new transform name

5. **Write tests** in `tests/test_transformations.py`:
   - Unit test for forward transform
   - Unit test for reverse transform
   - Round-trip test: `reverse(forward(x)) == x`

6. **Add an integration test** in `tests/test_integration.py` that exercises the new transform end-to-end

7. **Update documentation**: README transform table, this file if needed

---

## Questions?

- Open a [GitHub issue](https://github.com/VoicuTomut/eYcel/issues) for bugs or features
- Start a [GitHub Discussion](https://github.com/VoicuTomut/eYcel/discussions) for questions or ideas

---

## License

By contributing to eYcel, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

*Thank you for helping make eYcel better! 🔐*
