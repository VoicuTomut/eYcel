use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Valid transform names (matching Python VALID_TRANSFORMS).
pub const VALID_TRANSFORMS: &[&str] = &["hash", "offset", "scale", "shuffle", "keep", "anonymize", "substitute"];

/// Top-level rules structure, serialized as YAML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules {
    pub metadata: Metadata,
    pub columns: HashMap<String, ColumnConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub original_filename: String,
    #[serde(default)]
    pub timestamp: String,
    #[serde(default)]
    pub sheets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub transform: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col_type: Option<String>,
}

/// Validate a rules structure. Returns list of errors (empty = valid).
pub fn validate_rules(rules: &Rules) -> Vec<String> {
    let mut errors = Vec::new();

    if rules.metadata.original_filename.is_empty() {
        errors.push("'metadata.original_filename' is required.".to_string());
    }

    for (col_name, cfg) in &rules.columns {
        if cfg.transform.is_empty() {
            errors.push(format!("Column '{}': missing 'transform' key.", col_name));
        } else if !VALID_TRANSFORMS.contains(&cfg.transform.as_str()) {
            errors.push(format!(
                "Column '{}': unknown transform '{}'. Valid: {:?}",
                col_name, cfg.transform, VALID_TRANSFORMS
            ));
        }
    }

    errors
}

/// Serialize rules to YAML string.
pub fn rules_to_yaml(rules: &Rules) -> Result<String, String> {
    serde_yaml::to_string(rules).map_err(|e| format!("YAML serialization error: {}", e))
}

/// Deserialize rules from YAML string.
pub fn rules_from_yaml(yaml_str: &str) -> Result<Rules, String> {
    let rules: Rules =
        serde_yaml::from_str(yaml_str).map_err(|e| format!("YAML parse error: {}", e))?;

    let errors = validate_rules(&rules);
    if !errors.is_empty() {
        return Err(format!("Invalid rules: {}", errors.join("; ")));
    }

    Ok(rules)
}

/// Strip keys that might contain original data values.
pub fn sanitize_rules(_rules: &mut Rules) {
    // In the Rust version, the struct is typed so no unexpected keys can exist.
    // This is a no-op but kept for API compatibility.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rules() -> Rules {
        let mut columns = HashMap::new();
        columns.insert(
            "Name".to_string(),
            ColumnConfig {
                transform: "hash".to_string(),
                salt: Some("abc123".to_string()),
                factor: None,
                offset: None,
                offset_days: None,
                mapping: None,
                col_type: None,
            },
        );
        columns.insert(
            "Amount".to_string(),
            ColumnConfig {
                transform: "scale".to_string(),
                salt: None,
                factor: Some(1.5),
                offset: None,
                offset_days: None,
                mapping: None,
                col_type: None,
            },
        );
        Rules {
            metadata: Metadata {
                original_filename: "test.xlsx".to_string(),
                timestamp: "2024-01-01T00:00:00Z".to_string(),
                sheets: vec!["Sheet1".to_string()],
            },
            columns,
        }
    }

    #[test]
    fn test_validate_valid() {
        let rules = sample_rules();
        assert!(validate_rules(&rules).is_empty());
    }

    #[test]
    fn test_validate_bad_transform() {
        let mut rules = sample_rules();
        rules
            .columns
            .get_mut("Name")
            .unwrap()
            .transform = "invalid".to_string();
        let errors = validate_rules(&rules);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_roundtrip_yaml() {
        let rules = sample_rules();
        let yaml = rules_to_yaml(&rules).unwrap();
        let parsed = rules_from_yaml(&yaml).unwrap();
        assert_eq!(parsed.metadata.original_filename, "test.xlsx");
        assert_eq!(parsed.columns.len(), 2);
    }
}
