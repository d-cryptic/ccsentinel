# Contributing

## Dev Setup

### Prerequisites

- Rust (stable) — install via [rustup](https://rustup.rs/)
- Node.js 22+ and bun (for Tauri frontend)
- Tauri CLI v2: `cargo install tauri-cli`

### Optional (recommended)

- **devbox** — reproducible environment: `devbox shell`
- **direnv** — auto-loads `.envrc` when you `cd` into the repo
- **cargo-nextest** — faster parallel test runner: `cargo install cargo-nextest`
- **cargo-watch** — re-run on save: `cargo install cargo-watch`

### Clone and build

```bash
git clone https://github.com/d-cryptic/claude-sentinel
cd claude-sentinel

# Build all crates
cargo build

# Install the CLI locally
cargo install --path crates/cst-cli
```

---

## Repo Structure

```
crates/
  cst-core/   Shared library — all domain logic. No binary here.
  cst-cli/    CLI binary (cst). Thin layer over cst-core + TUI.
apps/
  desktop/    Tauri v2 app (React + TypeScript frontend).
docs/         Documentation.
.github/      CI workflows.
```

**Rule**: All shared logic goes in `cst-core`. Never duplicate business logic in `cst-cli` or the Tauri backend.

---

## TDD Workflow

Write tests **before** implementation (red-green-refactor):

1. **RED** — write a failing test
2. **GREEN** — minimal code to pass
3. **REFACTOR** — clean up without breaking tests

```bash
# Watch mode (re-runs on save)
cargo watch -x "nextest run"

# Run all tests
cargo nextest run

# Run tests for a specific crate
cargo nextest run -p cst-core
```

Tests live in `#[cfg(test)]` modules within each source file. Integration tests go in `crates/*/tests/`.

---

## Code Standards

- No `unwrap()` or `expect()` in production paths — use `?` with `anyhow`
- `thiserror` for library error types, `anyhow` for binary propagation
- All public functions have rustdoc comments
- Run `cargo clippy --all -- -D warnings` before committing
- Run `cargo fmt --all` before committing

---

## Commit Format

Conventional commits:

```
feat: add quota warning notifications
fix: prevent panic when profiles dir is missing
docs: update auto-switch configuration guide
chore: update dependencies
test: add scheduler edge case coverage
refactor: extract rate-limit detection into detector module
```

Commit after each logical unit of work — don't batch unrelated changes.

---

## Pull Requests

- Link the related issue in the PR description
- Keep PRs focused on one concern
- All CI checks must pass before merge
- Self-review the diff before requesting review

---

## Running the Tauri App

```bash
cd apps/desktop
bun install
cargo tauri dev
```

Build a release:

```bash
cargo tauri build
```

---

## Adding a New cst-core Module

1. Create `crates/cst-core/src/mymodule.rs`
2. Add `pub mod mymodule;` to `crates/cst-core/src/lib.rs`
3. Write tests in `#[cfg(test)]` at the bottom of the file
4. Export any public types from `lib.rs` if needed

---

## Adding a New CLI Command

1. Create `crates/cst-cli/src/commands/mycommand.rs`
2. Add `pub mod mycommand;` to `commands/mod.rs`
3. Add the variant to the `Commands` enum in `main.rs`
4. Add the match arm in `main()`
