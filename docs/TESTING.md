# Testing Guide

**166 unit tests, 0 integration tests, 0 failures** (as of v0.1.0)

---

## Running Tests

```bash
# Run all tests (preferred — parallel, better output)
cargo nextest run

# Run cst-core tests only
cargo nextest run -p cst-core

# Run a specific module
cargo nextest run -p cst-core history_parser

# Watch mode (TDD)
cargo watch -x "nextest run"

# Fallback with cargo test
cargo test --package cst-core
```

---

## Test Structure

All unit tests live in `#[cfg(test)]` modules at the bottom of their source file. There are no separate test files — tests are co-located with the code they verify.

| Module | Tests | What is covered |
|--------|-------|----------------|
| `auto_detect` | 24 | Walk-up, glob edge cases, URL normalisation, malformed TOML, no-match |
| `auth/secrets` | 19 | Serde roundtrips, describe security, env var edge cases, tool check |
| `team_sync` | 13 | Safety invariants (auth never synced), filter logic, session file allowlist |
| `history_parser` | 18 | Summation, error paths, CRLF, truncated lines, double-field behaviour |
| `auto_switch/config` | 10 | Serde roundtrips, round-robin config, schedule config |
| `auto_switch/detector` | 10 | Rate-limit pattern detection across all 10 known patterns |
| `auto_switch/scheduler` | 8 | Quota reset timing, `time_until_refill()` |
| `auto_switch/switch_log` | 8 | Append, last_n, persistence |
| `broadcast` | 6 | TTL expiry, dedup by `CST_BROADCAST_ID` |
| `profile` | 8 | CRUD, sort order, templates |
| `session` | 8 | CRUD, sort, archive, tag |
| `merge` | 6 | 3-layer deep merge, nested objects |
| `mcp` | 6 | Disable/add override merge |
| `env_overlay` | 4 | Load, inject |
| `stats` | 4 | Load, update, cost estimate |
| `hooks` | 2 | Non-fatal failure, success |
| `templates` | 3 | List, find, missing |
| `config` | 3 | Load, save, roundtrip |
| `platform` | 2 | Path resolution |

---

## Test Categories

### Happy Path

The majority of tests verify nominal behaviour: correct structs deserialise from TOML/JSON, functions return expected values, and CRUD operations persist correctly.

### Error Paths

Deliberate coverage of failure conditions:

| Test | Module | What it verifies |
|------|--------|-----------------|
| `detect_malformed_toml_returns_none` | `auto_detect` | Invalid TOML silently returns `None` (no panic) |
| `parse_tokens_missing_file_errors` | `history_parser` | `parse_tokens()` returns `Err` for missing file |
| `truncated_json_line_is_skipped` | `history_parser` | Partial JSON line (mid-write) skipped, not panic |
| `env_var_source_missing_errors` | `auth/secrets` | Missing env var returns `Err` |
| `load_missing_config_returns_error` | `team_sync` | Missing `team-sync.toml` returns `Err` |

### Security Invariants

Tests that assert credentials are **never** copied or exposed:

| Test | What it asserts |
|------|----------------|
| `copy_profile_to_repo_only_copies_safe_files` | `auth/` directory not created in repo copy |
| `copy_profile_to_repo_does_not_copy_api_key_enc` | `.enc` and `api_keys.toml` not copied |
| `copy_profile_from_repo_only_copies_safe_files` | Even a malicious repo can't write `auth/` to local |
| `session_files_sync_copies_env_toml_not_stats` | `stats.json` (usage data) not synced |
| `sync_files_list_excludes_auth` | `SYNC_FILES` constant contains no auth-related names |
| `sync_files_list_is_complete_and_safe` | Allowlist checked for `key`/`enc`/`oauth`/`secret` substrings |
| `session_sync_files_list_excludes_history_and_stats` | `SESSION_SYNC_FILES` has no `history`, `stats`, `auth` |
| `describe_1password_contains_reference_not_secret` | `describe()` shows `op://` reference, never a retrieved value |

### Behaviour Documentation

Tests that document observable (sometimes surprising) behaviour:

| Test | Module | Documents |
|------|--------|-----------|
| `both_toplevel_and_nested_usage_are_accumulated` | `history_parser` | If a single JSONL event has both `usage` and `message.usage`, both are summed (additive). Callers must not emit duplicate usage in the same event. |
| `exclude_takes_precedence_over_include` | `team_sync` | When a profile appears in both `include_profiles` and `exclude_profiles`, exclude wins. |
| `glob_star_matches_empty_segment` | `auto_detect` | `github.com/myco/*` matches `github.com/myco/` (trailing slash, empty segment). |

---

## Writing New Tests

### Module tests (standard)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn my_new_test() {
        let dir = TempDir::new().unwrap();
        // ... use dir.path() for isolated filesystem access
        assert!(some_function(dir.path()).is_ok());
    }
}
```

### Security invariant pattern

```rust
#[test]
fn sensitive_file_not_copied() {
    let src = TempDir::new().unwrap();
    let dst = TempDir::new().unwrap();
    // Create the sensitive file in src
    std::fs::write(src.path().join("auth/secret.key"), "SECRET").unwrap();
    // Run the copy function
    copy_safe_files(src.path(), dst.path()).unwrap();
    // Assert it did NOT appear in dst
    assert!(!dst.path().join("auth/secret.key").exists());
}
```

### Error path pattern

```rust
#[test]
fn missing_input_returns_err() {
    let result = load_config(Path::new("/definitely/nonexistent"));
    assert!(result.is_err());
}
```

---

## Test Philosophy

1. **Unit tests only** — no integration or E2E tests yet. All test logic uses `tempfile::TempDir` for isolation; no global state.
2. **Happy path + error paths** — every public function should have at least one error-path test.
3. **Security invariants** — any function that copies or serialises data must have a test confirming credentials are excluded.
4. **Behaviour documentation** — surprising or edge-case behaviour should be documented with a test comment explaining *why* it works the way it does.
5. **No I/O side effects on real home directory** — tests must never read from or write to `~/.claude-sentinel/`. Use `TempDir`.

---

## Integration Tests

`crates/cst-cli/tests/cli_integration.rs` — **60 integration tests** that build and invoke the `cst` binary against isolated temp directories (via `CST_DATA_DIR` env var override).

See [INTEGRATION-TESTING.md](INTEGRATION-TESTING.md) for the full breakdown.

```bash
# Run all integration tests
cargo test -p cst-cli --test cli_integration

# Docker (Linux)
./scripts/test-docker.sh
```

## Known Test Gaps (Future Work)

| Gap | Priority | Notes |
|-----|----------|-------|
| Property-based tests for `glob_match` / `normalise_git_url` | Medium | Idempotency, commutativity for exact strings |
| Race condition tests for broadcast TTL expiry | Medium | Concurrent shell simulations |
| `cst add-key` / keychain round-trip | Medium | Interactive stdin + OS keyring; needs manual runbook |
| `cst login` / OAuth flow | High | Requires real Claude Code token; see manual runbook in INTEGRATION-TESTING.md |
| Daemon start/stop/auto-switch under load | High | Long-running process; manual or scheduled CI |
| Windows symlink fallback (junction) | Low | Requires Windows CI runner |
