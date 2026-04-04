# eYcel — Detailed Project Building Plan

> **Goal:** A lightweight, cross-platform Python tool that encrypts/decrypts Excel data while preserving formulas. Packaged as `pip install eYcel`.

---

## Execution Strategy

### Agent-Driven Development Approach

This project will be built **one phase at a time** using focused sub-agents:

1. **Phase-by-Phase Execution**: Each phase is treated as a complete milestone
2. **Test-First Development**: Every sub-task includes acceptance criteria; tests run BEFORE marking complete
3. **Local Validation on MacBook**: All code executes on `~/GitHub/eYcel` on the user's MacBook
4. **Commit After Phase Passes**: Only commit to GitHub when all tests in a phase pass
5. **Agent Selection**: Tasks use appropriate agents:
   - **Simple structural tasks** → Direct tool execution (no agent)
   - **Code implementation** → Worker LLM sub-agents (spawn_subtask)
   - **Complex integration** → Master agent or user review
6. **Efficiency Rules**:
   - Group trivial tasks (folder creation, empty files) into single operations
   - Use sub-agents for logic-heavy modules (analyzers, transformations)
   - Run tests immediately after each module completes

### Task Location Legend
- 🖥️ **MacBook** — Must run on local machine (SSH to `macbook`)
- ☁️ **Workspace** — Can be done in conversation workspace
- 🔄 **Both** — May involve both environments

---

## Phase 1 — Foundation ✅ (Week 1)

**Goal:** Repository structure ready, skeleton in place, dependencies configured.

### 1.1 Repository Structure Setup 🖥️ [x] — **L (Done)**
- [x] 1.1.1 Clone/create GitHub repo at `~/GitHub/eYcel`
- [x] 1.1.2 Create `src/eYcel/` directory structure
- [x] 1.1.3 Create `tests/` directory structure  
- [x] 1.1.4 Create `examples/` directory
- [x] 1.1.5 Create `.github/workflows/` for CI

**Acceptance Criteria:**
- `ls -la ~/GitHub/eYcel/` shows all directories
- Git status shows clean working directory

---

### 1.2 Core Configuration Files ☁️ → 🖥️ [x] — **M (Done)**
- [x] 1.2.1 Write `requirements.txt` with: openpyxl>=3.1.0, PyYAML>=6.0, pytest>=7.0.0
- [x] 1.2.2 Write `setup.py` with entry point `eYcel=eYcel_cli:main`
- [x] 1.2.3 Write `pyproject.toml` for modern packaging
- [x] 1.2.4 Write `MANIFEST.in` to include non-Python files

**Acceptance Criteria:**
- `pip install -e .` succeeds on MacBook
- `eYcel --help` shows CLI help (after CLI is built)

---

### 1.3 Project Skeleton Files 🖥️ [ ] — **M**
- [ ] 1.3.1 Create `src/eYcel/__init__.py` with version export
- [ ] 1.3.2 Create `src/eYcel/encrypt.py` (empty with docstring)
- [ ] 1.3.3 Create `src/eYcel/decrypt.py` (empty with docstring)
- [ ] 1.3.4 Create `src/eYcel/column_analyzer.py` (empty with docstring)
- [ ] 1.3.5 Create `src/eYcel/formula_handler.py` (empty with docstring)
- [ ] 1.3.6 Create `src/eYcel/yaml_handler.py` (empty with docstring)
- [ ] 1.3.7 Create `src/eYcel/transformations.py` (empty with docstring)
- [ ] 1.3.8 Create `src/eYcel/exceptions.py` for custom errors
- [ ] 1.3.9 Create `eYcel_cli.py` at project root
- [ ] 1.3.10 Create `tests/__init__.py`
- [ ] 1.3.11 Create `tests/conftest.py` for pytest fixtures

**Acceptance Criteria:**
- All files exist and are importable
- `python -c "from src.eYcel import encrypt"` succeeds

---

### 1.4 CI/CD Configuration ☁️ → 🖥️ [ ] — **S**
- [ ] 1.4.1 Create `.github/workflows/ci.yml` with pytest on push
- [ ] 1.4.2 Add Python matrix: 3.9, 3.10, 3.11
- [ ] 1.4.3 Add platforms: ubuntu-latest, macos-latest
- [ ] 1.4.4 Test CI by pushing to a branch

**Acceptance Criteria:**
- GitHub Actions runs on push
- All matrix jobs pass

---

### 1.5 Documentation Foundation ☁️ [x] — **M (Done)**
- [x] 1.5.1 Write comprehensive `README.md`
- [x] 1.5.2 Add `.env.example` with all configuration options
- [x] 1.5.3 Add `.gitignore` (Python + eYcel-specific)

**Acceptance Criteria:**
- README renders correctly on GitHub
- `cp .env.example .env` creates valid config

---

### Phase 1 Deliverable
```bash
cd ~/GitHub/eYcel
python -c "import src.eYcel; print('Import OK')"
pytest tests/ --collect-only  # Should find tests (even if empty)
```

---

## Phase 2 — Core Engine (Week 2)

**Goal:** The transformation brain — analyze columns, apply rules, handle formulas, manage YAML.

### 2.1 Column Analyzer Module 🖥️ [ ] — **L** (Sub-agent recommended)
- [ ] 2.1.1 Implement `detect_cell_type(cell_value)` → returns: date, int, float, percentage, string, categorical, formula
- [ ] 2.1.2 Implement `is_formula_cell(cell)` using openpyxl
- [ ] 2.1.3 Implement `analyze_column(worksheet, column_letter)` → returns metadata dict
- [ ] 2.1.4 Implement `detect_categorical(values, threshold=0.2)` → bool if categorical
- [ ] 2.1.5 Implement `get_column_stats(values)` → min, max, avg, unique_count
- [ ] 2.1.6 Write `tests/test_column_analyzer.py` with 90%+ coverage

**Acceptance Criteria:**
- Correctly identifies dates, numbers, strings in sample Excel
- Distinguishes formula cells from data cells
- All unit tests pass: `pytest tests/test_column_analyzer.py -v`

**Effort:** L (complex logic, comprehensive tests)

---

### 2.2 Transformation Engine 🖥️ [ ] — **L** (Sub-agent recommended)
- [ ] 2.2.1 Implement `transform_hash(value: str, salt: str) -> str` using hashlib
- [ ] 2.2.2 Implement `transform_offset_date(date_val, offset_days: int) -> datetime`
- [ ] 2.2.3 Implement `transform_offset_number(num_val, offset: float) -> float`
- [ ] 2.2.4 Implement `transform_scale(value: float, factor: float) -> float`
- [ ] 2.2.5 Implement `transform_shuffle(value: str, mapping: dict) -> str`
- [ ] 2.2.6 Implement `transform_keep(value) -> value` (passthrough)
- [ ] 2.2.7 Implement `transform_anonymize(value, col_type: str) -> fake_value`
- [ ] 2.2.8 Implement reverse transformations for decrypt (where applicable)
- [ ] 2.2.9 Write `tests/test_transformations.py` with all transformations

**Acceptance Criteria:**
- `transform_hash("test", "salt")` returns consistent SHA256
- `transform_scale(100, 0.5)` returns 50.0
- `transform_shuffle("A", {"A": "X"})` returns "X"
- Reverse operations restore original values (except hash)
- All unit tests pass

**Effort:** L (7 transformation functions + reverse + tests)

---

### 2.3 Formula Handler Module 🖥️ [ ] — **M** (Sub-agent recommended)
- [ ] 2.3.1 Implement `extract_formulas(worksheet) -> dict[(row, col), formula_str]`
- [ ] 2.3.2 Implement `store_formulas(formula_dict, temp_storage)`
- [ ] 2.3.3 Implement `clear_formula_cells(worksheet, formula_cells)`
- [ ] 2.3.4 Implement `reinject_formulas(worksheet, formula_dict)`
- [ ] 2.3.5 Implement `verify_formulas_preserved(original_wb, processed_wb) -> bool`
- [ ] 2.3.6 Write `tests/test_formula_handler.py`

**Acceptance Criteria:**
- Extracts all formulas from sample Excel without modification
- Re-injected formulas are character-for-character identical
- Formula cells can be cleared and restored
- All tests pass

**Effort:** M (openpyxl-specific logic, requires sample Excel file)

---

### 2.4 YAML Handler Module 🖥️ [ ] — **M** (Sub-agent recommended)
- [ ] 2.4.1 Implement `generate_rules(metadata: dict, columns: dict) -> dict`
- [ ] 2.4.2 Implement `save_rules(rules_dict, filepath)` using PyYAML
- [ ] 2.4.3 Implement `load_rules(filepath) -> dict` with validation
- [ ] 2.4.4 Implement `validate_rules(rules_dict) -> (bool, errors_list)`
- [ ] 2.4.5 Implement `sanitize_rules(rules_dict) -> rules_dict` (ensure no original data)
- [ ] 2.4.6 Define YAML schema validation rules
- [ ] 2.4.7 Write `tests/test_yaml_handler.py`

**Acceptance Criteria:**
- Generated YAML contains no original cell data
- YAML round-trip: save → load produces identical structure
- Validation catches malformed rules (missing required keys)
- Schema enforced on load
- All tests pass

**Effort:** M (YAML handling + schema validation)

---

### 2.5 Memory-Efficient Processing Utilities 🖥️ [ ] — **M**
- [ ] 2.5.1 Implement `chunk_iterator(worksheet, chunk_size=1000)` generator
- [ ] 2.5.2 Implement `process_column_in_chunks(worksheet, column, processor_func)`
- [ ] 2.5.3 Implement `get_memory_usage_mb() -> float` using psutil or sys
- [ ] 2.5.4 Add memory warning when >EYCEL_MAX_MEMORY_MB
- [ ] 2.5.5 Write `tests/test_memory_utils.py`

**Acceptance Criteria:**
- 100k row file processes in <50MB peak memory
- Chunking doesn't lose data
- Memory warnings trigger correctly
- All tests pass

**Effort:** M (performance-critical code)

---

### Phase 2 Deliverable
```bash
cd ~/GitHub/eYcel
pytest tests/test_column_analyzer.py tests/test_transformations.py \
       tests/test_formula_handler.py tests/test_yaml_handler.py -v
# All tests pass
```

---

## Phase 3 — Encrypt / Decrypt Pipeline (Week 3)

**Goal:** Full working encrypt → decrypt round-trip with formula preservation.

### 3.1 Encryption Module 🖥️ [ ] — **L** (Sub-agent recommended)
- [ ] 3.1.1 Implement `encrypt_excel(input_path, output_path, rules=None)` main function
- [ ] 3.1.2 Implement `auto_detect_transform(column_data, column_name) -> transform_type`
- [ ] 3.1.3 Implement interactive mode: prompt user per column for transform choice
- [ ] 3.1.4 Implement batch mode: apply rules dict directly without prompts
- [ ] 3.1.5 Implement `generate_output_paths(input_path) -> (encrypted_path, rules_path)`
- [ ] 3.1.6 Ensure formulas are extracted before data transformation
- [ ] 3.1.7 Ensure formulas are re-injected after transformation
- [ ] 3.1.8 Generate and save `*_rules.yaml` alongside encrypted file
- [ ] 3.1.9 Add progress indicator for large files (print row count)
- [ ] 3.1.10 Write `tests/test_encrypt.py` with integration tests

**Acceptance Criteria:**
- `encrypt_excel("test.xlsx", "out.xlsx")` produces encrypted file + rules.yaml
- Encrypted file has different values but same formulas
- Rules file contains no original data
- Interactive mode prompts for each column
- Batch mode runs silently with provided rules
- All tests pass

**Effort:** L (main API + interactive + batch modes)

---

### 3.2 Decryption Module 🖥️ [ ] — **L** (Sub-agent recommended)
- [ ] 3.2.1 Implement `decrypt_excel(encrypted_path, rules_path, output_path)` main function
- [ ] 3.2.2 Implement `load_and_validate_rules(rules_path) -> rules_dict`
- [ ] 3.2.3 Implement reverse transformation dispatch based on transform type
- [ ] 3.2.4 Implement `apply_reverse_transform(column_data, transform_config) -> restored_data`
- [ ] 3.2.5 Ensure formulas are preserved during decryption
- [ ] 3.2.6 Add validation: encrypted file structure matches rules expectations
- [ ] 3.2.7 Add error handling for corrupted rules or encrypted files
- [ ] 3.2.8 Add progress indicator for large files
- [ ] 3.2.9 Write `tests/test_decrypt.py` with integration tests

**Acceptance Criteria:**
- `decrypt_excel("enc.xlsx", "rules.yaml", "out.xlsx")` restores original values
- Float values match within 0.001 tolerance
- All formulas identical to original
- Corrupted rules file raises clear error
- All tests pass

**Effort:** L (main API + reverse transforms + error handling)

---

### 3.3 Round-Trip Integration Tests 🖥️ [ ] — **M**
- [ ] 3.3.1 Create `tests/test_integration.py`
- [ ] 3.3.2 Test: encrypt → decrypt → compare with original (values match)
- [ ] 3.3.3 Test: formulas identical after round-trip
- [ ] 3.3.4 Test: sheet names preserved
- [ ] 3.3.5 Test: column headers preserved
- [ ] 3.3.6 Test: multiple sheets handled correctly
- [ ] 3.3.7 Test: empty sheets handled gracefully
- [ ] 3.3.8 Test: file with only formulas (no data) works
- [ ] 3.3.9 Test: special characters in sheet names work

**Acceptance Criteria:**
- All integration tests pass
- Round-trip produces byte-identical formulas
- Values match within tolerance
- Edge cases handled gracefully

**Effort:** M (comprehensive test scenarios)

---

### 3.4 Sample Data & Performance Tests 🖥️ [ ] — **M**
- [ ] 3.4.1 Create `examples/sample_data.xlsx` with test data
- [ ] 3.4.2 Create `examples/generate_large_file.py` (100k rows)
- [ ] 3.4.3 Create `examples/process_sample.py` demonstrating API
- [ ] 3.4.4 Run memory profile: `python -m memory_profiler examples/process_sample.py`
- [ ] 3.4.5 Verify peak memory < 50MB for 100k rows
- [ ] 3.4.6 Document performance benchmarks

**Acceptance Criteria:**
- `examples/sample_data.xlsx` opens in Excel
- 100k row file generates in <30 seconds
- Memory usage stays under 50MB
- Performance documented in README

**Effort:** M (test data generation + profiling)

---

### Phase 3 Deliverable
```bash
cd ~/GitHub/eYcel
pytest tests/test_encrypt.py tests/test_decrypt.py tests/test_integration.py -v
python examples/process_sample.py  # Runs without error
python -m memory_profiler examples/process_sample.py  # <50MB
```

---

## Phase 4 — CLI Interface (Week 4)

**Goal:** User-friendly command-line tool with progress indicators.

### 4.1 CLI Argument Parsing 🖥️ [ ] — **M** (Sub-agent recommended)
- [ ] 4.1.1 Implement `eYcel_cli.py` with argparse
- [ ] 4.1.2 Subcommand: `encrypt --input <file> --output <file> [--rules <file>] [--quiet]`
- [ ] 4.1.3 Subcommand: `decrypt --input <file> --rules <file> --output <file> [--quiet]`
- [ ] 4.1.4 Subcommand: `validate --rules <file>`
- [ ] 4.1.5 Global flag: `--verbose` / `-v`
- [ ] 4.1.6 Global flag: `--quiet` / `-q`
- [ ] 4.1.7 Global flag: `--version`
- [ ] 4.1.8 Help text for all commands and flags

**Acceptance Criteria:**
- `python eYcel_cli.py --help` shows help
- `python eYcel_cli.py encrypt --help` shows encrypt help
- All arguments parse correctly
- Invalid arguments show friendly error

**Effort:** M (argparse setup + validation)

---

### 4.2 CLI Implementation 🖥️ [ ] — **M**
- [ ] 4.2.1 Implement `encrypt_command(args)` calling `encrypt_excel()`
- [ ] 4.2.2 Implement `decrypt_command(args)` calling `decrypt_excel()`
- [ ] 4.2.3 Implement `validate_command(args)` loading and validating rules
- [ ] 4.2.4 Add input file existence check
- [ ] 4.2.5 Add output directory writability check
- [ ] 4.2.6 Add rules file validation before processing
- [ ] 4.2.7 Implement progress bar using `tqdm` (optional dependency)
- [ ] 4.2.8 Implement colored output for success/error (optional)
- [ ] 4.2.9 Write `tests/test_cli.py` with subprocess tests

**Acceptance Criteria:**
- `python eYcel_cli.py encrypt -i test.xlsx -o enc.xlsx` works
- `python eYcel_cli.py decrypt -i enc.xlsx -r rules.yaml -o out.xlsx` works
- `python eYcel_cli.py validate -r rules.yaml` works
- Non-existent input file shows clear error
- Progress shown for large files
- Exit code 0 on success, 1 on error
- All CLI tests pass

**Effort:** M (CLI logic + error handling + tests)

---

### 4.3 Entry Point Registration 🖥️ [ ] — **S**
- [ ] 4.3.1 Update `setup.py` with `console_scripts` entry point: `eYcel=eYcel_cli:main`
- [ ] 4.3.2 Update `pyproject.toml` with `[project.scripts]` entry point
- [ ] 4.3.3 Reinstall package: `pip install -e .`
- [ ] 4.3.4 Test: `eYcel --help` works globally
- [ ] 4.3.5 Test: `eYcel encrypt --help` works globally

**Acceptance Criteria:**
- `which eYcel` shows installed path
- `eYcel --version` shows version
- Works from any directory

**Effort:** S (configuration only)

---

### 4.4 CLI Documentation 🖥️ [ ] — **S**
- [ ] 4.4.1 Add CLI usage examples to README.md
- [ ] 4.4.2 Add error message guide (common issues + solutions)
- [ ] 4.4.3 Add example session transcript to docs/

**Acceptance Criteria:**
- README shows CLI examples
- Users can troubleshoot common errors

**Effort:** S (documentation)

---

### Phase 4 Deliverable
```bash
eYcel --help
eYcel encrypt --input examples/sample_data.xlsx --output /tmp/enc.xlsx
# Produces encrypted file + rules.yaml
eYcel decrypt --input /tmp/enc.xlsx --rules /tmp/enc_rules.yaml --output /tmp/out.xlsx
# Produces restored file
pytest tests/test_cli.py -v
```

---

## Phase 5 — Packaging & Distribution (Week 5)

**Goal:** `pip install eYcel` works on Windows, Mac, Linux.

### 5.1 Modern Packaging Setup 🖥️ [ ] — **M**
- [ ] 5.1.1 Finalize `pyproject.toml` with [build-system] requirements
- [ ] 5.1.2 Finalize `pyproject.toml` with [project] metadata (name, version, description, authors, license)
- [ ] 5.1.3 Add [project.optional-dependencies] for dev (pytest, memory_profiler)
- [ ] 5.1.4 Add [project.urls] (Homepage, Repository, Issues)
- [ ] 5.1.5 Add [project.classifiers] for PyPI
- [ ] 5.1.6 Update `setup.py` as legacy fallback
- [ ] 5.1.7 Update `MANIFEST.in` for all required files
- [ ] 5.1.8 Add `LICENSE` file (MIT)

**Acceptance Criteria:**
- `python -m build` succeeds
- `twine check dist/*` passes with no warnings
- Wheel and sdist created

**Effort:** M (packaging configuration)

---

### 5.2 Cross-Platform Testing 🖥️ [ ] — **L**
- [ ] 5.2.1 Test install on macOS (local MacBook): `pip install -e .`
- [ ] 5.2.2 Test install on Linux (MCPtest server): `pip install -e .`
- [ ] 5.2.3 Update GitHub Actions matrix: [ubuntu, macos, windows] × [py3.9, py3.11]
- [ ] 5.2.4 Run full test suite on each platform in CI
- [ ] 5.2.5 Fix any platform-specific issues
- [ ] 5.2.6 Test wheel install (not just editable): `pip install dist/*.whl`

**Acceptance Criteria:**
- All CI matrix jobs pass
- Install succeeds on all 3 platforms
- All tests pass on all platforms

**Effort:** L (3 platforms × 2 Python versions = 6 test combinations)

---

### 5.3 PyPI Publishing 🖥️ [ ] — **M**
- [ ] 5.3.1 Register account on PyPI (pypi.org)
- [ ] 5.3.2 Create API token on PyPI
- [ ] 5.3.3 Configure `~/.pypirc` with token
- [ ] 5.3.4 Build distribution: `python -m build`
- [ ] 5.3.5 Upload to PyPI: `twine upload dist/*`
- [ ] 5.3.6 Verify: `pip install eYcel` from PyPI (in fresh venv)
- [ ] 5.3.7 Test installed version works: `eYcel --version`

**Acceptance Criteria:**
- Package visible on https://pypi.org/project/eYcel/
- `pip install eYcel` installs latest version
- Installed CLI works immediately

**Effort:** M (one-time setup + publishing)

---

### 5.4 pipx Support Verification 🖥️ [ ] — **S**
- [ ] 5.4.1 Install pipx if not present: `pip install pipx`
- [ ] 5.4.2 Install eYcel via pipx: `pipx install eYcel`
- [ ] 5.4.3 Verify: `eYcel --version` works in new shell
- [ ] 5.4.4 Verify isolation: pipx doesn't affect system Python
- [ ] 5.4.5 Add pipx instructions to README.md

**Acceptance Criteria:**
- `pipx install eYcel` succeeds
- CLI available globally without activating venv
- README documents pipx option

**Effort:** S (verification + docs)

---

### Phase 5 Deliverable
```bash
# Fresh environment
pip install eYcel
eYcel --version  # Shows v1.0.0

# Or with pipx
pipx install eYcel
eYcel encrypt --help
```

---

## Phase 6 — GUI Application (Future / Post v1.0)

**Goal:** Simple drag-and-drop GUI for non-technical users.

### 6.1 Streamlit Web GUI (Option A) 🖥️ [ ] — **L**
- [ ] 6.1.1 Create `app/streamlit_app.py`
- [ ] 6.1.2 Add file uploader widget for Excel files
- [ ] 6.1.3 Add column transformation selector (dropdown per column)
- [ ] 6.1.4 Add "Encrypt" button with progress indicator
- [ ] 6.1.5 Add download buttons for encrypted file + rules.yaml
- [ ] 6.1.6 Add "Decrypt" tab with file upload + rules upload
- [ ] 6.1.7 Add validation display for rules files
- [ ] 6.1.8 Add `requirements-app.txt` with streamlit
- [ ] 6.1.9 Write `app/README.md` with run instructions
- [ ] 6.1.10 Test locally: `streamlit run app/streamlit_app.py`

**Acceptance Criteria:**
- Web UI accessible at localhost:8501
- File upload works for .xlsx files
- Column transformations configurable via dropdowns
- Download produces valid encrypted file + rules
- All core functionality accessible via GUI

**Effort:** L (full web UI implementation)

---

### 6.2 Desktop GUI (Option B - Tkinter) 🖥️ [ ] — **L** (Alternative/Additional)
- [ ] 6.2.1 Create `app/tkinter_app.py`
- [ ] 6.2.2 Add file picker dialog for input files
- [ ] 6.2.3 Add table view for column selection and transform mapping
- [ ] 6.2.4 Add progress bar widget
- [ ] 6.2.5 Add status bar for messages
- [ ] 6.2.6 Add encrypt/decrypt action buttons
- [ ] 6.2.7 Add output directory picker
- [ ] 6.2.8 Package as standalone executable with PyInstaller
- [ ] 6.2.9 Test on Windows and macOS

**Acceptance Criteria:**
- Native window opens on double-click
- File picker works on both platforms
- Progress bar updates during processing
- Standalone .exe/.app runs without Python installed

**Effort:** L (desktop app + packaging)

---

### 6.3 GUI Documentation ☁️ [ ] — **S**
- [ ] 6.3.1 Add GUI screenshots to README.md
- [ ] 6.3.2 Write GUI usage guide in docs/gui.md
- [ ] 6.3.3 Add video/GIF demonstration

**Acceptance Criteria:**
- Users can follow docs to run GUI
- Visual examples show expected UI

**Effort:** S (documentation + media)

---

### Phase 6 Deliverable
```bash
# Streamlit
cd app && streamlit run streamlit_app.py
# Opens browser with GUI

# Or Tkinter
python app/tkinter_app.py
# Opens native window
```

---

## Summary: Effort by Phase

| Phase | Effort | Key Deliverable |
|-------|--------|-----------------|
| 1 — Foundation | M (4-6 hrs) | `pip install -e .` works |
| 2 — Core Engine | L (12-16 hrs) | All unit tests pass |
| 3 — Encrypt/Decrypt | L (10-14 hrs) | Round-trip integration works |
| 4 — CLI | M (6-8 hrs) | `eYcel` command works globally |
| 5 — Packaging | M (6-8 hrs) | `pip install eYcel` from PyPI works |
| 6 — GUI | L (12-16 hrs) | Non-technical users can use GUI |
| **Total** | **~50-68 hrs** | **v1.0.0 + GUI** |

---

## Version Milestones

| Version | Contents | Phase Completion |
|---------|----------|------------------|
| v0.1.0 | Core engine — column analyzer + transformations | Phase 2 |
| v0.2.0 | Full encrypt/decrypt pipeline | Phase 3 |
| v0.3.0 | CLI interface | Phase 4 |
| **v1.0.0** | **PyPI release** | **Phase 5** |
| v1.1.0 | Streamlit GUI | Phase 6A |
| v2.0.0 | Native desktop app | Phase 6B |

---

## Key Constraints (Never Violate)

1. **Formulas must be preserved exactly** — this is the core value proposition
2. **Rules file must contain zero original data** — security guarantee
3. **Memory < 50MB for 100k rows** — efficiency requirement
4. **No pandas** — too heavy for this use case
5. **Python 3.9+ only** — modern but widely available
6. **Works offline** — no cloud dependency
7. **Test before commit** — every phase must pass tests before GitHub push

---

## Quick Reference: Running Tests

```bash
# All tests
cd ~/GitHub/eYcel && pytest tests/ -v

# Specific module
pytest tests/test_transformations.py -v

# With coverage
pytest tests/ --cov=src/eYcel --cov-report=html

# Memory profiling
python -m memory_profiler examples/process_sample.py
```

---

*Plan version: 2.0 | Last updated: 2024-01-15*
