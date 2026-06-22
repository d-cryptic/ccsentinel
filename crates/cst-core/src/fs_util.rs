//! Filesystem helpers shared across the crate: atomic writes and
//! restrictive permissions for credential files.

use anyhow::{Context, Result};
use std::path::Path;

/// Atomically write `contents` to `path`.
///
/// Writes to a sibling temporary file in the same directory, then renames it
/// over the destination. A concurrent reader (e.g. Claude Code reading
/// `settings.json`) therefore never observes a half-written file: it sees
/// either the old contents or the complete new contents.
///
/// The parent directory must already exist.
///
/// # Examples
///
/// ```
/// use cst_core::fs_util::write_atomic;
/// let dir = tempfile::tempdir().unwrap();
/// let path = dir.path().join("settings.json");
/// write_atomic(&path, b"{\"model\":\"opus\"}").unwrap();
/// assert_eq!(std::fs::read_to_string(&path).unwrap(), "{\"model\":\"opus\"}");
/// ```
pub fn write_atomic(path: &Path, contents: impl AsRef<[u8]>) -> Result<()> {
    let parent = path
        .parent()
        .with_context(|| format!("path has no parent directory: {}", path.display()))?;
    // Unique-ish temp name to avoid clobbering a concurrent writer's temp file.
    let pid = std::process::id();
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "tmp".to_string());
    let tmp = parent.join(format!(".{file_name}.{pid}.tmp"));

    std::fs::write(&tmp, contents.as_ref())
        .with_context(|| format!("writing temp file {}", tmp.display()))?;

    match std::fs::rename(&tmp, path) {
        Ok(()) => Ok(()),
        Err(e) => {
            // Best-effort cleanup so we don't leave temp files behind on failure.
            let _ = std::fs::remove_file(&tmp);
            Err(e).with_context(|| format!("renaming {} -> {}", tmp.display(), path.display()))
        }
    }
}

/// Restrict a credential file to owner read/write only (`0600`) on Unix.
///
/// No-op on non-Unix platforms (Windows credentials are protected by the OS
/// keychain / ACLs instead). Safe to call on any file; errors only on a real
/// permission/IO failure.
///
/// # Examples
///
/// ```
/// use cst_core::fs_util::secure_file;
/// let dir = tempfile::tempdir().unwrap();
/// let path = dir.path().join("oauth.json");
/// std::fs::write(&path, b"secret").unwrap();
/// secure_file(&path).unwrap();
/// ```
pub fn secure_file(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
            .with_context(|| format!("restricting permissions on {}", path.display()))?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

/// Restrict a credential directory to owner access only (`0700`) on Unix.
///
/// No-op on non-Unix platforms.
pub fn secure_dir(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
            .with_context(|| format!("restricting permissions on {}", path.display()))?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_atomic_creates_file_with_contents() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.json");
        write_atomic(&path, b"hello").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "hello");
    }

    #[test]
    fn write_atomic_overwrites_existing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.json");
        std::fs::write(&path, b"old").unwrap();
        write_atomic(&path, b"new").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "new");
    }

    #[test]
    fn write_atomic_leaves_no_temp_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.json");
        write_atomic(&path, b"data").unwrap();
        let entries: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(entries, vec!["out.json".to_string()]);
    }

    #[cfg(unix)]
    #[test]
    fn secure_file_sets_0600() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("secret");
        std::fs::write(&path, b"x").unwrap();
        secure_file(&path).unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }
}
