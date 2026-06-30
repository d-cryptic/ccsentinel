//! 3-layer settings.json deep merge.
//!
//! Layer order (later wins on conflict):
//! 1. Global `~/.claude/settings.json`
//! 2. Profile `settings-override.json`
//! 3. Session `settings-override.json`
//!
//! The top-level `hooks` object is special-cased: its per-event arrays are
//! merged ADDITIVELY (concatenate + dedupe) rather than replaced, so hooks
//! declared in a lower layer (e.g. the global main-branch edit guard +
//! formatters) survive when a higher layer — a profile/session override or an
//! addon — adds its own hooks. See `merge_hooks_additive`.

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

/// Additively merge a `hooks` object (`overlay`) into `base`.
///
/// Both values are expected to be the value of a top-level `hooks` key: an
/// object mapping event names (`PreToolUse`, `SessionEnd`, …) to arrays of
/// hook entries. For each event present in `overlay`:
/// - if both layers hold an array, the entries are CONCATENATED, dropping any
///   exact duplicate (compared by serialized JSON) so re-application is
///   idempotent;
/// - otherwise the overlay's value is taken (new event, or non-array base).
///
/// Unlike [`deep_merge`] (which replaces arrays), this preserves lower-layer
/// hooks instead of substituting them.
pub fn merge_hooks_additive(base: &mut Value, overlay: &Value) {
    let overlay_map = match overlay.as_object() {
        Some(m) => m,
        None => return, // null / missing / wrong shape — nothing to merge
    };
    if !base.is_object() {
        *base = Value::Object(serde_json::Map::new());
    }
    let base_map = base.as_object_mut().expect("base coerced to object above");
    for (event, overlay_val) in overlay_map {
        match (base_map.get_mut(event), overlay_val.as_array()) {
            (Some(existing), Some(add_arr)) if existing.is_array() => {
                let existing_arr = existing.as_array_mut().expect("checked is_array");
                for item in add_arr {
                    let is_dup = existing_arr
                        .iter()
                        .any(|e| serde_json::to_string(e).ok() == serde_json::to_string(item).ok());
                    if !is_dup {
                        existing_arr.push(item.clone());
                    }
                }
            }
            _ => {
                base_map.insert(event.clone(), overlay_val.clone());
            }
        }
    }
}

/// Load JSON from a file, returning an empty object if the file doesn't exist.
pub fn load_json(path: &Path) -> Result<Value> {
    if !path.exists() {
        return Ok(Value::Object(serde_json::Map::new()));
    }
    let contents =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&contents).with_context(|| format!("parsing JSON at {}", path.display()))
}

/// Perform the 3-layer merge and write the result to `output_path`.
///
/// All keys use array-replace deep-merge semantics, EXCEPT the top-level
/// `hooks` object whose per-event arrays are merged additively (see
/// [`merge_hooks_additive`]).
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

    // Capture the global `hooks` BEFORE the generic merge replaces it.
    let mut hooks = merged.get("hooks").cloned().unwrap_or(Value::Null);

    deep_merge(&mut merged, &profile_json);
    deep_merge(&mut merged, &session_json);

    // Rebuild `hooks` additively across all three layers, intentionally
    // overriding the array-replace that deep_merge just applied. This keeps
    // lower-layer hooks (e.g. the global edit guard + formatters) intact when
    // an override/addon contributes its own hooks.
    if let Some(h) = profile_json.get("hooks") {
        merge_hooks_additive(&mut hooks, h);
    }
    if let Some(h) = session_json.get("hooks") {
        merge_hooks_additive(&mut hooks, h);
    }
    if hooks.as_object().map(|o| !o.is_empty()).unwrap_or(false) {
        merged["hooks"] = hooks;
    }

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let output = serde_json::to_string_pretty(&merged)?;
    // Atomic write: Claude Code may be reading this file concurrently; a
    // truncated/partial JSON would break it. write_atomic renames into place.
    crate::fs_util::write_atomic(output_path, output)
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

        let result: Value =
            serde_json::from_str(&std::fs::read_to_string(&output).unwrap()).unwrap();
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

    // ── Additive hooks merge (regression: addon hooks must not clobber the
    //    global main-branch edit guard + formatters) ──────────────────────────

    #[test]
    fn test_hooks_preserve_global_when_override_adds() {
        // global has the guard + formatter on PreToolUse; an override adds watchmen.
        let mut hooks = json!({
            "PreToolUse": [
                { "hooks": [{ "type": "command", "command": "main-branch-guard" }] },
                { "hooks": [{ "type": "command", "command": "ruff-format" }] }
            ]
        });
        let overlay = json!({
            "PreToolUse": [
                { "hooks": [{ "type": "command", "command": "watchmen_observe.sh" }] }
            ]
        });
        merge_hooks_additive(&mut hooks, &overlay);
        let arr = hooks["PreToolUse"].as_array().unwrap();
        assert_eq!(arr.len(), 3, "guard + formatter + watchmen all present");
        let blob = serde_json::to_string(&hooks).unwrap();
        assert!(blob.contains("main-branch-guard"), "global guard preserved");
        assert!(blob.contains("ruff-format"), "global formatter preserved");
        assert!(blob.contains("watchmen_observe.sh"), "addon hook added");
    }

    #[test]
    fn test_hooks_dedupe_idempotent() {
        let mut hooks = json!({
            "PreToolUse": [{ "hooks": [{ "type": "command", "command": "guard" }] }]
        });
        let overlay = json!({
            "PreToolUse": [{ "hooks": [{ "type": "command", "command": "watchmen" }] }]
        });
        merge_hooks_additive(&mut hooks, &overlay);
        merge_hooks_additive(&mut hooks, &overlay); // applied twice
        let arr = hooks["PreToolUse"].as_array().unwrap();
        assert_eq!(arr.len(), 2, "watchmen not duplicated on re-apply");
    }

    #[test]
    fn test_hooks_additive_new_event() {
        let mut hooks = json!({
            "PreToolUse": [{ "hooks": [{ "type": "command", "command": "guard" }] }]
        });
        let overlay = json!({
            "SessionEnd": [{ "hooks": [{ "type": "command", "command": "watchmen" }] }]
        });
        merge_hooks_additive(&mut hooks, &overlay);
        assert!(hooks.get("SessionEnd").is_some(), "new event added");
        assert_eq!(hooks["SessionEnd"].as_array().unwrap().len(), 1);
        assert_eq!(
            hooks["PreToolUse"].as_array().unwrap().len(),
            1,
            "existing event untouched"
        );
    }

    #[test]
    fn test_non_hooks_arrays_still_replace() {
        // Through the real merge_and_write path: non-hooks arrays REPLACE,
        // hooks arrays CONCATENATE.
        let dir = TempDir::new().unwrap();
        let global = dir.path().join("g.json");
        let profile_ov = dir.path().join("p.json");
        let session_ov = dir.path().join("s.json");
        let output = dir.path().join("o.json");

        std::fs::write(
            &global,
            r#"{"permissions":{"allow":["a","b"]},"hooks":{"PreToolUse":[{"hooks":[{"command":"guard"}]}]}}"#,
        )
        .unwrap();
        std::fs::write(
            &profile_ov,
            r#"{"permissions":{"allow":["c"]},"hooks":{"PreToolUse":[{"hooks":[{"command":"watchmen"}]}]}}"#,
        )
        .unwrap();

        merge_and_write(&global, &profile_ov, &session_ov, &output).unwrap();
        let result: Value =
            serde_json::from_str(&std::fs::read_to_string(&output).unwrap()).unwrap();

        // non-hooks array replaced (deep_merge semantics preserved elsewhere)
        assert_eq!(result["permissions"]["allow"], json!(["c"]));
        // hooks array concatenated (guard preserved + watchmen added)
        assert_eq!(result["hooks"]["PreToolUse"].as_array().unwrap().len(), 2);
    }
}
