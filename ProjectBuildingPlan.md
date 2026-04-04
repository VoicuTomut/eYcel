# eYcel — Project Building Plan

## Vision
Build eYcel as a lightweight, installable Python application for Excel data anonymization.
**Target:** `pip install eYcel` or `pipx install eYcel` on any machine (macOS, Linux, Windows).

---

## Phase 1 — Project Foundation ✅ (Week 1)
### Goals
- [x] GitHub repository created (`VoicuTomut/eYcel`)
- [x] README.md — comprehensive, with workflow diagram
- [x] `.env.example` — environment configuration template
- [x] `.gitignore` — Python + eYcel specific ignores
- [x] `ProjectBuildingPlan.md` — this file
- [ ] Project skeleton (`src/`, `tests/`, `examples/`)
- [ ] `requirements.txt` and `pyproject.toml`
- [ ] Basic CI/CD with GitHub Actions (`pytest` on push)

### Deliverables
- `pyproject.toml` with CLI entry point: `eYcel = "eYcel.cli:main"`
- `requirements.txt` (openpyxl, PyYAML, click — nothing else)
- `src/eYcel/__init__.py`
- `.github/workflows/tests.yml` — runs pytest on every push

---

## Phase 2 — Core Engine (Week 2)
### Goals
- [ ] **Column Analyzer** — detect data types per column: `date`, `float`, `int`, `%`, `string`, `formula`
- [ ] **Transformation Engine** — apply per-column rules: `hash`, `offset`, `shuffle`, `anonymize`, `keep`, `rename`
- [ ] **Formula Handler** — extract formulas before transform, re-inject after, verify integrity
- [ ] **YAML Handler** — generate and load `rules.yaml` (zero original data stored)

### Modules to Create
```
src/eYcel/
├── column_analyzer.py    # Type detection per column
├── transformations.py    # All transform logic
├── formula_handler.py    # Formula extract / preserve / re-inject
└── yaml_handler.py       # rules.yaml read/write
```

### Tests Required
```
tests/
├── test_column_analyzer.py
├── test_transformations.py
└── test_formula_handler.py
```

### Key Constraints
- No pandas in core engine
- Memory: process in chunks of 1000 rows
- Use openpyxl read-only mode for large files
- Cache formula parsing results

---

## Phase 3 — Encrypt / Decrypt Modules (Week 3)
### Goals
- [ ] **Encrypt module** — ingest Excel → apply transforms → output `encrypted.xlsx` + `rules.yaml`
- [ ] **Decrypt module** — ingest `encrypted.xlsx` + `rules.yaml` → restore original data exactly
- [ ] **Round-trip test** — `encrypt → decrypt == original` for all data types
- [ ] **Memory profiling** — verify < 50MB for 100k rows

### Modules to Create
```
src/eYcel/
├── encrypt.py    # Main encryption pipeline
└── decrypt.py    # Main decryption pipeline
```

### Tests Required
```
tests/
├── test_encrypt.py
├── test_decrypt.py
├── test_integration.py     # Full round-trip: encrypt → decrypt == original
└── test_performance.py     # Memory usage < 50MB for 100k rows
```

### Success Criteria for This Phase
1. All formulas survive the encrypt → decrypt cycle unchanged
2. Every data type round-trips perfectly: dates, floats, ints, %, strings
3. `rules.yaml` contains **zero original data values**
4. Memory usage stays under 50MB for 100k row files

---

## Phase 4 — CLI Interface (Week 4)
### Goals
- [ ] CLI with `click`: `eYcel encrypt`, `eYcel decrypt`, `eYcel validate`
- [ ] **Interactive mode** — prompt user for per-column transformation rules
- [ ] **Batch mode** — pass rules config via YAML flag (`--rules config.yaml`)
- [ ] Progress indicators for large files
- [ ] `--verbose` and `--quiet` flags

### Module to Create
```
src/eYcel/
└── cli.py    # click-based CLI entry point
```

### CLI Usage
```bash
# Basic encrypt
eYcel encrypt sales.xlsx

# With options
eYcel encrypt sales.xlsx --output ./secure/ --interactive --verbose

# Decrypt
eYcel decrypt sales_encrypted.xlsx sales_rules.yaml

# Validate rules file
eYcel validate sales_rules.yaml

# Batch with pre-defined rules
eYcel encrypt sales.xlsx --rules my_column_rules.yaml --quiet
```

---

## Phase 5 — Packaging & Distribution (Week 5)
### Goals
- [ ] `pyproject.toml` fully configured for PyPI upload
- [ ] `pipx install eYcel` works on macOS, Linux, Windows
- [ ] GitHub Actions: auto-publish to PyPI on version tag (`v*.*.*`)
- [ ] Semantic versioning (`MAJOR.MINOR.PATCH`)
- [ ] Cross-platform testing (macOS, Ubuntu, Windows in CI)

### Deliverables
- Working `pip install eYcel` from PyPI
- Working `pipx install eYcel` (isolated, no venv needed)
- GitHub Release workflow: tag → build → publish

### pyproject.toml key config
```toml
[project]
name = "eYcel"
requires-python = ">=3.9"
dependencies = ["openpyxl>=3.1.0", "PyYAML>=6.0", "click>=8.0"]

[project.scripts]
eYcel = "eYcel.cli:main"
```

---

## Phase 6 — Optional GUI (Future / Post v1.0)
### Goals
- [ ] Choose: **Streamlit** (web) or **Tkinter** (desktop)
- [ ] Drag-and-drop Excel file upload
- [ ] Visual column rule selector (table with dropdowns)
- [ ] One-click encrypt / decrypt
- [ ] Preview of anonymized output before saving

---

## Technical Constraints (Non-Negotiable)
| Constraint | Value |
|------------|-------|
| Python version | 3.9+ |
| Core dependencies | openpyxl, PyYAML, click ONLY |
| Memory limit | < 50MB for 100k rows |
| pandas in core | ❌ Never |
| Formula preservation | ✅ Always |
| Original data in rules.yaml | ❌ Never |
| Test coverage | ≥ 80% |

---

## Success Criteria (Final Checklist)
- [ ] `pip install eYcel` works on macOS, Linux, Windows
- [ ] `pipx install eYcel` works as single isolated install
- [ ] `eYcel encrypt data.xlsx` → produces `encrypted.xlsx` + `rules.yaml`
- [ ] `eYcel decrypt encrypted.xlsx rules.yaml` → restores original perfectly
- [ ] All Excel formulas preserved through full encrypt → decrypt cycle
- [ ] Memory usage < 50MB for files with 100k rows
- [ ] Test coverage ≥ 80%
- [ ] `rules.yaml` contains zero original data values
- [ ] CI/CD: tests pass on every push, auto-publish on tag

---

## Repository
- **GitHub:** https://github.com/VoicuTomut/eYcel
- **Owner:** VoicuTomut (Andrei Voicu Tomuț)
- **License:** MIT
