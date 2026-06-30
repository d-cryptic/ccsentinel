use anyhow::Result;
use cst_core::auth::{activate_profile_auth, activate_profile_auth_with};
use cst_core::env_overlay::EnvOverlay;
use cst_core::mcp::McpOverride;
use cst_core::profile::Profile;
use cst_core::shell::{env_exports, parse_profile_session, shell_init_code, ShellKind};
use cst_core::{fs_util, merge, platform, validate_profile_name, validate_session_name};
use std::collections::HashMap;
use std::path::Path;

pub fn shell_init(shell_arg: Option<String>) -> Result<()> {
    let shell = match shell_arg.as_deref() {
        Some("zsh") => ShellKind::Zsh,
        Some("bash") => ShellKind::Bash,
        Some("fish") => ShellKind::Fish,
        Some("powershell") | Some("ps") => ShellKind::PowerShell,
        _ => ShellKind::detect(),
    };
    println!("{}", shell_init_code(&shell));
    Ok(())
}

pub fn env_cmd(profile_session: &str) -> Result<()> {
    let (profile, session) = parse_profile_session(profile_session);
    // Validate before using profile/session as path components or in shell exports.
    validate_profile_name(&profile)?;
    validate_session_name(&session)?;
    let shell = ShellKind::detect();

    let claude_config_dir = platform::claude_config_dir(&profile, &session);
    let profile_dir = platform::profile_dir(&profile);
    let session_dir = platform::session_dir(&profile, &session);

    // Build env vars to export
    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert(
        "CLAUDE_CONFIG_DIR".to_string(),
        claude_config_dir.to_string_lossy().to_string(),
    );
    vars.insert(
        "CST_CURRENT".to_string(),
        format!("{}:{}", profile, session),
    );

    // Load profile once, then fire lifecycle hooks and inject auth-specific vars.
    // Parsing profile.toml once avoids TOCTOU and three redundant disk reads.
    // activate_profile_auth_with handles OAuth symlink swap, API key injection, etc.
    // (best-effort — if profile doesn't exist yet, just export CLAUDE_CONFIG_DIR)
    if profile_dir.exists() {
        let profile_toml = profile_dir.join("profile.toml");
        if let Ok(contents) = std::fs::read_to_string(&profile_toml) {
            if let Ok(p) = toml::from_str::<Profile>(&contents) {
                if let Err(e) = p.hooks.run_pre_switch_in() {
                    tracing::warn!("pre_switch_in hook failed for {profile}: {e}");
                }

                match activate_profile_auth_with(&profile, Some(&p)) {
                    Ok(auth_vars) => vars.extend(auth_vars),
                    Err(e) => tracing::warn!("auth activation failed for {profile}: {e}"),
                }

                if let Err(e) = p.hooks.run_post_switch_in() {
                    tracing::warn!("post_switch_in hook failed for {profile}: {e}");
                }
            } else {
                // Profile file exists but couldn't be parsed — fall back to name-only auth
                match activate_profile_auth(&profile) {
                    Ok(auth_vars) => vars.extend(auth_vars),
                    Err(e) => tracing::warn!("auth activation failed for {profile}: {e}"),
                }
            }
        }
    }

    // Load per-session env.toml overlay
    if session_dir.exists() {
        let overlay = EnvOverlay::load(&session_dir)?;
        vars.extend(overlay.env);
    }

    // Run settings merge if session dir exists
    if session_dir.exists() {
        let global_settings = platform::global_claude_dir().join("settings.json");
        let profile_override = profile_dir.join("settings-override.json");
        let session_override = session_dir.join("settings-override.json");
        let output = platform::claude_config_dir(&profile, &session).join("settings.json");
        if let Err(e) = merge::merge_and_write(
            &global_settings,
            &profile_override,
            &session_override,
            &output,
        ) {
            tracing::warn!("settings merge failed for {profile}:{session} — Claude Code may use stale settings: {e}");
        }
    }

    // Apply MCP overrides (incl. addon MCP servers) into the session's
    // .claude.json so they actually reach Claude Code. Best-effort.
    if session_dir.exists() {
        apply_mcp_override(
            &claude_config_dir,
            &session_dir,
            &platform::global_claude_dir(),
        );
    }

    // Update global config
    let mut cfg = cst_core::GlobalConfig::load().unwrap_or_default();
    cfg.current_profile = profile.clone();
    cfg.current_session = session.clone();
    if let Err(e) = cfg.save() {
        tracing::warn!("failed to save active profile/session to config: {e}");
    }

    // Output exports
    print!("{}", env_exports(&vars, &shell));
    Ok(())
}

/// Read `<session>/.claude/.claude.json`, apply the session's `McpOverride` to
/// its `mcpServers` object, and write the result back — preserving every other
/// top-level key (read-modify-write).
///
/// Best-effort: a missing, oversized, or unparseable `.claude.json` must never
/// abort the switch. We log a warning and continue, matching the surrounding code.
fn apply_mcp_override(claude_config_dir: &Path, session_dir: &Path, global_claude_dir: &Path) {
    // Guard against pathological `.claude.json` files (they can be large/stateful).
    const MAX_CLAUDE_JSON_BYTES: u64 = 50 * 1024 * 1024;
    let path = claude_config_dir.join(".claude.json");

    // Load the override first; if there is nothing to apply and no file exists,
    // skip to avoid creating a spurious .claude.json.
    let ov = match McpOverride::load(session_dir) {
        Ok(o) => o,
        Err(e) => {
            tracing::warn!("loading mcp-override failed: {e}");
            return;
        }
    };
    // Global MCP base: servers declared once in `<global>/mcp-servers.json`
    // (i.e. ~/.claude/mcp-servers.json) are merged into EVERY session's
    // .claude.json, mirroring how the global settings.json propagates hooks.
    // This is what makes e.g. voicemode universal across all profiles without
    // a per-session override.
    let global_base = read_global_mcp_base(global_claude_dir);

    let override_empty = ov.add.is_empty() && ov.disable.is_empty();
    if override_empty && global_base.is_empty() && !path.exists() {
        return;
    }

    let mut root: serde_json::Value = if path.exists() {
        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > MAX_CLAUDE_JSON_BYTES {
                tracing::warn!(
                    "{} is {} bytes (> {MAX_CLAUDE_JSON_BYTES}), skipping MCP override",
                    path.display(),
                    meta.len()
                );
                return;
            }
        }
        match std::fs::read_to_string(&path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
        {
            Some(v) => v,
            None => {
                tracing::warn!("could not parse {}, skipping MCP override", path.display());
                return;
            }
        }
    } else {
        serde_json::json!({ "mcpServers": {} })
    };

    if !root.is_object() {
        tracing::warn!(
            "{} is not a JSON object, skipping MCP override",
            path.display()
        );
        return;
    }

    // Merge mcpServers with precedence (low -> high): global base <
    // session-existing servers < override.add; override.disable removes from
    // the union. Skip the write entirely when nothing changed, to avoid
    // churning the (large, stateful) .claude.json on every switch.
    let existing: HashMap<String, serde_json::Value> = root
        .get("mcpServers")
        .and_then(|v| v.as_object())
        .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default();
    let mut base = global_base;
    for (k, v) in &existing {
        base.insert(k.clone(), v.clone());
    }
    let merged = ov.apply(&base);
    if merged == existing {
        return;
    }
    let merged_value = serde_json::Value::Object(merged.into_iter().collect());
    if let Some(obj) = root.as_object_mut() {
        obj.insert("mcpServers".to_string(), merged_value);
    }

    match serde_json::to_string_pretty(&root) {
        Ok(out) => {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Err(e) = fs_util::write_atomic(&path, out) {
                tracing::warn!("writing {} failed: {e}", path.display());
            }
        }
        Err(e) => tracing::warn!("serializing {} failed: {e}", path.display()),
    }
}

/// Read the global MCP base — `<global_claude_dir>/mcp-servers.json` — and
/// return its server map. Accepts the canonical `{"mcpServers": { ... }}`
/// shape or a bare `{ name: config }` map. Returns an empty map (never errors)
/// when the file is absent, too large, or unparseable; the merge degrades to
/// the prior per-session behaviour.
fn read_global_mcp_base(global_claude_dir: &Path) -> HashMap<String, serde_json::Value> {
    const MAX_BASE_BYTES: u64 = 5 * 1024 * 1024;
    let mut out: HashMap<String, serde_json::Value> = HashMap::new();
    let path = global_claude_dir.join("mcp-servers.json");
    if !path.exists() {
        return out;
    }
    if let Ok(meta) = std::fs::metadata(&path) {
        if meta.len() > MAX_BASE_BYTES {
            tracing::warn!(
                "{} is {} bytes (> {MAX_BASE_BYTES}), ignoring global MCP base",
                path.display(),
                meta.len()
            );
            return out;
        }
    }
    match std::fs::read_to_string(&path)
        .ok()
        .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
    {
        Some(v) => {
            let servers = v
                .get("mcpServers")
                .and_then(|m| m.as_object())
                .cloned()
                .or_else(|| v.as_object().cloned());
            if let Some(obj) = servers {
                for (k, val) in obj {
                    out.insert(k, val);
                }
            }
        }
        None => tracing::warn!(
            "could not parse {}, ignoring global MCP base",
            path.display()
        ),
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn apply_mcp_override_preserves_unrelated_keys() {
        let session = TempDir::new().unwrap();
        let claude_dir = session.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();

        // Existing .claude.json with stateful keys that must round-trip untouched.
        std::fs::write(
            claude_dir.join(".claude.json"),
            r#"{"userID":"u123","projects":{"/x":{"y":1}},"mcpServers":{"keep":{"command":"node"}}}"#,
        )
        .unwrap();

        // Override adds a server and disables nothing.
        let mut add = HashMap::new();
        add.insert(
            "voicemode".to_string(),
            serde_json::json!({"command": "uvx"}),
        );
        McpOverride {
            disable: vec![],
            add,
        }
        .save(session.path())
        .unwrap();

        apply_mcp_override(
            &claude_dir,
            session.path(),
            std::path::Path::new("/no/such/global"),
        );

        let v: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(claude_dir.join(".claude.json")).unwrap(),
        )
        .unwrap();
        // Unrelated keys preserved.
        assert_eq!(v["userID"], serde_json::json!("u123"));
        assert_eq!(v["projects"]["/x"]["y"], serde_json::json!(1));
        // Both old and added MCP servers present.
        assert!(v["mcpServers"]["keep"].is_object());
        assert_eq!(
            v["mcpServers"]["voicemode"]["command"],
            serde_json::json!("uvx")
        );
    }

    #[test]
    fn apply_mcp_override_creates_file_when_override_present() {
        let session = TempDir::new().unwrap();
        let claude_dir = session.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();

        let mut add = HashMap::new();
        add.insert(
            "voicemode".to_string(),
            serde_json::json!({"command": "uvx"}),
        );
        McpOverride {
            disable: vec![],
            add,
        }
        .save(session.path())
        .unwrap();

        apply_mcp_override(
            &claude_dir,
            session.path(),
            std::path::Path::new("/no/such/global"),
        );

        let path = claude_dir.join(".claude.json");
        assert!(path.exists());
        let v: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            v["mcpServers"]["voicemode"]["command"],
            serde_json::json!("uvx")
        );
    }

    #[test]
    fn apply_mcp_override_no_override_no_file_is_noop() {
        let session = TempDir::new().unwrap();
        let claude_dir = session.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();
        // No mcp-override.json, no .claude.json — must not create one.
        apply_mcp_override(
            &claude_dir,
            session.path(),
            std::path::Path::new("/no/such/global"),
        );
        assert!(!claude_dir.join(".claude.json").exists());
    }

    #[test]
    fn global_mcp_base_injected_into_empty_session() {
        let session = TempDir::new().unwrap();
        let claude_dir = session.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();
        let global = TempDir::new().unwrap();
        std::fs::write(
            global.path().join("mcp-servers.json"),
            r#"{"mcpServers":{"voicemode":{"command":"uvx"}}}"#,
        )
        .unwrap();

        // No session .claude.json, no override — voicemode comes purely from the
        // global base, proving it propagates universally like global hooks.
        apply_mcp_override(&claude_dir, session.path(), global.path());

        let v: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(claude_dir.join(".claude.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            v["mcpServers"]["voicemode"]["command"],
            serde_json::json!("uvx")
        );
    }

    #[test]
    fn session_server_wins_over_global_base() {
        let session = TempDir::new().unwrap();
        let claude_dir = session.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();
        std::fs::write(
            claude_dir.join(".claude.json"),
            r#"{"mcpServers":{"voicemode":{"command":"session-custom"}}}"#,
        )
        .unwrap();
        let global = TempDir::new().unwrap();
        std::fs::write(
            global.path().join("mcp-servers.json"),
            r#"{"mcpServers":{"voicemode":{"command":"uvx"}}}"#,
        )
        .unwrap();

        apply_mcp_override(&claude_dir, session.path(), global.path());

        let v: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(claude_dir.join(".claude.json")).unwrap(),
        )
        .unwrap();
        // A session-specific definition takes precedence over the global base.
        assert_eq!(
            v["mcpServers"]["voicemode"]["command"],
            serde_json::json!("session-custom")
        );
    }

    #[test]
    fn override_disable_removes_global_base_server() {
        let session = TempDir::new().unwrap();
        let claude_dir = session.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();
        // Session already has voicemode (e.g. from a prior base merge) + a keeper.
        std::fs::write(
            claude_dir.join(".claude.json"),
            r#"{"mcpServers":{"keep":{"command":"node"},"voicemode":{"command":"uvx"}}}"#,
        )
        .unwrap();
        let global = TempDir::new().unwrap();
        std::fs::write(
            global.path().join("mcp-servers.json"),
            r#"{"mcpServers":{"voicemode":{"command":"uvx"}}}"#,
        )
        .unwrap();
        // Per-session disable wins over the global base.
        McpOverride {
            disable: vec!["voicemode".to_string()],
            add: HashMap::new(),
        }
        .save(session.path())
        .unwrap();

        apply_mcp_override(&claude_dir, session.path(), global.path());

        let v: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(claude_dir.join(".claude.json")).unwrap(),
        )
        .unwrap();
        assert!(v["mcpServers"]["keep"].is_object());
        assert!(v["mcpServers"].get("voicemode").is_none());
    }
}
