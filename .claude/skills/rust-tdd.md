---
name: rust-tdd
description: TDD patterns for Rust — red-green-refactor with cargo-nextest
type: reference
---

# Rust TDD Patterns

## Workflow

1. **RED**: Write a failing `#[test]` that describes desired behavior
2. **GREEN**: Write minimal code to make it pass (no more than needed)
3. **REFACTOR**: Clean up without breaking tests

Run in watch mode: `cargo watch -x 'nextest run'`

## Test Structure

```rust
// Inline unit tests — in the same file as the code
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_profile_create_sets_name() {
        let dir = TempDir::new().unwrap();
        let manager = ProfileManager::new(dir.path());
        let profile = manager.create("work", AuthType::OAuth).unwrap();
        assert_eq!(profile.name, "work");
    }

    // For async code:
    #[tokio::test]
    async fn test_daemon_detects_rate_limit() { ... }
}
```

Integration tests in `crates/cst-core/tests/`:
```rust
// tests/profile_integration.rs
use cst_core::profile::ProfileManager;
```

## Test Helpers

```rust
// Use tempfile for isolated data dirs
let dir = tempfile::TempDir::new()?;
let data_dir = dir.path();

// Don't test actual Keychain in unit tests — use trait injection
trait CredentialStore: Send + Sync {
    fn store(&self, name: &str, value: &str) -> Result<()>;
    fn retrieve(&self, name: &str) -> Result<String>;
}
struct MockCredentialStore { ... }
```

## What NOT to Test

- Don't test the OS Keychain directly in unit tests (use mock)
- Don't test actual `~/.claude.json` symlink creation in unit tests (use tempdir)
- Don't write tests that depend on network or external Claude Code binary

## Naming Convention

`test_{what}_{scenario}_{expected_result}`

Examples:
- `test_profile_new_creates_directory`
- `test_merge_session_override_wins_over_profile`
- `test_daemon_rotates_key_on_rate_limit`
