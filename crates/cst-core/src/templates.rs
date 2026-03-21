//! Built-in profile templates — pre-configured settings-override.json content.

use crate::profile::AuthType;
use serde_json::{json, Value};

pub struct Template {
    pub name: &'static str,
    pub description: &'static str,
    pub auth_type: AuthType,
    pub settings_override: Value,
}

/// All built-in templates.
pub fn all() -> Vec<Template> {
    vec![
        Template {
            name: "pro",
            description: "Claude Pro subscription — sandbox on, standard config",
            auth_type: AuthType::OAuth,
            settings_override: json!({
                "sandbox": { "enabled": true }
            }),
        },
        Template {
            name: "max",
            description: "Claude Max subscription — bypass permissions, all MCPs",
            auth_type: AuthType::OAuth,
            settings_override: json!({
                "permissions": { "defaultMode": "bypassPermissions" },
                "alwaysThinkingEnabled": true
            }),
        },
        Template {
            name: "api",
            description: "API key — minimal config, no MCPs",
            auth_type: AuthType::Api,
            settings_override: json!({
                "permissions": { "defaultMode": "default" }
            }),
        },
        Template {
            name: "bedrock",
            description: "AWS Bedrock — no MCPs, standard sandbox",
            auth_type: AuthType::Bedrock,
            settings_override: json!({
                "sandbox": { "enabled": true }
            }),
        },
        Template {
            name: "vertex",
            description: "Google Vertex AI",
            auth_type: AuthType::Vertex,
            settings_override: json!({}),
        },
    ]
}

/// Find a template by name.
pub fn find(name: &str) -> Option<Template> {
    all().into_iter().find(|t| t.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_templates_have_names() {
        let templates = all();
        assert!(!templates.is_empty());
        for t in &templates {
            assert!(!t.name.is_empty());
        }
    }

    #[test]
    fn test_find_existing_template() {
        assert!(find("max").is_some());
        assert!(find("api").is_some());
    }

    #[test]
    fn test_find_nonexistent_template() {
        assert!(find("nonexistent").is_none());
    }
}
