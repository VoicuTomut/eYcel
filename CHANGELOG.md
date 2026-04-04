# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- GUI interface (Phase 5)
- Additional transformation types
- Batch processing mode
- Performance optimizations for very large files

---

## [0.2.0] - 2024-04-04

### Phase 2: Core Engine Complete

### Added
- **Column Analyzer** (`column_analyzer.py`)
  - Automatic type detection for dates, integers, floats, strings, booleans
  - Formula cell detection and preservation
  - Categorical data detection with configurable thresholds
  - Column statistics (min, max, mean, null count, etc.)

- **Transformations** (`transformations.py`)
  - `hash()` — Deterministic string hashing (one-way)
  - `offset_date()` — Date shifting with full reversibility
  - `offset_number()` — Numeric offset with full reversibility
  - `scale()` — Multiplicative scaling with full reversibility
  - `shuffle()` — Categorical value shuffling with mapping
  - `keep()` — Pass-through for unchanged columns
  - `anonymize()` — Auto-select transform based on data type

- **Formula Handler** (`formula_handler.py`)
  - Extract formulas before transformation
  - Clear formula cells before data processing
  - Re-inject formulas after transformation
  - Verify formula preservation integrity

- **YAML Handler** (`yaml_handler.py`)
  - Generate rules files with metadata
  - Save and load rules with validation
  - Sanitize rules to ensure no original data leakage
  - Support for all transformation types

- **Encryption Pipeline** (`encrypt.py`)
  - Full end-to-end encryption workflow
  - Automatic transformation selection
  - Formula preservation throughout process
  - Progress reporting support

- **Decryption Pipeline** (`decrypt.py`)
  - Full end-to-end decryption workflow
  - Reverse all transformation types
  - Formula restoration verification

- **Test Suite** (119 tests, 80%+ coverage)
  - Unit tests for all transformation functions
  - Unit tests for column analysis
  - Unit tests for formula handling
  - Unit tests for YAML operations
  - Unit tests for encrypt/decrypt pipelines
  - Integration tests for full round-trip
  - CLI command tests

### Technical Details
- Memory-efficient processing using generators
- Type hints throughout codebase
- Comprehensive docstrings
- Error handling with custom exceptions
- Deterministic transformations using seeded random

---

## [0.1.0] - 2024-04-03

### Phase 1: Foundation

### Added
- **Project Structure**
  - Repository setup with GitHub
  - Python package structure (`src/eYcel/`)
  - Development environment configuration
  - Initial CI/CD pipeline

- **Build System**
  - `pyproject.toml` configuration
  - `setup.py` for backward compatibility
  - `MANIFEST.in` for package distribution
  - `requirements.txt` for dependencies

- **Documentation**
  - `README.md` with basic project info
  - `LICENSE` (MIT)
  - `CONTRIBUTING.md` guidelines
  - `.gitignore` for Python projects

- **Development Tools**
  - `pytest` configuration
  - `flake8` linting setup
  - `pytest-cov` coverage reporting
  - `memory-profiler` for performance monitoring

- **CI/CD**
  - GitHub Actions workflow skeleton
  - Automated testing on push/PR

### Project Kickoff
- Initial commit with project scaffolding
- Package installable via `pip install -e .`
- Basic CLI structure defined

---

## Version Key

- **MAJOR**: Breaking changes to API or file formats
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

---

## Migration Guides

### Upgrading to 0.2.0

No migration needed — this is the first functional release. Projects using 0.1.0 (scaffolding only) should reinstall:

```bash
pip install --upgrade eYcel
```

---

*For detailed commit history, see [GitHub commits](https://github.com/VoicuTomut/eYcel/commits/main).*
