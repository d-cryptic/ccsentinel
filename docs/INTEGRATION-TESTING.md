# Integration Testing

This document describes the integration test strategy for the `cst` CLI binary.

## Test Tiers

### Tier 1: Hermetic (CI-safe, no credentials)

All tests in `crates/cst-cli/tests/cli_integration.rs` are hermetic. They:

- Create isolated temp directories for `CST_DATA_DIR` and `HOME`
- Never touch `~/.claude-sentinel/` or `~/.claude/`
- Require no network access or API keys
- Run on all platforms (macOS, Linux, Windows)

This tier covers: profile CRUD, session CRUD, auto-detect, doctor output,
shell-init, completions, templates, and team-sync with local bare git repos.

### Tier 2: Semi-real (local git, no credentials)

Team sync tests use `git init --bare` to create a local repository, then
exercise `cst team init`, `cst team push`, and `cst team pull` against it.
These tests verify the git integration without needing a remote server.

These tests are gated behind a `has_git()` check and skip gracefully when
git is not installed.

### Tier 3: Real-auth (manual, not in CI)

Tests that require actual Claude Code credentials (OAuth login, API key
validation, quota checking) are not automated. See the manual runbook below.

## What Each Tier Tests

| Area | Tier 1 | Tier 2 | Tier 3 |
|------|--------|--------|--------|
| `--version`, `--help` | x | | |
| `shell-init` (bash/zsh/fish) | x | | |
| `completions` | x | | |
| `templates` | x | | |
| Profile create/clone/rename/rm | x | | |
| Session create/tag/archive/rm | x | | |
| `.cstrc` auto-detect | x | | |
| `auto-detect-status` | x | | |
| `doctor` (structure checks) | x | | |
| `validate` (structure) | x | | |
| `use` (env exports) | x | | |
| Starship/Tmux config output | x | | |
| `team init` (local bare repo) | | x | |
| `team push` / `team pull` | | x | |
| `cst login` (OAuth flow) | | | x |
| `cst remaining` (quota API) | | | x |
| `cst run` (with real Claude) | | | x |

## Running Tests Locally

### All integration tests

```bash
cargo test -p cst-cli --test cli_integration
```

### With output visible

```bash
cargo test -p cst-cli --test cli_integration -- --nocapture
```

### A single test

```bash
cargo test -p cst-cli --test cli_integration -- version_exits_zero
```

### Run only profile tests

```bash
cargo test -p cst-cli --test cli_integration -- profile
```

## Docker Testing

For Linux testing on macOS or to validate against specific distros:

```bash
# Run both Ubuntu and Debian
./scripts/test-docker.sh

# Or manually
docker build -f docker/test/Dockerfile -t cst-test-ubuntu .
docker run --rm cst-test-ubuntu

docker build -f docker/test/Dockerfile.debian -t cst-test-debian .
docker run --rm cst-test-debian
```

## CI Workflow

The `.github/workflows/integration.yml` workflow runs on every push to `main`
and every pull request. It executes:

1. **Native matrix**: Ubuntu, macOS, Windows (via GitHub Actions runners)
2. **Docker matrix**: Ubuntu 22.04 and Debian Bookworm containers

### Platform-specific notes

- **Ubuntu**: Requires `libdbus-1-dev`, `libssl-dev`, `libsecret-1-dev`
- **macOS**: No extra dependencies needed
- **Windows**: The `junction` crate is used for symlinks instead of Unix symlinks

## Test Architecture

### `CST_DATA_DIR` environment variable

The key enabler for hermetic testing is the `CST_DATA_DIR` env var override
in `cst-core/src/platform.rs`. When set, `platform::data_dir()` returns its
value instead of the default `~/.claude-sentinel/`. All path functions in
the platform module flow through `data_dir()`, so the entire data layer is
redirected.

### `TestEnv` struct

Each test creates a `TestEnv` which:

1. Builds the `cst` binary (once per test run via `CARGO_BIN_EXE_cst`)
2. Creates a fresh `TempDir` for `CST_DATA_DIR`
3. Creates a fresh `TempDir` for `HOME`
4. Populates a minimal `~/.claude/` directory
5. Runs `cst` with these env vars, isolating each test completely

### Test naming conventions

- `*_exits_zero` / `*_exits_nonzero` -- exit code checks
- `*_creates_*` -- filesystem side-effect assertions
- `*_fails` -- expected failure paths
- `*_emits_*` -- stdout/stderr content checks

## Manual Runbook: Real-Auth Testing

Before a release, manually verify these flows:

### OAuth flow

```bash
cst new test-oauth --auth oauth
cst login test-oauth
cst use test-oauth
cst status          # should show authenticated
cst remaining       # should show quota
```

### API key flow

```bash
cst new test-api --auth api
cst add-key test-api
cst use test-api
cst status
```

### Cross-profile switch

```bash
cst new alpha --auth oauth
cst new beta --auth api
cst use alpha
cst status          # alpha active
cst use beta
cst status          # beta active
```

### Daemon auto-switch

```bash
cst daemon start
cst daemon status   # should show running
# Create .cstrc in a project dir, cd into it
# Verify auto-switch happens at next prompt
cst daemon stop
```

## Known Limitations

1. **Session commands require active profile**: The `session new`, `session rm`,
   `session tag`, and `session archive` commands read `current_profile` from
   `config.toml`. Integration tests must call `set_current()` or `cst use`
   before exercising session subcommands.

2. **Daemon tests are limited**: The daemon requires a long-running process and
   IPC socket. Integration tests do not start the daemon. Daemon functionality
   should be tested via the manual runbook.

3. **TUI tests are not automated**: The `cst tui` and `cst top` commands
   require a terminal. They are excluded from integration tests.

4. **Keyring tests**: The `cst add-key` command reads from stdin interactively.
   It is not tested in the automated suite.

5. **Platform-specific symlink behavior**: On Windows, junctions are used
   instead of symlinks. The test suite accounts for this by checking directory
   existence rather than symlink targets.
