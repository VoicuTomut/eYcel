use chrono::{NaiveDate, NaiveDateTime, TimeDelta};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate::cell_value::CellValue;

// ---------------------------------------------------------------------------
// Forward transforms (encrypt direction)
// ---------------------------------------------------------------------------

/// One-way SHA-256 hash with salt prefix. Returns 12-char hex digest.
pub fn transform_hash(value: &str, salt: &str) -> String {
    let raw = format!("{}:{}", salt, value);
    let hash = Sha256::digest(raw.as_bytes());
    format!("{:x}", hash)[..12].to_string()
}

/// Shift a NaiveDate by a fixed number of days.
pub fn transform_offset_date(date_val: NaiveDate, offset_days: i32) -> NaiveDate {
    date_val + TimeDelta::days(offset_days as i64)
}

/// Shift a NaiveDateTime by a fixed number of days.
pub fn transform_offset_datetime(dt: NaiveDateTime, offset_days: i32) -> NaiveDateTime {
    dt + TimeDelta::days(offset_days as i64)
}

/// Add a fixed offset to a numeric value.
pub fn transform_offset_number(num_val: f64, offset: f64) -> f64 {
    num_val + offset
}

/// Multiply a numeric value by a factor.
pub fn transform_scale(value: f64, factor: f64) -> Result<f64, &'static str> {
    if factor == 0.0 {
        return Err("Scale factor must not be zero.");
    }
    Ok(value * factor)
}

/// Rename a category using a mapping dict. Returns original if not found.
pub fn transform_shuffle(value: &str, mapping: &HashMap<String, String>) -> String {
    mapping.get(value).cloned().unwrap_or_else(|| value.to_string())
}

/// Pass-through — return value unchanged.
pub fn transform_keep(value: CellValue) -> CellValue {
    value
}

/// Replace value with a realistic fake of the same type. Irreversible.
pub fn transform_anonymize(value: &CellValue, col_type: &str, rng: &mut impl Rng) -> CellValue {
    match col_type {
        "int" => CellValue::Int(rng.gen_range(1..=99999)),
        "float" => {
            let v: f64 = rng.gen_range(0.01..99999.99);
            CellValue::Float((v * 100.0).round() / 100.0)
        }
        "date" => {
            let base = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
            let days = rng.gen_range(0..9000);
            CellValue::Date(base + TimeDelta::days(days))
        }
        "string" | "categorical" => {
            let length = match value {
                CellValue::String(s) => s.len().max(4),
                _ => 4,
            };
            let s: String = (0..length)
                .map(|_| (b'A' + rng.gen_range(0..26)) as char)
                .collect();
            CellValue::String(s)
        }
        _ => value.clone(),
    }
}

// ---------------------------------------------------------------------------
// Reverse transforms (decrypt direction)
// ---------------------------------------------------------------------------

/// Reverse a date offset by subtracting the original shift.
pub fn reverse_offset_date(encrypted_date: NaiveDate, offset_days: i32) -> NaiveDate {
    encrypted_date - TimeDelta::days(offset_days as i64)
}

/// Reverse a datetime offset.
pub fn reverse_offset_datetime(dt: NaiveDateTime, offset_days: i32) -> NaiveDateTime {
    dt - TimeDelta::days(offset_days as i64)
}

/// Reverse a number offset by subtracting.
pub fn reverse_offset_number(encrypted_val: f64, offset: f64) -> f64 {
    encrypted_val - offset
}

/// Reverse scaling by dividing.
pub fn reverse_scale(encrypted_val: f64, factor: f64) -> Result<f64, &'static str> {
    if factor == 0.0 {
        return Err("Scale factor must not be zero.");
    }
    Ok(encrypted_val / factor)
}

/// Reverse a shuffle by inverting the mapping.
pub fn reverse_shuffle(encrypted_val: &str, mapping: &HashMap<String, String>) -> String {
    let inverse: HashMap<&str, &str> = mapping.iter().map(|(k, v)| (v.as_str(), k.as_str())).collect();
    inverse
        .get(encrypted_val)
        .map(|s| s.to_string())
        .unwrap_or_else(|| encrypted_val.to_string())
}

// ---------------------------------------------------------------------------
// Global text substitution (consistent across entire file)
// ---------------------------------------------------------------------------

/// Fake name syllables for generating readable substitutes.
const SYLLABLES: &[&str] = &[
    "ba", "be", "bi", "bo", "bu", "da", "de", "di", "do", "du",
    "fa", "fe", "fi", "fo", "fu", "ga", "ge", "gi", "go", "gu",
    "ka", "ke", "ki", "ko", "ku", "la", "le", "li", "lo", "lu",
    "ma", "me", "mi", "mo", "mu", "na", "ne", "ni", "no", "nu",
    "pa", "pe", "pi", "po", "pu", "ra", "re", "ri", "ro", "ru",
    "sa", "se", "si", "so", "su", "ta", "te", "ti", "to", "tu",
    "va", "ve", "vi", "vo", "vu", "za", "ze", "zi", "zo", "zu",
];

/// Generate a readable fake word from an index.
/// Produces words like "Bako", "Delu", "Fimo", "Gape", etc.
pub fn fake_word_from_index(index: usize) -> String {
    let base = SYLLABLES.len(); // 70 syllables
    let s1 = index % base;
    let s2 = (index / base) % base;

    let mut word = String::new();
    // First syllable capitalized
    let first = SYLLABLES[s1];
    let mut chars = first.chars();
    if let Some(c) = chars.next() {
        word.push(c.to_uppercase().next().unwrap_or(c));
    }
    for c in chars {
        word.push(c);
    }
    // Second syllable
    word.push_str(SYLLABLES[s2]);

    // For very large indices, add a numeric suffix
    let tier = index / (base * base);
    if tier > 0 {
        word.push_str(&tier.to_string());
    }

    word
}

/// Build a global text substitution map from all unique text values.
/// Same text → same fake word everywhere in the file.
/// Returns (original→fake, fake→original) mappings.
pub fn build_global_text_map(
    unique_texts: &[String],
) -> HashMap<String, String> {
    let mut sorted = unique_texts.to_vec();
    sorted.sort();
    sorted.dedup();

    sorted
        .into_iter()
        .enumerate()
        .map(|(i, original)| (original, fake_word_from_index(i)))
        .collect()
}

// ---------------------------------------------------------------------------
// Random parameter generators (matching Python behavior)
// ---------------------------------------------------------------------------

/// Generate a random alphanumeric salt string.
pub fn random_salt(rng: &mut impl Rng, length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    (0..length)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

/// Return a random scale factor in [0.1, 0.9) ∪ (1.1, 2.0].
pub fn random_factor(rng: &mut impl Rng) -> f64 {
    loop {
        let f: f64 = rng.gen_range(0.1..2.0);
        if !(0.9..=1.1).contains(&f) {
            return (f * 1_000_000.0).round() / 1_000_000.0;
        }
    }
}

/// Return a random numeric offset in [-1000, -100] ∪ [100, 1000].
pub fn random_offset(rng: &mut impl Rng) -> f64 {
    let v: f64 = rng.gen_range(100.0..1000.0);
    let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
    (v * sign * 10000.0).round() / 10000.0
}

/// Return a random day offset between ±365.
pub fn random_day_offset(rng: &mut impl Rng) -> i32 {
    rng.gen_range(-365..=365)
}

/// Build a deterministic shuffle mapping: sorted unique values → Cat_0, Cat_1, ...
pub fn build_shuffle_mapping(unique_values: &[String]) -> HashMap<String, String> {
    let mut sorted: Vec<String> = unique_values.to_vec();
    sorted.sort();
    sorted.dedup();
    sorted
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, format!("Cat_{}", i)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    #[test]
    fn test_hash() {
        let result = transform_hash("hello", "salt123");
        assert_eq!(result.len(), 12);
        // Same input + salt = same hash
        assert_eq!(result, transform_hash("hello", "salt123"));
        // Different salt = different hash
        assert_ne!(result, transform_hash("hello", "other"));
    }

    #[test]
    fn test_offset_date() {
        let d = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let shifted = transform_offset_date(d, 10);
        assert_eq!(shifted, NaiveDate::from_ymd_opt(2024, 6, 25).unwrap());
        let restored = reverse_offset_date(shifted, 10);
        assert_eq!(restored, d);
    }

    #[test]
    fn test_offset_number() {
        let v = transform_offset_number(100.0, 50.5);
        assert!((v - 150.5).abs() < 1e-9);
        let r = reverse_offset_number(v, 50.5);
        assert!((r - 100.0).abs() < 1e-9);
    }

    #[test]
    fn test_scale() {
        let v = transform_scale(100.0, 2.5).unwrap();
        assert!((v - 250.0).abs() < 1e-9);
        let r = reverse_scale(v, 2.5).unwrap();
        assert!((r - 100.0).abs() < 1e-9);
        assert!(transform_scale(100.0, 0.0).is_err());
    }

    #[test]
    fn test_shuffle() {
        let mut map = HashMap::new();
        map.insert("USA".to_string(), "Cat_0".to_string());
        map.insert("UK".to_string(), "Cat_1".to_string());
        assert_eq!(transform_shuffle("USA", &map), "Cat_0");
        assert_eq!(transform_shuffle("FR", &map), "FR");
        assert_eq!(reverse_shuffle("Cat_0", &map), "USA");
        assert_eq!(reverse_shuffle("Cat_99", &map), "Cat_99");
    }

    #[test]
    fn test_anonymize() {
        let mut rng = SmallRng::seed_from_u64(42);
        let v = transform_anonymize(&CellValue::Int(100), "int", &mut rng);
        matches!(v, CellValue::Int(_));

        let mut rng = SmallRng::seed_from_u64(42);
        let v = transform_anonymize(&CellValue::String("hello".into()), "string", &mut rng);
        if let CellValue::String(s) = v {
            assert_eq!(s.len(), 5);
            assert!(s.chars().all(|c| c.is_ascii_uppercase()));
        } else {
            panic!("Expected string");
        }
    }

    #[test]
    fn test_build_shuffle_mapping() {
        let vals = vec!["B".into(), "A".into(), "C".into(), "A".into()];
        let map = build_shuffle_mapping(&vals);
        assert_eq!(map.get("A").unwrap(), "Cat_0");
        assert_eq!(map.get("B").unwrap(), "Cat_1");
        assert_eq!(map.get("C").unwrap(), "Cat_2");
    }

    #[test]
    fn test_random_factor_range() {
        let mut rng = SmallRng::seed_from_u64(0);
        for _ in 0..100 {
            let f = random_factor(&mut rng);
            assert!(f >= 0.1 && f <= 2.0);
            assert!(!(0.9..=1.1).contains(&f));
        }
    }
}
