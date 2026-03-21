//! MCP server override management — add/disable per-profile MCPs.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

/// Per-profile MCP overrides stored in `sessions/{s}/mcp-override.json`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpOverride {
    /// MCP server names to disable from the global `~/.claude.json`.
    #[serde(default)]
    pub disable: Vec<String>,
    /// Additional MCP servers to add (merged into the active config).
    #[serde(default)]
    pub add: HashMap<String, Value>,
}

impl McpOverride {
    pub fn load(session_dir: &Path) -> Result<Self> {
        let path = session_dir.join("mcp-override.json");
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn save(&self, session_dir: &Path) -> Result<()> {
        let path = session_dir.join("mcp-override.json");
        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    /// Apply this override to the global MCPs section of `~/.claude.json`.
    /// Returns the merged MCPs map.
    pub fn apply(&self, global_mcps: &HashMap<String, Value>) -> HashMap<String, Value> {
        let mut result = global_mcps.clone();
        // Remove disabled servers
        for name in &self.disable {
            result.remove(name);
        }
        // Add new servers
        for (name, config) in &self.add {
            result.insert(name.clone(), config.clone());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    fn sample_global() -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("github".to_string(), json!({"command": "npx"}));
        m.insert("postman".to_string(), json!({"command": "npx"}));
        m
    }

    #[test]
    fn test_apply_disables_server() {
        let ov = McpOverride {
            disable: vec!["postman".to_string()],
            add: HashMap::new(),
        };
        let result = ov.apply(&sample_global());
        assert!(!result.contains_key("postman"));
        assert!(result.contains_key("github"));
    }

    #[test]
    fn test_apply_adds_server() {
        let mut add = HashMap::new();
        add.insert("custom-mcp".to_string(), json!({"command": "node"}));
        let ov = McpOverride { disable: vec![], add };
        let result = ov.apply(&sample_global());
        assert!(result.contains_key("custom-mcp"));
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = TempDir::new().unwrap();
        let ov = McpOverride {
            disable: vec!["postman".to_string()],
            add: HashMap::new(),
        };
        ov.save(dir.path()).unwrap();
        let loaded = McpOverride::load(dir.path()).unwrap();
        assert_eq!(loaded.disable, vec!["postman"]);
    }
}
