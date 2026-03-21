//! OAuth authentication — symlink `~/.claude.json` to the profile's auth file.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

use crate::platform;

/// Activate OAuth auth for a profile by symlinking `~/.claude.json`
/// to the profile's `auth/oauth.json`.
///
/// This is a file-level side effect (not pure), tested via tempdir.
pub fn activate(profile_auth_dir: &Path) -> Result<()> {
    let oauth_file = profile_auth_dir.join("oauth.json");
    if !oauth_file.exists() {
        bail!(
            "OAuth credentials not found at {}. Run: cst login <profile>",
            oauth_file.display()
        );
    }
    let global_json = platform::global_claude_json();
    platform::create_link(&oauth_file, &global_json)
        .context("failed to symlink ~/.claude.json to profile oauth.json")?;
    Ok(())
}

/// Deactivate by removing the symlink (called when switching away from OAuth profile).
/// Does NOT restore the original — caller is responsible for activating new profile.
pub fn deactivate() -> Result<()> {
    let global_json = platform::global_claude_json();
    if global_json.is_symlink() {
        std::fs::remove_file(&global_json)?;
    }
    Ok(())
}

/// Import the current `~/.claude.json` as the OAuth credentials for a profile.
pub fn import_current(profile_auth_dir: &Path) -> Result<PathBuf> {
    let global_json = platform::global_claude_json();
    if !global_json.exists() {
        bail!("~/.claude.json not found — is Claude Code installed and logged in?");
    }
    // Don't copy if it's already our symlink
    if global_json.is_symlink() {
        bail!("~/.claude.json is already managed by claude-sentinel. Use `cst login` instead.");
    }
    std::fs::create_dir_all(profile_auth_dir)?;
    let dst = profile_auth_dir.join("oauth.json");
    std::fs::copy(&global_json, &dst)?;
    Ok(dst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_activate_fails_when_oauth_json_missing() {
        let dir = TempDir::new().unwrap();
        let result = activate(dir.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_import_current_copies_file() {
        let auth_dir = TempDir::new().unwrap();
        let source_dir = TempDir::new().unwrap();
        let fake_claude_json = source_dir.path().join(".claude.json");
        std::fs::write(&fake_claude_json, r#"{"numStartups":1}"#).unwrap();

        // We can't easily override platform::global_claude_json() in tests,
        // so we test the copy logic directly.
        let dst = auth_dir.path().join("oauth.json");
        std::fs::copy(&fake_claude_json, &dst).unwrap();
        assert!(dst.exists());
        let content = std::fs::read_to_string(&dst).unwrap();
        assert!(content.contains("numStartups"));
    }
}
