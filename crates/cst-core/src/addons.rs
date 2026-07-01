//! Extensible "addons" system — declare MCP servers and settings/hooks once and
//! auto-apply them to every profile/session.
//!
//! Solves the problem that a freshly-created cst profile/session does not inherit
//! shared MCP servers (e.g. voicemode) or tool hooks (e.g. watchmen). Addons are
//! declarative JSON files stored in the data dir's `addons/` folder:
//!
//! ```json
//! {
//!   "name": "voicemode",
//!   "description": "…",
//!   "mcpServers": { "voicemode": { "type": "http", "url": "http://127.0.0.1:8765/mcp" } },
//!   "settings": { "hooks": { … } }
//! }
//! ```
//!
//! `mcpServers` is merged into each session's `McpOverride.add` (and from there
//! into the session `.claude.json` at switch time). `settings` is deep-merged into
//! the session `settings-override.json`, so it flows through the existing
//! `merge_and_write` layering into `.claude/settings.json`.
//!
//! Every public entry point has an `*_in` / `*_from` sibling that takes an explicit
//! addons directory. The no-arg versions delegate to [`platform::addons_dir`]; the
//! explicit versions keep tests hermetic (no global env mutation, no real-dir I/O).

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};

use crate::mcp::McpOverride;
use crate::merge::deep_merge;
use crate::platform;

/// File extension for on-disk addon definitions.
const ADDON_FILE_EXT: &str = "json";
/// Filename of the per-session settings override that addons deep-merge into.
const SETTINGS_OVERRIDE_FILE: &str = "settings-override.json";

/// A single addon: a named bundle of MCP servers and settings to apply to sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Addon {
    pub name: String,
    #[serde(default)]
    pub description: String,
    /// MCP servers merged into `McpOverride.add` for every session.
    #[serde(rename = "mcpServers", default)]
    pub mcp_servers: Map<String, Value>,
    /// Settings object deep-merged into each session's `settings-override.json`.
    #[serde(default = "empty_object")]
    pub settings: Value,
}

fn empty_object() -> Value {
    Value::Object(Map::new())
}

impl Addon {
    /// Path of an addon's JSON file within `addons_dir`.
    fn file_path_in(addons_dir: &Path, name: &str) -> PathBuf {
        addons_dir.join(format!("{name}.{ADDON_FILE_EXT}"))
    }

    /// Load an addon by name from the default addons dir (file, else built-in).
    pub fn load(name: &str) -> Result<Addon> {
        Self::load_from(&platform::addons_dir(), name)
    }

    /// Load an addon by name from `addons_dir`, preferring the on-disk file and
    /// falling back to a built-in definition of the same name.
    pub fn load_from(addons_dir: &Path, name: &str) -> Result<Addon> {
        let path = Self::file_path_in(addons_dir, name);
        if path.exists() {
            let contents = std::fs::read_to_string(&path)
                .with_context(|| format!("reading addon {}", path.display()))?;
            return serde_json::from_str(&contents)
                .with_context(|| format!("parsing addon {}", path.display()));
        }
        builtin()
            .into_iter()
            .find(|a| a.name == name)
            .with_context(|| format!("addon '{name}' not found (no file and no built-in)"))
    }

    /// Persist this addon into the default addons dir (atomic).
    pub fn save(&self) -> Result<()> {
        self.save_in(&platform::addons_dir())
    }

    /// Persist this addon into `addons_dir` (atomic).
    pub fn save_in(&self, addons_dir: &Path) -> Result<()> {
        let path = Self::file_path_in(addons_dir, &self.name);
        std::fs::create_dir_all(addons_dir)
            .with_context(|| format!("creating addons dir {}", addons_dir.display()))?;
        let contents = serde_json::to_string_pretty(self)?;
        crate::fs_util::write_atomic(&path, contents)
            .with_context(|| format!("writing addon {}", path.display()))
    }
}

/// Load every on-disk addon from the default addons dir.
pub fn load_all() -> Result<Vec<Addon>> {
    load_all_in(&platform::addons_dir())
}

/// Load every on-disk addon from `addons_dir`. A missing dir yields an empty vec.
pub fn load_all_in(addons_dir: &Path) -> Result<Vec<Addon>> {
    if !addons_dir.exists() {
        return Ok(Vec::new());
    }
    let mut addons = Vec::new();
    for entry in std::fs::read_dir(addons_dir)
        .with_context(|| format!("reading {}", addons_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some(ADDON_FILE_EXT) {
            continue;
        }
        match std::fs::read_to_string(&path)
            .and_then(|c| serde_json::from_str::<Addon>(&c).map_err(std::io::Error::other))
        {
            Ok(addon) => addons.push(addon),
            Err(e) => tracing::warn!("skipping invalid addon {}: {e}", path.display()),
        }
    }
    addons.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(addons)
}

/// The two shipped addons: `voicemode` (MCP server) and `watchmen` (settings hooks).
pub fn builtin() -> Vec<Addon> {
    vec![voicemode_addon(), watchmen_addon()]
}

/// Write built-in addon JSON not yet present in the default addons dir.
pub fn ensure_builtins_written() -> Result<()> {
    ensure_builtins_written_in(&platform::addons_dir())
}

/// Write built-in addon JSON not yet present in `addons_dir`.
/// Never overwrites a user-modified file.
pub fn ensure_builtins_written_in(addons_dir: &Path) -> Result<()> {
    for addon in builtin() {
        if !Addon::file_path_in(addons_dir, &addon.name).exists() {
            addon.save_in(addons_dir)?;
        }
    }
    Ok(())
}

/// Apply the enabled addons (resolved from the default addons dir) to a session.
pub fn apply_to_session(enabled: &[String], session_dir: &Path) -> Result<()> {
    apply_to_session_from(&platform::addons_dir(), enabled, session_dir)
}

/// Apply the enabled addons (resolved from `addons_dir`) to a single session directory.
///
/// - Merges each addon's `mcpServers` into the session `McpOverride.add` (addon wins).
/// - Deep-merges each addon's `settings` into the session `settings-override.json`.
///
/// Idempotent: running twice produces the same on-disk state.
pub fn apply_to_session_from(
    addons_dir: &Path,
    enabled: &[String],
    session_dir: &Path,
) -> Result<()> {
    if enabled.is_empty() {
        return Ok(());
    }

    let addons: Vec<Addon> = enabled
        .iter()
        .filter_map(|name| match Addon::load_from(addons_dir, name) {
            Ok(a) => Some(a),
            Err(e) => {
                tracing::warn!("skipping unknown addon '{name}': {e}");
                None
            }
        })
        .collect();

    // 1. Merge MCP servers into the session's mcp-override.json.
    let mut mcp = McpOverride::load(session_dir)?;
    let mut mcp_changed = false;
    for addon in &addons {
        for (name, config) in &addon.mcp_servers {
            mcp.add.insert(name.clone(), config.clone());
            mcp_changed = true;
        }
    }
    if mcp_changed {
        mcp.save(session_dir)?;
    }

    // 2. Deep-merge settings into the session's settings-override.json.
    let settings_path = session_dir.join(SETTINGS_OVERRIDE_FILE);
    let mut settings = crate::merge::load_json(&settings_path)?;
    let mut settings_changed = false;
    for addon in &addons {
        if addon.settings.is_object()
            && !addon
                .settings
                .as_object()
                .map(Map::is_empty)
                .unwrap_or(true)
        {
            deep_merge(&mut settings, &addon.settings);
            settings_changed = true;
        }
    }
    if settings_changed {
        let output = serde_json::to_string_pretty(&settings)?;
        crate::fs_util::write_atomic(&settings_path, output)
            .with_context(|| format!("writing {}", settings_path.display()))?;
    }

    Ok(())
}

fn voicemode_addon() -> Addon {
    let mcp_servers = serde_json::from_value(serde_json::json!({
        "voicemode": {
            "type": "http",
            "url": "http://127.0.0.1:8765/mcp"
        }
    }))
    .expect("static voicemode mcpServers is valid JSON object");
    Addon {
        name: "voicemode".to_string(),
        description:
            "Local voice (Whisper STT + Kokoro TTS) MCP for Claude Code — talk to/from Claude, fully offline"
                .to_string(),
        mcp_servers,
        settings: empty_object(),
    }
}

fn watchmen_addon() -> Addon {
    // Mirror the REAL installed watchmen hook shape from ~/.claude/settings.json:
    // 9 events; PreToolUse/PostToolUse carry an empty `matcher`, the rest omit it.
    const HOOK: &str =
        "/Users/barun/Developers/personal/watchmen/src/watchmen/hooks/watchmen_observe.sh";
    let with_matcher = serde_json::json!([{
        "hooks": [{ "type": "command", "command": HOOK }],
        "matcher": ""
    }]);
    let without_matcher = serde_json::json!([{
        "hooks": [{ "type": "command", "command": HOOK }]
    }]);
    let settings = serde_json::json!({
        "hooks": {
            "PreToolUse": with_matcher,
            "PostToolUse": with_matcher,
            "SessionStart": without_matcher,
            "SessionEnd": without_matcher,
            "UserPromptSubmit": without_matcher,
            "Stop": without_matcher,
            "SubagentStop": without_matcher,
            "Notification": without_matcher,
            "PreCompact": without_matcher
        }
    });
    Addon {
        name: "watchmen".to_string(),
        description:
            "Watchmen session-mining hooks — captures Claude Code transcripts for skill curation"
                .to_string(),
        mcp_servers: Map::new(),
        settings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_builtin_has_voicemode_and_watchmen() {
        let names: Vec<_> = builtin().into_iter().map(|a| a.name).collect();
        assert!(names.contains(&"voicemode".to_string()));
        assert!(names.contains(&"watchmen".to_string()));
    }

    #[test]
    fn test_addon_save_and_load_roundtrip() {
        let addons_dir = TempDir::new().unwrap();
        voicemode_addon().save_in(addons_dir.path()).unwrap();
        let loaded = Addon::load_from(addons_dir.path(), "voicemode").unwrap();
        assert_eq!(loaded.name, "voicemode");
        assert!(loaded.mcp_servers.contains_key("voicemode"));
    }

    #[test]
    fn test_load_falls_back_to_builtin_when_no_file() {
        let addons_dir = TempDir::new().unwrap();
        // Empty dir — load must fall back to the built-in definition.
        let loaded = Addon::load_from(addons_dir.path(), "watchmen").unwrap();
        assert_eq!(loaded.name, "watchmen");
        assert!(loaded.settings["hooks"]["PreToolUse"].is_array());
    }

    #[test]
    fn test_load_all_in_reads_only_json() {
        let addons_dir = TempDir::new().unwrap();
        voicemode_addon().save_in(addons_dir.path()).unwrap();
        std::fs::write(addons_dir.path().join("notes.txt"), b"ignore me").unwrap();
        let all = load_all_in(addons_dir.path()).unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].name, "voicemode");
    }

    #[test]
    fn test_ensure_builtins_written_does_not_overwrite() {
        let addons_dir = TempDir::new().unwrap();
        // Pre-seed a user-modified voicemode addon.
        let mut custom = voicemode_addon();
        custom.description = "USER EDIT".to_string();
        custom.save_in(addons_dir.path()).unwrap();

        ensure_builtins_written_in(addons_dir.path()).unwrap();

        // voicemode preserved, watchmen newly written.
        let vm = Addon::load_from(addons_dir.path(), "voicemode").unwrap();
        assert_eq!(vm.description, "USER EDIT");
        assert!(Addon::file_path_in(addons_dir.path(), "watchmen").exists());
    }

    #[test]
    fn test_apply_to_session_writes_mcp_and_settings() {
        let addons_dir = TempDir::new().unwrap();
        let session = TempDir::new().unwrap();
        apply_to_session_from(
            addons_dir.path(), // empty → built-in fallback
            &["voicemode".to_string(), "watchmen".to_string()],
            session.path(),
        )
        .unwrap();

        // mcp-override.json gained the voicemode server.
        let mcp = McpOverride::load(session.path()).unwrap();
        assert!(mcp.add.contains_key("voicemode"));

        // settings-override.json gained the watchmen hooks.
        let settings =
            crate::merge::load_json(&session.path().join(SETTINGS_OVERRIDE_FILE)).unwrap();
        assert!(settings["hooks"]["PreToolUse"].is_array());
    }

    #[test]
    fn test_apply_to_session_is_idempotent() {
        let addons_dir = TempDir::new().unwrap();
        let session = TempDir::new().unwrap();
        let enabled = vec!["voicemode".to_string(), "watchmen".to_string()];
        apply_to_session_from(addons_dir.path(), &enabled, session.path()).unwrap();
        let first = std::fs::read_to_string(session.path().join("mcp-override.json")).unwrap();
        let first_settings =
            std::fs::read_to_string(session.path().join(SETTINGS_OVERRIDE_FILE)).unwrap();

        apply_to_session_from(addons_dir.path(), &enabled, session.path()).unwrap();
        let second = std::fs::read_to_string(session.path().join("mcp-override.json")).unwrap();
        let second_settings =
            std::fs::read_to_string(session.path().join(SETTINGS_OVERRIDE_FILE)).unwrap();

        assert_eq!(first, second);
        assert_eq!(first_settings, second_settings);
    }

    #[test]
    fn test_apply_to_session_preserves_existing_unrelated_settings() {
        let addons_dir = TempDir::new().unwrap();
        let session = TempDir::new().unwrap();
        // Seed an unrelated key that must survive the merge.
        std::fs::write(
            session.path().join(SETTINGS_OVERRIDE_FILE),
            r#"{"model":"opus","permissions":{"defaultMode":"default"}}"#,
        )
        .unwrap();

        apply_to_session_from(addons_dir.path(), &["watchmen".to_string()], session.path())
            .unwrap();

        let settings =
            crate::merge::load_json(&session.path().join(SETTINGS_OVERRIDE_FILE)).unwrap();
        assert_eq!(settings["model"], serde_json::json!("opus"));
        assert_eq!(
            settings["permissions"]["defaultMode"],
            serde_json::json!("default")
        );
        assert!(settings["hooks"]["PreToolUse"].is_array());
    }

    #[test]
    fn test_apply_to_session_empty_is_noop() {
        let addons_dir = TempDir::new().unwrap();
        let session = TempDir::new().unwrap();
        apply_to_session_from(addons_dir.path(), &[], session.path()).unwrap();
        assert!(!session.path().join("mcp-override.json").exists());
        assert!(!session.path().join(SETTINGS_OVERRIDE_FILE).exists());
    }
}
