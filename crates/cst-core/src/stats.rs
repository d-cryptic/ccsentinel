//! Usage statistics per session.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionStats {
    pub session_count: u64,
    pub rate_limit_hits: u64,
    pub key_rotations: u64,
    pub auto_switches_out: u64,
    pub tokens_in: u64,
    pub tokens_out: u64,
    pub first_used: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
    /// Estimated API cost in USD (for api/bedrock/vertex profiles).
    #[serde(default)]
    pub estimated_cost_usd: f64,
}

impl SessionStats {
    pub fn load(session_dir: &Path) -> Result<Self> {
        let path = session_dir.join("stats.json");
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn save(&self, session_dir: &Path) -> Result<()> {
        let path = session_dir.join("stats.json");
        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn record_session_start(&mut self) {
        self.session_count += 1;
        let now = Utc::now();
        if self.first_used.is_none() {
            self.first_used = Some(now);
        }
        self.last_used = Some(now);
    }

    pub fn record_rate_limit(&mut self) {
        self.rate_limit_hits += 1;
    }

    pub fn record_key_rotation(&mut self) {
        self.key_rotations += 1;
    }

    /// Add token counts and estimate cost based on model pricing.
    /// Uses a rough estimate for Sonnet 4.5: $3/M input, $15/M output.
    pub fn add_tokens(&mut self, tokens_in: u64, tokens_out: u64) {
        self.tokens_in += tokens_in;
        self.tokens_out += tokens_out;
        // Rough cost estimate
        self.estimated_cost_usd +=
            (tokens_in as f64 / 1_000_000.0) * 3.0 + (tokens_out as f64 / 1_000_000.0) * 15.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_stats_save_and_load_roundtrip() {
        let dir = TempDir::new().unwrap();
        let mut stats = SessionStats::default();
        stats.record_session_start();
        stats.record_rate_limit();
        stats.save(dir.path()).unwrap();

        let loaded = SessionStats::load(dir.path()).unwrap();
        assert_eq!(loaded.session_count, 1);
        assert_eq!(loaded.rate_limit_hits, 1);
    }

    #[test]
    fn test_add_tokens_updates_cost() {
        let mut stats = SessionStats::default();
        stats.add_tokens(1_000_000, 100_000);
        // 1M input × $3 + 100k output × $15/M = $3 + $1.50 = $4.50
        assert!((stats.estimated_cost_usd - 4.50).abs() < 0.01);
    }

    #[test]
    fn test_load_nonexistent_returns_default() {
        let dir = TempDir::new().unwrap();
        let stats = SessionStats::load(dir.path()).unwrap();
        assert_eq!(stats.session_count, 0);
    }
}
