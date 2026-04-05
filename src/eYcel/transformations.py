"""
Transformation functions for encrypting and decrypting Excel data.

Each forward transform has a matching reverse so that encrypt → decrypt
produces the original value.  The only *irreversible* transform is `hash`
(one-way by design) and `anonymize` (random replacement).
"""
import hashlib
import random
import string
from typing import Any, Dict, Optional, Union
from datetime import datetime, date, timedelta


# ---------------------------------------------------------------------------
# Forward transforms (encrypt direction)
# ---------------------------------------------------------------------------

def transform_hash(value: Any, salt: str) -> str:
    """
    One-way SHA-256 hash of a value with a salt prefix.

    Args:
        value: Value to hash (converted to string).
        salt:  Per-column random salt string.

    Returns:
        12-character hex digest prefix (safe, short, readable).
    """
    raw = f"{salt}:{value}"
    return hashlib.sha256(raw.encode("utf-8")).hexdigest()[:12]


def transform_offset_date(
    date_val: Union[datetime, date], offset_days: int
) -> Union[datetime, date]:
    """
    Shift a date/datetime by a fixed number of days.

    Args:
        date_val:    Original date or datetime.
        offset_days: Days to add (negative = subtract).

    Returns:
        Shifted date of the same type.
    """
    delta = timedelta(days=offset_days)
    return date_val + delta


def transform_offset_number(num_val: float, offset: float) -> float:
    """
    Add a fixed offset to a numeric value.

    Args:
        num_val: Original number.
        offset:  Value to add (can be negative).

    Returns:
        Shifted number (float).
    """
    return float(num_val) + float(offset)


def transform_scale(value: float, factor: float) -> float:
    """
    Multiply a numeric value by a secret factor.

    Args:
        value:  Original number.
        factor: Multiplication factor (must not be 0).

    Returns:
        Scaled number (float).

    Raises:
        ValueError: If factor is zero.
    """
    if factor == 0:
        raise ValueError("Scale factor must not be zero.")
    return float(value) * float(factor)


def transform_shuffle(value: str, mapping: Dict[str, str]) -> str:
    """
    Rename a category string using a pre-built mapping dictionary.

    Args:
        value:   Original category string.
        mapping: Dict mapping original → anonymised label.

    Returns:
        Anonymised label, or the original value if not found in mapping.
    """
    return mapping.get(str(value), str(value))


def transform_keep(value: Any) -> Any:
    """
    Pass-through — return value unchanged.

    Args:
        value: Any cell value.

    Returns:
        Identical value.
    """
    return value


def transform_anonymize(value: Any, col_type: str, _seed: Optional[int] = None) -> Any:
    """
    Replace a value with a realistic-looking fake of the same type.

    Args:
        value:    Original value (used only to determine length/shape).
        col_type: One of 'string', 'int', 'float', 'date', 'categorical'.
        _seed:    Optional RNG seed for deterministic tests.

    Returns:
        A plausible fake value of the same Python type.
    """
    rng = random.Random(_seed)
    if col_type == "int":
        return rng.randint(1, 99999)
    if col_type == "float":
        return round(rng.uniform(0.01, 99999.99), 2)
    if col_type == "date":
        base = date(2000, 1, 1)
        return base + timedelta(days=rng.randint(0, 9000))
    if col_type in ("string", "categorical"):
        length = max(4, len(str(value)))
        return "".join(rng.choices(string.ascii_uppercase, k=length))
    # fallback
    return value


# ---------------------------------------------------------------------------
# Reverse transforms (decrypt direction)
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# Consistent text substitution
# ---------------------------------------------------------------------------

SYLLABLES = [
    "ba", "be", "bi", "bo", "bu", "da", "de", "di", "do", "du",
    "fa", "fe", "fi", "fo", "fu", "ga", "ge", "gi", "go", "gu",
    "ka", "ke", "ki", "ko", "ku", "la", "le", "li", "lo", "lu",
    "ma", "me", "mi", "mo", "mu", "na", "ne", "ni", "no", "nu",
    "pa", "pe", "pi", "po", "pu", "ra", "re", "ri", "ro", "ru",
    "sa", "se", "si", "so", "su", "ta", "te", "ti", "to", "tu",
    "va", "ve", "vi", "vo", "vu", "za", "ze", "zi", "zo", "zu",
]


def fake_word_from_index(index: int) -> str:
    """Generate a readable fake word from an index."""
    base = len(SYLLABLES)
    s1 = index % base
    s2 = (index // base) % base
    word = SYLLABLES[s1].capitalize() + SYLLABLES[s2]
    tier = index // (base * base)
    if tier > 0:
        word += str(tier)
    return word


def build_global_text_map(unique_texts: list) -> Dict[str, str]:
    """Build a global text substitution map. Same text → same fake word."""
    sorted_texts = sorted(set(str(t) for t in unique_texts if t))
    return {orig: fake_word_from_index(i) for i, orig in enumerate(sorted_texts)}


def substitute_text_in_formula(formula: str, text_map: Dict[str, str]) -> str:
    """Replace text literals inside a formula using the text map."""
    result = []
    i = 0
    chars = list(formula)
    while i < len(chars):
        if chars[i] == '"':
            start = i + 1
            i += 1
            while i < len(chars) and chars[i] != '"':
                i += 1
            literal = "".join(chars[start:i])
            replacement = text_map.get(literal, literal)
            result.append('"')
            result.append(replacement)
            result.append('"')
            if i < len(chars):
                i += 1
        else:
            result.append(chars[i])
            i += 1
    return "".join(result)


def reverse_text_in_formula(formula: str, inverse_map: Dict[str, str]) -> str:
    """Reverse text substitutions inside a formula."""
    return substitute_text_in_formula(formula, inverse_map)


# ---------------------------------------------------------------------------
# Reverse transforms (decrypt direction)
# ---------------------------------------------------------------------------

def reverse_offset_date(
    encrypted_date: Union[datetime, date], offset_days: int
) -> Union[datetime, date]:
    """
    Reverse a date offset by subtracting the original shift.

    Args:
        encrypted_date: The shifted date.
        offset_days:    The same offset_days used during encryption.

    Returns:
        Original date.
    """
    return encrypted_date - timedelta(days=offset_days)


def reverse_offset_number(encrypted_val: float, offset: float) -> float:
    """
    Reverse a number offset by subtracting the original shift.

    Args:
        encrypted_val: The shifted number.
        offset:        The same offset used during encryption.

    Returns:
        Original number (float).
    """
    return float(encrypted_val) - float(offset)


def reverse_scale(encrypted_val: float, factor: float) -> float:
    """
    Reverse scaling by dividing.

    Args:
        encrypted_val: The scaled number.
        factor:        The same factor used during encryption.

    Returns:
        Original number (float).

    Raises:
        ValueError: If factor is zero.
    """
    if factor == 0:
        raise ValueError("Scale factor must not be zero.")
    return float(encrypted_val) / float(factor)


def reverse_shuffle(encrypted_val: str, mapping: Dict[str, str]) -> str:
    """
    Reverse a shuffle by inverting the mapping.

    Args:
        encrypted_val: The anonymised category label.
        mapping:       The original original → anonymised mapping.

    Returns:
        Original category string, or the encrypted value if not found.
    """
    inverse = {v: k for k, v in mapping.items()}
    return inverse.get(str(encrypted_val), str(encrypted_val))
