# Changelog

All notable changes to claude-sentinel are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased] — v0.1.0

### Added

#### Core library (`cst-core`)
- **1Password / Doppler secret integration** — `auth/secrets.rs`: `SecretSource` enum supporting Keychain (default), `op read "op://..."` (1Password CLI), `doppler secrets get KEY` (Doppler CLI), and plain env var; `ApiKeyEntry.source` field replaces hardcoded keychain path; backwards-compatible with existing `api_keys.toml`; `add_external_key()` + `describe_sources()` helpers; 11 unit tests
- **Team profile sharing** — `team_sync.rs`: `cst team init/push/pull/status`; clones git remote into local cache; `push()` copies only safe files (profile.toml, settings overrides, MCP config, env.toml, auto-switch.toml) and commits; `pull()` fetches + resets; credentials and stats never synced; include/exclude_profiles filter; 6 unit tests
- **`.cstrc` auto-detect** — `auto_detect.rs`: walk directory tree for `.cstrc` (TOML), match `profile`/`session` fields; `[[auto_detect]]` entries with `git_remote_pattern` globs for git-URL-based selection; glob normalises SSH/HTTPS URLs automatically; 13 unit tests
- **Live `history.jsonl` parser** — `history_parser.rs`: scan Claude Code's JSONL history for `usage` objects, sum `input_tokens`/`output_tokens`/cache fields; `estimated_cost_usd()` helper; gracefully skips invalid lines; 10 unit tests
- **Round-robin config** — `RoundRobin` struct in `auto_switch/config.rs`: `[round_robin]` TOML section with `pool`, `rotate_after_tokens`, `enabled`; daemon can distribute usage across a pool of profiles to maximise uptime

#### CLI (`cst-cli`)
- **`cst _auto-detect <dir> <current>`** — hidden command called by precmd hook; emits env exports when `.cstrc` requests a different profile than currently active
- **`cst auto-detect-status [<dir>]`** — show what `.cstrc` would activate in a directory without switching
- **`cst remaining` live counts** — now prefers live `history.jsonl` token counts over cached `stats.json`; shows `(live)` label when real data is available

#### Shell integration
- `_cst_check_switch` precmd hook now has a third step (after pending-switch and broadcast): checks `.cstrc` via `cst _auto-detect $PWD $CST_CURRENT`
- Supports zsh, bash, fish

#### Infrastructure
- **`.github/workflows/release.yml`** — builds `cst` for 5 targets (aarch64/x86_64 × macOS/Linux + Windows x86_64); `.zip` for Windows, `.tar.gz` for Unix; creates GitHub Release with checksums; extracts release notes from `CHANGELOG.md`
- **Homebrew formula** — `Formula/claude-sentinel.rb`: selects binary by arch/OS, installs shell completions, caveats for first-run
- **Raycast script commands** — `raycast/`: switch-profile, show-status, show-remaining, list-profiles (bash scripts, compact/fullOutput modes)
- 130 unit tests (up from 87)

#### Core library (`cst-core`)
- **Profile management** — CRUD, clone, rename, import, templates (pro/max/api/bedrock/vertex)
- **Session management** — CRUD, tag, archive, symlink setup for shared global config
- **Auth modules** — OAuth symlink swap, API key pool (Keychain/AES-GCM), AWS Bedrock env injection, Google Vertex AI env injection
- **3-layer settings merge** — global + profile + session overrides deep-merged on activate
- **MCP overrides** — per-profile add/disable of MCP servers vs global `~/.claude.json`
- **env.toml overlay** — per-session extra environment variables
- **ProfileHooks** — pre/post switch_in/out lifecycle hooks (non-fatal `sh -c`)
- **SessionStats** — token counts, cost estimates, rate-limit hit tracking
- **Auto-switch daemon** — tokio async file watcher, rate-limit pattern detection (10 patterns), fallback chain, quota reset scheduler, switch-back timer
- **Switch log** — append-only JSONL event log with reason, from/to, timestamp
- **Broadcast switch** — TTL-based broadcast file for signalling all open shells to switch profiles; per-shell `CST_BROADCAST_ID` prevents duplicate application
- **Platform paths** — cross-platform data/profile/session/claude-config dirs via `dirs` crate

#### CLI (`cst-cli`)
- **Full command surface**: `use`, `status`, `list`, `remaining`, `top`, `history`, `why`, `new`, `import`, `clone`, `rm`, `rename`, `login`, `add-key`, `session *`, `daemon *`, `auto-switch *`, `pause`, `run`, `sync`, `stats`, `doctor`, `validate`, `shell-init`, `starship`, `tmux`, `completions`, `templates`, `init`
- **`cst switch-all <from> <to>`** — broadcast profile switch to all open shells
- **`cst session switch <session> --to <profile>`** — reassign a session to a different profile
- **`cst top`** — htop-style live dashboard (1s refresh): token usage table, quota timers, recent switch events, daemon status, braille spinner
- **`cst doctor`** — 5-group health check: Claude Code install, data dir, profiles/sessions symlinks, daemon PID health, shell rc integration; exits 1 on hard failures
- **`cst remaining`** — token usage for active session + profile totals + rate-limit countdown timers + cross-profile summary table
- **`cst starship`** — Starship custom module output with quota warning; `--config` prints TOML snippet
- **`cst tmux`** — tmux status-right segment; `--config` prints config snippet
- **ratatui TUI** — 4-tab interactive navigator (Profiles, Sessions, Auto-Switch, History); Enter activates via pending-switch; `r` refreshes

#### Shell integration
- `eval "$(cst shell-init)"` — installs `cst` shell function + `_cst_check_switch` precmd hook
- **Precmd hook** — checks both one-shot pending-switch (daemon-initiated) and broadcast-switch file
- Supports: zsh, bash, fish, PowerShell
- `CST_BROADCAST_ID` per-shell env var prevents re-applying the same broadcast

#### Desktop app (`apps/desktop`)
- Tauri v2 app with system tray — left-click toggles window, right-click menu
- 4-tab window: Profiles, Sessions, Auto-Switch, Stats
- **Neubrutalism design system** — pure `#000`/`#fff`, 2-4px solid borders, 4px offset shadows, zero border-radius, monospace throughout, ALL-CAPS labels
- Zustand stores for profiles and daemon state
- Tauri commands wrap `cst-core` for all CRUD and daemon operations

#### Infrastructure
- Cargo workspace: `cst-core`, `cst-cli`, `apps/desktop/src-tauri`
- GitHub Actions CI: test + clippy + release build matrix (ubuntu + macos, x86_64 + aarch64)
- devbox + direnv dev environment
- 87 unit tests

#### Documentation
- `docs/ARCHITECTURE.md` — crate structure, data flows, daemon design, TUI, Tauri app
- `docs/AUTH.md` — all 4 auth types
- `docs/AUTO-SWITCH.md` — daemon config, rate-limit patterns, monitoring with `cst top`
- `docs/CONTRIBUTING.md` — dev setup, TDD workflow, CI pipeline, commit conventions
- `docs/DESIGN.md` — complete neubrutalism design system spec
- `docs/INSTALL.md` — installation guide
- `docs/USAGE.md` — full CLI reference with examples and ASCII layout diagrams

---

## [0.0.0] — 2026-03-22

- Initial commit: Cargo workspace scaffold, directory structure, devbox/direnv config
