# eYcel — Project Building Plan

> Goal: A lightweight, cross-platform Python tool that encrypts/decrypts Excel data
> while preserving formulas. Packaged as a simple `pip install eYcel` app.

---

## Phase 1 — Foundation ✅ (Week 1)

**Goal:** Repo ready, skeleton in place, CI configured.

- [x] Create GitHub repository (public)
- [x] Clone to MacBook at `~/github/eYcel`
- [x] Write improved README.md
- [x] Add .env.example
- [x] Add .gitignore
- [ ] Create project skeleton (folders + empty files)
- [ ] Add requirements.txt
- [ ] Add setup.py / pyproject.toml
- [ ] Configure GitHub Actions CI (run pytest on push)
- [ ] Push all to GitHub

**Deliverable:** `pip install -e .` works locally.

---

## Phase 2 — Core Engine (Week 2)

**Goal:** The transformation brain — analyze columns, apply rules.

### 2.1 Column Analyzer (`src/eYcel/column_analyzer.py`)
- [ ] Detect column data types: date, integer, float, percentage, string, categorical
- [ ] Identify formula cells vs data cells
- [ ] Return structured column metadata dict
- [ ] Unit tests: `tests/test_column_analyzer.py`

### 2.2 Transformation Engine (`src/eYcel/transformations.py`)
- [ ] `transform_hash(value, salt)` — SHA256 hash
- [ ] `transform_offset(value, offset)` — shift dates/numbers
- [ ] `transform_scale(value, factor)` — multiply numbers
- [ ] `transform_shuffle(value, mapping)` — rename categories
- [ ] `transform_keep(value)` — passthrough
- [ ] `transform_anonymize(value, col_type)` — type-aware fake value
- [ ] Unit tests for each transformation: `tests/test_transformations.py`

### 2.3 Formula Handler (`src/eYcel/formula_handler.py`)
- [ ] Extract all formula cells from workbook
- [ ] Store formula text separately from data
- [ ] Re-inject formulas after transformation
- [ ] Verify formulas unchanged after round-trip
- [ ] Unit tests: `tests/test_formula_handler.py`

### 2.4 YAML Handler (`src/eYcel/yaml_handler.py`)
- [ ] Generate rules.yaml from transformation decisions
- [ ] Validate rules.yaml schema on load
- [ ] Load rules.yaml for decryption
- [ ] Ensure NO original data stored in rules
- [ ] Unit tests: `tests/test_yaml_handler.py`

**Deliverable:** All core modules pass unit tests.

---

## Phase 3 — Encrypt / Decrypt Pipeline (Week 3)

**Goal:** Full working encrypt → decrypt round-trip.

### 3.1 Encryption (`src/eYcel/encrypt.py`)
- [ ] `encrypt_excel(input_path, output_path, rules=None)`
- [ ] Interactive mode: prompt user per column for transformation choice
- [ ] Batch mode: apply rules dict directly
- [ ] Generate `*_rules.yaml` alongside output
- [ ] Preserve all formulas exactly
- [ ] Memory-safe: chunk processing for >1000 rows
- [ ] Integration test: `tests/test_encrypt.py`

### 3.2 Decryption (`src/eYcel/decrypt.py`)
- [ ] `decrypt_excel(encrypted_path, rules_path, output_path)`
- [ ] Load and validate rules.yaml
- [ ] Apply reverse transformations column by column
- [ ] Re-inject original formulas
- [ ] Verify output matches original structure
- [ ] Integration test: `tests/test_decrypt.py`

### 3.3 Round-Trip Test
- [ ] `tests/test_integration.py`: encrypt → decrypt → compare with original
- [ ] All values match within tolerance (float precision)
- [ ] All formulas identical
- [ ] All sheet names/column headers preserved

### 3.4 Memory Profiling
- [ ] `examples/process_sample.py` with 100k row test file
- [ ] Confirm peak memory < 50MB
- [ ] `python -m memory_profiler examples/process_sample.py`

**Deliverable:** `encrypt_excel()` + `decrypt_excel()` fully functional and tested.

---

## Phase 4 — CLI Interface (Week 4)

**Goal:** Simple, user-friendly command-line tool.

### 4.1 CLI (`eYcel_cli.py`)
- [ ] `eYcel encrypt --input <file> --output <file> [--rules <file>] [--quiet]`
- [ ] `eYcel decrypt --input <file> --rules <file> --output <file>`
- [ ] `eYcel validate --rules <file>`
- [ ] Interactive column selection (when no rules provided)
- [ ] Progress bar for large files (`tqdm` optional)
- [ ] `--verbose` and `--quiet` flags
- [ ] Friendly error messages

### 4.2 Entry Point Registration
- [ ] Register `eYcel` command in `setup.py` / `pyproject.toml`
- [ ] Test: `eYcel --help` works after `pip install`

**Deliverable:** Full CLI works end-to-end on sample data.

---

## Phase 5 — Packaging & Distribution (Week 5)

**Goal:** `pip install eYcel` works on Windows, Mac, Linux.

### 5.1 Package Setup
- [ ] Write `pyproject.toml` (modern packaging)
- [ ] Write `setup.py` (legacy fallback)
- [ ] Add `MANIFEST.in` for non-Python files
- [ ] Set minimum Python version: 3.9

### 5.2 Cross-Platform Testing
- [ ] Test install on macOS (local MacBook)
- [ ] Test install on Linux (MCPtest server)
- [ ] Test install on Windows (VM or GitHub Actions)
- [ ] GitHub Actions matrix: `[ubuntu, macos, windows] x [py3.9, py3.11]`

### 5.3 PyPI Publishing
- [ ] Register on PyPI
- [ ] `python -m build`
- [ ] `twine upload dist/*`
- [ ] Verify: `pip install eYcel` from PyPI

### 5.4 pipx Support
- [ ] Verify `pipx install eYcel` works
- [ ] Add pipx install instructions to README

**Deliverable:** `pip install eYcel` works on any machine. `eYcel encrypt` runs from terminal.

---

## Phase 6 — GUI App (Future / Post v1.0)

**Goal:** A simple drag-and-drop GUI for non-technical users.

### Option A — Streamlit (Web-based, easiest)
- [ ] `app/streamlit_app.py`
- [ ] Upload Excel file
- [ ] Select transformations per column via dropdowns
- [ ] Download encrypted file + rules
- [ ] `streamlit run app/streamlit_app.py`

### Option B — Tkinter (Desktop, no server needed)
- [ ] `app/tkinter_app.py`
- [ ] File picker for input
- [ ] Table view for column selection
- [ ] Progress bar
- [ ] Works offline, no browser needed

### Option C — Electron/Tauri wrapper (Future)
- [ ] Wrap Python CLI as a native desktop app
- [ ] One-click installer for Windows/Mac

**Deliverable:** Non-technical users can use eYcel without terminal.

---

## Tech Stack Summary

| Component | Technology | Why |
|-----------|-----------|-----|
| Excel I/O | `openpyxl` | Preserves formulas, memory efficient |
| Rules file | `PyYAML` | Human-readable, simple |
| Hashing | `hashlib` (built-in) | No extra dependency |
| CLI | `argparse` (built-in) | No extra dependency |
| Testing | `pytest` | Standard, fast |
| Packaging | `pyproject.toml` | Modern Python standard |
| CI/CD | GitHub Actions | Free, cross-platform |
| GUI (future) | Streamlit | Fastest to build |

---

## Key Constraints (Never Violate)

1. **Formulas must be preserved exactly** — this is the core value proposition
2. **Rules file must contain zero original data** — security guarantee
3. **Memory < 50MB for 100k rows** — efficiency requirement
4. **No pandas** — too heavy for this use case
5. **Python 3.9+ only** — modern but widely available
6. **Works offline** — no cloud dependency

---

## Version Milestones

| Version | Description |
|---------|-------------|
| v0.1.0 | Core engine — column analyzer + transformations |
| v0.2.0 | Full encrypt/decrypt pipeline |
| v0.3.0 | CLI interface |
| v1.0.0 | PyPI release — installable on any machine |
| v1.1.0 | Streamlit GUI |
| v2.0.0 | Native desktop app |
