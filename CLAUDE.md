# CLAUDE.md — claude-sentinel

## What This Is

**Claude Sentinel** (`cst`) — Intelligent Claude Code account, profile, and session manager.

- **CLI binary**: `cst` (Rust, `crates/cst-cli/`)
- **Shared library**: `cst-core` (Rust, `crates/cst-core/`)
- **Desktop app**: Claude Sentinel (Tauri v2 + React/TypeScript, `apps/desktop/`)

## Repo Structure

```
crates/
  cst-core/     Shared library — profile, session, auth, daemon, merge logic
  cst-cli/      CLI binary (cst command)
apps/
  desktop/      Tauri v2 desktop app (React + TypeScript, neubrutalism UI)
docs/           Documentation (updated after every major feature)
TODO.md         Living task list
```

## Dev Setup

Uses **devbox** + **direnv** for reproducible environment.

```bash
# Auto-activates with direnv. Or manually:
devbox shell
```

## Key Commands

```bash
make test           # cargo nextest run (preferred)
make test-watch     # TDD watch mode
make build          # cargo build
make install        # cargo install --path crates/cst-cli
make lint           # clippy --all -D warnings
make fmt            # cargo fmt --all
make check          # fmt + lint + test (run before commit)
make dev-app        # Tauri dev mode
make changelog      # generate CHANGELOG.md via git-cliff
```

## Coding Rules

1. **TDD**: Write the failing test FIRST, then implement to make it pass
2. **DRY**: All shared logic lives in `cst-core`. Never duplicate in `cst-cli` or Tauri
3. **No `unwrap()` in production paths** — use `?` with `anyhow::Result`
4. **`thiserror`** for library error types, **`anyhow`** for binary error propagation
5. **All public functions** must have rustdoc comments with at least one example
6. **`clippy::all`** must pass — zero warnings allowed in CI

## Domain Model

- **Profile**: An account identity (auth type + credentials + settings override)
- **Session**: An isolated workspace within a profile (own `CLAUDE_CONFIG_DIR`, project history)
- **Auth types**: `oauth` | `api` | `bedrock` | `vertex`
- **Auto-switch**: Background daemon that monitors rate limits and switches profiles automatically
- **Data dir**: `~/.claude-sentinel/` (platform-resolved)

## Architecture Decisions

See `docs/ARCHITECTURE.md`

## Testing

```bash
# Unit tests: inline #[cfg(test)] modules in source files
# Integration tests: crates/*/tests/ directories
cargo nextest run                    # all tests
cargo nextest run -p cst-core        # just core
cargo nextest run test_profile_crud  # specific test
```

## Commit Convention

`feat:`, `fix:`, `docs:`, `chore:`, `test:`, `refactor:`, `perf:`

Commit after every logical unit of work. Never commit broken code.
