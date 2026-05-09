//! Per-session environment variable overlay — `env.toml`.
//!
//! Users can place extra env vars in `sessions/{s}/env.toml`:
//! ```toml
//! [env]
//! CLAUDE_CODE_MAX_OUTPUT_TOKENS = "32000"
//! PROJECT_ENV = "staging"
//! ```

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::shell::is_valid_env_key;

/// Contents of `env.toml` in a session directory.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvOverlay {
    /// Extra env vars to inject when this session is activated.
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl EnvOverlay {
    /// Load from `{session_dir}/env.toml`. Returns empty overlay if the file doesn't exist.
    ///
    /// Returns an error if any key is not a valid POSIX environment variable name
    /// (`[A-Za-z_][A-Za-z0-9_]*`), preventing shell injection via malformed key names.
    pub fn load(session_dir: &Path) -> Result<Self> {
        let path = session_dir.join("env.toml");
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(&path)?;
        let overlay: Self = toml::from_str(&contents)?;
        for key in overlay.env.keys() {
            if !is_valid_env_key(key) {
                bail!(
                    "env.toml contains invalid env var name {:?} — \
                     names must match [A-Za-z_][A-Za-z0-9_]*",
                    key
                );
            }
        }
        Ok(overlay)
    }

    /// Save to `{session_dir}/env.toml`.
    pub fn save(&self, session_dir: &Path) -> Result<()> {
        let path = session_dir.join("env.toml");
        std::fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    /// Return the env vars map, ready to merge into the activation exports.
    pub fn vars(&self) -> &HashMap<String, String> {
        &self.env
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_nonexistent_returns_empty() {
        let dir = TempDir::new().unwrap();
        let overlay = EnvOverlay::load(dir.path()).unwrap();
        assert!(overlay.env.is_empty());
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = TempDir::new().unwrap();
        let mut overlay = EnvOverlay::default();
        overlay
            .env
            .insert("PROJECT_ENV".to_string(), "staging".to_string());
        overlay.env.insert(
            "CLAUDE_CODE_MAX_OUTPUT_TOKENS".to_string(),
            "32000".to_string(),
        );
        overlay.save(dir.path()).unwrap();

        let loaded = EnvOverlay::load(dir.path()).unwrap();
        assert_eq!(
            loaded.env.get("PROJECT_ENV").map(String::as_str),
            Some("staging")
        );
        assert_eq!(
            loaded
                .env
                .get("CLAUDE_CODE_MAX_OUTPUT_TOKENS")
                .map(String::as_str),
            Some("32000")
        );
    }

    #[test]
    fn test_vars_returns_reference_to_map() {
        let mut overlay = EnvOverlay::default();
        overlay.env.insert("FOO".to_string(), "bar".to_string());
        assert_eq!(overlay.vars().get("FOO").map(String::as_str), Some("bar"));
    }

    #[test]
    fn test_load_rejects_invalid_key_names() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("env.toml");
        // A key that would allow shell injection
        std::fs::write(&path, "[env]\n\"FOO=bar;id\" = \"val\"\n").unwrap();
        let result = EnvOverlay::load(dir.path());
        assert!(result.is_err(), "should reject key with shell metacharacters");
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("invalid env var name"),
            "error message should mention invalid name: {msg}"
        );
    }

    #[test]
    fn test_load_accepts_valid_key_names() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("env.toml");
        std::fs::write(&path, "[env]\nCLAUDE_TOKEN = \"abc\"\n_PRIVATE = \"val\"\n").unwrap();
        let overlay = EnvOverlay::load(dir.path()).unwrap();
        assert_eq!(overlay.env.len(), 2);
    }
}
