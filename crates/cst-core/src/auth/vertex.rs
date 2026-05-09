//! Google Vertex AI authentication — inject Vertex env vars.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::EnvMap;

/// Vertex AI configuration stored in `auth/vertex.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VertexConfig {
    pub project_id: String,
    pub region: String,
    /// Path to service account JSON file.
    #[serde(default)]
    pub credentials_file: Option<String>,
}

impl VertexConfig {
    /// Produce the env vars needed to use Claude Code with Vertex AI.
    pub fn env_vars(&self) -> Result<EnvMap> {
        let mut map = EnvMap::new();
        map.insert("CLAUDE_CODE_USE_VERTEX".to_string(), "1".to_string());
        map.insert(
            "ANTHROPIC_VERTEX_PROJECT_ID".to_string(),
            self.project_id.clone(),
        );
        map.insert("CLOUD_ML_REGION".to_string(), self.region.clone());
        if let Some(ref creds) = self.credentials_file {
            map.insert("GOOGLE_APPLICATION_CREDENTIALS".to_string(), creds.clone());
        }
        Ok(map)
    }

    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(!self.project_id.is_empty(), "Vertex project ID must be set");
        anyhow::ensure!(!self.region.is_empty(), "Vertex region must be set");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_env_vars_sets_required_keys() {
        let cfg = VertexConfig {
            project_id: "my-project".into(),
            region: "us-central1".into(),
            credentials_file: None,
        };
        let vars = cfg.env_vars().unwrap();
        assert_eq!(vars["CLAUDE_CODE_USE_VERTEX"], "1");
        assert_eq!(vars["ANTHROPIC_VERTEX_PROJECT_ID"], "my-project");
        assert_eq!(vars["CLOUD_ML_REGION"], "us-central1");
        assert!(!vars.contains_key("GOOGLE_APPLICATION_CREDENTIALS"));
    }

    #[test]
    fn test_vertex_env_vars_with_credentials_file() {
        let cfg = VertexConfig {
            project_id: "proj".into(),
            region: "us-east1".into(),
            credentials_file: Some("/path/to/creds.json".into()),
        };
        let vars = cfg.env_vars().unwrap();
        assert_eq!(
            vars["GOOGLE_APPLICATION_CREDENTIALS"],
            "/path/to/creds.json"
        );
    }

    #[test]
    fn test_validate_fails_when_empty() {
        let cfg = VertexConfig::default();
        assert!(cfg.validate().is_err());
    }
}
