"""
YAML rules file generation, validation, and loading.

The rules file is the *only* artifact that lets you reverse the encryption.
It must contain transformation parameters (salts, offsets, factors, mappings)
but ZERO original data values.
"""
from typing import Any, Dict, List, Tuple
from pathlib import Path
from datetime import datetime, timezone
import yaml

# Valid transform names
VALID_TRANSFORMS = {"hash", "offset", "scale", "shuffle", "keep", "anonymize", "substitute"}


# ---------------------------------------------------------------------------
# Generation
# ---------------------------------------------------------------------------

def generate_rules(
    metadata: Dict[str, Any],
    columns: Dict[str, Dict[str, Any]],
) -> Dict[str, Any]:
    """
    Build the complete rules dictionary.

    Args:
        metadata: At minimum {'original_filename': str}.
                  A 'timestamp' key is added automatically if missing.
        columns:  Mapping of column_header → transform config dict.
                  Each config must include at least {'transform': <name>}.

    Returns:
        Rules dict ready to serialise as YAML.
    """
    # Defensive copy — never mutate the caller's dict
    meta_copy = dict(metadata)
    if "timestamp" not in meta_copy:
        meta_copy["timestamp"] = datetime.now(timezone.utc).isoformat()

    rules: Dict[str, Any] = {
        "metadata": meta_copy,
        "columns": {k: dict(v) for k, v in columns.items()},
    }
    return rules


# ---------------------------------------------------------------------------
# Persistence
# ---------------------------------------------------------------------------

def save_rules(rules_dict: Dict[str, Any], filepath: str) -> None:
    """
    Serialise *rules_dict* to a YAML file at *filepath*.

    Args:
        rules_dict: Rules dictionary (from generate_rules).
        filepath:   Destination path (created if it does not exist).

    Raises:
        OSError: If the file cannot be written.
    """
    path = Path(filepath)
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as fh:
        yaml.dump(rules_dict, fh, default_flow_style=False, allow_unicode=True, sort_keys=False)


def load_rules(filepath: str) -> Dict[str, Any]:
    """
    Load a rules YAML file and validate its structure.

    Args:
        filepath: Path to the rules YAML file.

    Returns:
        Validated rules dictionary.

    Raises:
        FileNotFoundError: If the file does not exist.
        ValueError:        If the file fails schema validation.
    """
    path = Path(filepath)
    if not path.exists():
        raise FileNotFoundError(f"Rules file not found: {filepath}")

    with path.open("r", encoding="utf-8") as fh:
        data = yaml.safe_load(fh)

    ok, errors = validate_rules(data)
    if not ok:
        raise ValueError(f"Invalid rules file: {'; '.join(errors)}")

    return data


# ---------------------------------------------------------------------------
# Validation
# ---------------------------------------------------------------------------

def validate_rules(rules_dict: Any) -> Tuple[bool, List[str]]:
    """
    Validate the top-level structure of a rules dictionary.

    Args:
        rules_dict: Object loaded from YAML (may be anything).

    Returns:
        (True, [])           if valid.
        (False, [error, …])  if invalid, listing all problems found.
    """
    errors: List[str] = []

    if not isinstance(rules_dict, dict):
        return False, ["Rules must be a YAML mapping at the top level."]

    # Required top-level keys
    for key in ("metadata", "columns"):
        if key not in rules_dict:
            errors.append(f"Missing required top-level key: '{key}'")

    if errors:
        return False, errors

    # Metadata checks
    meta = rules_dict["metadata"]
    if not isinstance(meta, dict):
        errors.append("'metadata' must be a mapping.")
    elif "original_filename" not in meta:
        errors.append("'metadata.original_filename' is required.")

    # Columns checks
    cols = rules_dict["columns"]
    if not isinstance(cols, dict):
        errors.append("'columns' must be a mapping.")
    else:
        for col_name, cfg in cols.items():
            if not isinstance(cfg, dict):
                errors.append(f"Column '{col_name}': config must be a mapping.")
                continue
            transform = cfg.get("transform")
            if transform is None:
                errors.append(f"Column '{col_name}': missing 'transform' key.")
            elif transform not in VALID_TRANSFORMS:
                errors.append(
                    f"Column '{col_name}': unknown transform '{transform}'. "
                    f"Valid: {sorted(VALID_TRANSFORMS)}"
                )

    return (len(errors) == 0), errors


# ---------------------------------------------------------------------------
# Sanitisation
# ---------------------------------------------------------------------------

def sanitize_rules(rules_dict: Dict[str, Any]) -> Dict[str, Any]:
    """
    Strip any key that looks like it might contain original data values.

    Scans each column config and removes keys named 'original_values',
    'sample_data', or 'raw_*'.  The check is intentionally conservative —
    legitimate param keys (salt, factor, offset_days, mapping) are safe.

    Args:
        rules_dict: Rules dictionary to sanitise (modified in place).

    Returns:
        The sanitised rules dictionary.
    """
    _FORBIDDEN_KEYS = {"original_values", "sample_data", "raw_data"}

    cols = rules_dict.get("columns", {})
    for col_cfg in cols.values():
        if isinstance(col_cfg, dict):
            for key in list(col_cfg.keys()):
                if key in _FORBIDDEN_KEYS or key.startswith("raw_"):
                    del col_cfg[key]
    return rules_dict
