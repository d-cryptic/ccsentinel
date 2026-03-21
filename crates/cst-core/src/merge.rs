//! 3-layer settings.json deep merge.
//!
//! Layer order (later wins on conflict):
//! 1. Global `~/.claude/settings.json`
//! 2. Profile `settings-override.json`
//! 3. Session `settings-override.json`

use anyhow::{Context, Result};
use serde_json::Value;
use std::path::Path;

/// Deep-merge `src` into `dst`. Arrays are replaced (not concatenated).
/// Objects are recursively merged key-by-key.
pub fn deep_merge(dst: &mut Value, src: &Value) {
    match (dst, src) {
        (Value::Object(dst_map), Value::Object(src_map)) => {
            for (key, src_val) in src_map {
                let dst_val = dst_map.entry(key.clone()).or_insert(Value::Null);
                deep_merge(dst_val, src_val);
            }
        }
        (dst, src) => {
            *dst = src.clone();
        }
    }
}

/// Load JSON from a file, returning an empty object if the file doesn't exist.
pub fn load_json(path: &Path) -> Result<Value> {
    if !path.exists() {
        return Ok(Value::Object(serde_json::Map::new()));
    }
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("parsing JSON at {}", path.display()))
}

/// Perform the 3-layer merge and write the result to `output_path`.
///
/// # Arguments
/// * `global_path` — `~/.claude/settings.json`
/// * `profile_override` — `profiles/{p}/settings-override.json` (may not exist)
/// * `session_override` — `profiles/{p}/sessions/{s}/settings-override.json` (may not exist)
/// * `output_path` — `profiles/{p}/sessions/{s}/.claude/settings.json`
pub fn merge_and_write(
    global_path: &Path,
    profile_override: &Path,
    session_override: &Path,
    output_path: &Path,
) -> Result<()> {
    let mut merged = load_json(global_path)?;
    let profile_json = load_json(profile_override)?;
    let session_json = load_json(session_override)?;

    deep_merge(&mut merged, &profile_json);
    deep_merge(&mut merged, &session_json);

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let output = serde_json::to_string_pretty(&merged)?;
    std::fs::write(output_path, output)
        .with_context(|| format!("writing merged settings to {}", output_path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    #[test]
    fn test_deep_merge_simple_override() {
        let mut dst = json!({ "model": "haiku", "thinking": false });
        let src = json!({ "thinking": true });
        deep_merge(&mut dst, &src);
        assert_eq!(dst["thinking"], json!(true));
        assert_eq!(dst["model"], json!("haiku")); // untouched
    }

    #[test]
    fn test_deep_merge_nested_objects() {
        let mut dst = json!({ "sandbox": { "enabled": true, "network": { "allowed": [] } } });
        let src = json!({ "sandbox": { "network": { "allowed": ["github.com"] } } });
        deep_merge(&mut dst, &src);
        assert_eq!(dst["sandbox"]["enabled"], json!(true));
        assert_eq!(dst["sandbox"]["network"]["allowed"], json!(["github.com"]));
    }

    #[test]
    fn test_deep_merge_array_replacement() {
        let mut dst = json!({ "tags": ["a", "b"] });
        let src = json!({ "tags": ["c"] });
        deep_merge(&mut dst, &src);
        assert_eq!(dst["tags"], json!(["c"]));
    }

    #[test]
    fn test_merge_and_write_roundtrip() {
        let dir = TempDir::new().unwrap();

        let global = dir.path().join("global.json");
        let profile_ov = dir.path().join("profile.json");
        let session_ov = dir.path().join("session.json");
        let output = dir.path().join("output.json");

        std::fs::write(&global, r#"{"model":"haiku","thinking":false}"#).unwrap();
        std::fs::write(&profile_ov, r#"{"model":"sonnet"}"#).unwrap();
        // session override doesn't exist

        merge_and_write(&global, &profile_ov, &session_ov, &output).unwrap();

        let result: Value = serde_json::from_str(&std::fs::read_to_string(&output).unwrap()).unwrap();
        assert_eq!(result["model"], json!("sonnet"));
        assert_eq!(result["thinking"], json!(false));
    }

    #[test]
    fn test_load_json_nonexistent_returns_empty_object() {
        let dir = TempDir::new().unwrap();
        let val = load_json(&dir.path().join("nope.json")).unwrap();
        assert!(val.is_object());
        assert_eq!(val.as_object().unwrap().len(), 0);
    }
}
