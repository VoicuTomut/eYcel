"""
Tests for the transformations module.
All reversible transforms are tested for round-trip correctness.
"""
import pytest
from datetime import date, timedelta

from src.eYcel.transformations import (
    transform_hash,
    transform_offset_date,
    transform_offset_number,
    transform_scale,
    transform_shuffle,
    transform_keep,
    transform_anonymize,
    reverse_offset_date,
    reverse_offset_number,
    reverse_scale,
    reverse_shuffle,
)


# ---------------------------------------------------------------------------
# Hash
# ---------------------------------------------------------------------------

class TestHash:
    def test_returns_string(self):
        result = transform_hash("hello", "salt123")
        assert isinstance(result, str)

    def test_length_12(self):
        result = transform_hash("hello", "salt123")
        assert len(result) == 12

    def test_deterministic(self):
        a = transform_hash("value", "mysalt")
        b = transform_hash("value", "mysalt")
        assert a == b

    def test_different_salts_differ(self):
        a = transform_hash("value", "salt1")
        b = transform_hash("value", "salt2")
        assert a != b

    def test_different_values_differ(self):
        a = transform_hash("val1", "salt")
        b = transform_hash("val2", "salt")
        assert a != b


# ---------------------------------------------------------------------------
# Offset (dates)
# ---------------------------------------------------------------------------

class TestOffsetDate:
    def test_forward(self):
        d = date(2023, 1, 1)
        shifted = transform_offset_date(d, 10)
        assert shifted == date(2023, 1, 11)

    def test_negative_offset(self):
        d = date(2023, 6, 15)
        shifted = transform_offset_date(d, -15)
        assert shifted == date(2023, 5, 31)

    def test_round_trip(self):
        original = date(2022, 8, 20)
        offset = 45
        encrypted = transform_offset_date(original, offset)
        restored = reverse_offset_date(encrypted, offset)
        assert restored == original

    def test_zero_offset(self):
        d = date(2020, 12, 31)
        assert transform_offset_date(d, 0) == d


# ---------------------------------------------------------------------------
# Offset (numbers)
# ---------------------------------------------------------------------------

class TestOffsetNumber:
    def test_forward(self):
        assert transform_offset_number(100.0, 50.0) == 150.0

    def test_negative(self):
        assert transform_offset_number(100.0, -30.0) == 70.0

    def test_round_trip(self):
        original = 987.65
        offset = -123.45
        encrypted = transform_offset_number(original, offset)
        restored = reverse_offset_number(encrypted, offset)
        assert abs(restored - original) < 1e-9

    def test_integer_input(self):
        result = transform_offset_number(5, 3)
        assert result == 8.0


# ---------------------------------------------------------------------------
# Scale
# ---------------------------------------------------------------------------

class TestScale:
    def test_forward(self):
        assert transform_scale(200.0, 0.5) == 100.0

    def test_round_trip(self):
        original = 333.33
        factor = 0.73
        encrypted = transform_scale(original, factor)
        restored = reverse_scale(encrypted, factor)
        assert abs(restored - original) < 1e-9

    def test_zero_factor_raises(self):
        with pytest.raises(ValueError):
            transform_scale(100.0, 0)

    def test_reverse_zero_factor_raises(self):
        with pytest.raises(ValueError):
            reverse_scale(100.0, 0)

    def test_factor_greater_than_one(self):
        assert transform_scale(10.0, 2.0) == 20.0


# ---------------------------------------------------------------------------
# Shuffle
# ---------------------------------------------------------------------------

class TestShuffle:
    MAPPING = {"Electronics": "Cat_0", "Clothing": "Cat_1", "Food": "Cat_2"}

    def test_known_value(self):
        assert transform_shuffle("Electronics", self.MAPPING) == "Cat_0"

    def test_unknown_value_passthrough(self):
        assert transform_shuffle("Unknown", self.MAPPING) == "Unknown"

    def test_round_trip(self):
        original = "Clothing"
        encrypted = transform_shuffle(original, self.MAPPING)
        restored = reverse_shuffle(encrypted, self.MAPPING)
        assert restored == original

    def test_full_mapping_reversible(self):
        for original, label in self.MAPPING.items():
            assert reverse_shuffle(label, self.MAPPING) == original


# ---------------------------------------------------------------------------
# Keep
# ---------------------------------------------------------------------------

class TestKeep:
    def test_string(self):
        assert transform_keep("hello") == "hello"

    def test_none(self):
        assert transform_keep(None) is None

    def test_number(self):
        assert transform_keep(42) == 42


# ---------------------------------------------------------------------------
# Anonymize
# ---------------------------------------------------------------------------

class TestAnonymize:
    def test_int_type(self):
        result = transform_anonymize(100, "int", _seed=42)
        assert isinstance(result, int)

    def test_float_type(self):
        result = transform_anonymize(3.14, "float", _seed=42)
        assert isinstance(result, float)

    def test_date_type(self):
        result = transform_anonymize(date(2020, 1, 1), "date", _seed=42)
        assert hasattr(result, "year")

    def test_string_type(self):
        result = transform_anonymize("hello", "string", _seed=42)
        assert isinstance(result, str)
        assert len(result) >= 4

    def test_seeded_deterministic(self):
        a = transform_anonymize("x", "string", _seed=99)
        b = transform_anonymize("x", "string", _seed=99)
        assert a == b
