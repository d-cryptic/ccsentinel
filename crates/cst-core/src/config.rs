//! Global runtime configuration — which profile:session is currently active.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::platform;

/// The global state stored in `~/.claude-sentinel/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Currently active profile name (empty = none).
    #[serde(default)]
    pub current_profile: String,
    /// Currently active session name (defaults to "default").
    #[serde(default = "default_session")]
    pub current_session: String,
}

fn default_session() -> String {
    "default".to_string()
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            current_profile: String::new(),
            current_session: default_session(),
        }
    }
}

impl GlobalConfig {
    /// Load config from disk, or return defaults if the file doesn't exist yet.
    pub fn load() -> Result<Self> {
        let path = platform::global_config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("reading config at {}", path.display()))?;
        toml::from_str(&contents).context("parsing config.toml")
    }

    /// Persist config to disk.
    pub fn save(&self) -> Result<()> {
        let path = platform::global_config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self).context("serializing config")?;
        std::fs::write(&path, contents)
            .with_context(|| format!("writing config at {}", path.display()))
    }

    /// Load from a specific path (used in tests).
    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(path)?;
        toml::from_str(&contents).context("parsing config")
    }

    /// Save to a specific path (used in tests).
    pub fn save_to(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Return `"profile:session"` string.
    pub fn current_ref(&self) -> String {
        if self.current_profile.is_empty() {
            String::new()
        } else {
            format!("{}:{}", self.current_profile, self.current_session)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_session_is_default() {
        let cfg = GlobalConfig::default();
        assert_eq!(cfg.current_session, "default");
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");

        let cfg = GlobalConfig {
            current_profile: "work".into(),
            current_session: "backend".into(),
        };
        cfg.save_to(&path).unwrap();

        let loaded = GlobalConfig::load_from(&path).unwrap();
        assert_eq!(loaded.current_profile, "work");
        assert_eq!(loaded.current_session, "backend");
    }

    #[test]
    fn test_load_nonexistent_returns_default() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("nonexistent.toml");
        let cfg = GlobalConfig::load_from(&path).unwrap();
        assert!(cfg.current_profile.is_empty());
    }

    #[test]
    fn test_current_ref_format() {
        let cfg = GlobalConfig {
            current_profile: "work".into(),
            current_session: "backend".into(),
        };
        assert_eq!(cfg.current_ref(), "work:backend");
    }

    #[test]
    fn test_current_ref_empty_when_no_profile() {
        let cfg = GlobalConfig::default();
        assert!(cfg.current_ref().is_empty());
    }
}
