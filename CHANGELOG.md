# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- PyPI publication and package distribution
- Enhanced GUI features (batch processing, custom rules editor)
- Support for Excel macros (VBA) preservation
- Performance optimizations for very large files

## [0.2.0] - 2026-04-05

### Added
- **Comprehensive CLI test coverage**: 100% coverage of `cli.py` module
- **Exception module tests**: Full coverage of `exceptions.py`
- **Enhanced memory utilities tests**: Additional edge‑case coverage for `memory_utils.py`

### Changed
- **Version bump**: Updated `__version__` to 0.2.0
- **Code quality**: Applied flake8 linting with max‑line‑length=100
- **Removed unused imports** (`os`, `Optional`, `column_index_from_string`, `get_column_letter`, `validate_rules`)
- **Fixed line‑length violations** in `column_analyzer.py`, `encrypt.py`, `transformations.py`
- **Eliminated unused variables** (`max_col`, `unique_vals`)

### Fixed
- **Import consistency**: Restored missing `column_index_from_string` import in `encrypt.py`
- **Whitespace cleanup**: Trailing whitespace in split line

### Metrics
- **Test count**: 196 passed (up from 154)
- **Coverage**: 92% (up from 71%)
- **Linting**: Zero flake8 violations

## [0.1.0] - 2026-04-04

### Added
- **CLI**: Full command‑line interface with `encrypt`, `decrypt`, and `validate` subcommands.
- **Core engine**: Six reversible and one‑way transformations:
  - `hash` – one‑way hashing of string values.
  - `offset` – reversible numeric/date offset.
  - `scale` – reversible numeric scaling.
  - `shuffle` – reversible category shuffling.
  - `keep` – leave values unchanged.
  - `anonymize` – pattern‑based anonymization.
- **Formula preservation**: All Excel formulas are extracted before transformation and restored exactly afterward.
- **Automatic column detection**: Analyzes each column and suggests the most appropriate transformation.
- **Streamlit GUI**: Web interface with drag‑and‑drop upload, file preview, column analysis, and one‑click encrypt/decrypt.
- **YAML rules file**: Human‑readable configuration that stores all transformation parameters for later decryption.
- **Validation**: Standalone validation of rules files.
- **Comprehensive test suite**: 154 passing unit tests covering all modules.
- **Examples**: Fully‑worked example files (`simple.xlsx`, `encrypted.xlsx`, `encrypted_rules.yaml`).
- **GitHub Actions CI**: Automated testing on push.

### Changed
- *Initial release – no previous changes.*

### Fixed
- *Initial release – no previous fixes.*

---

## Version Key

- **MAJOR**: Breaking changes to API or file formats
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

---

## Migration Guides

### Upgrading to 0.1.0

No migration needed — this is the first functional release.

---

*For detailed commit history, see [GitHub commits](https://github.com/VoicuTomut/eYcel/commits/main).*