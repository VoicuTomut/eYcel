# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- PyPI publication and package distribution
- Cross-platform CI testing (Windows, macOS, Linux)
- Improved test coverage to ≥80%
- Development tooling (flake8, twine) installation
- Documentation updates and example improvements

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