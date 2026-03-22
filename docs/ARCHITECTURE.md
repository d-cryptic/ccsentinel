# Architecture

## Overview

Claude Sentinel is a Cargo workspace with two crates and one Tauri app:

```
cst-core    Shared library — all domain logic, no I/O side effects in tests
cst-cli     CLI binary (cst) — thin layer over cst-core + TUI
desktop     Tauri v2 app — menu bar/tray + management window
```

## Data Flow: Profile Switch

```
cst use work:backend
  │
  ├── shell function calls: cst _env work:backend
  │     ├── cst-core: loads profile "work", session "backend"
  │     ├── cst-core: runs 3-layer settings merge → writes .claude/settings.json
  │     ├── cst-core: validates/creates symlinks (agents/, rules/, skills/, etc.)
  │     ├── cst-core: auth.activate() → OAuth symlink OR sets ANTHROPIC_API_KEY
  │     └── outputs: export CLAUDE_CONFIG_DIR=... CST_CURRENT=...
  │
  └── shell function eval's the exports → env vars set in current shell
```

## Data Directory

```
~/.claude-sentinel/
  config.toml              Current profile:session state
  profiles/
    {name}/
      profile.toml         Metadata (auth_type, created_at, color)
      auth/                Credentials (encrypted or keychain refs)
      sessions/
        {name}/
          .claude/         CLAUDE_CONFIG_DIR target
          settings-override.json
          stats.json
      settings-override.json
      auto-switch.toml
```

## Auth Architecture

Each auth type is handled by a dedicated module in `cst-core::auth`:

| Type | Module | Mechanism |
|------|--------|-----------|
| OAuth | `oauth.rs` | Symlink `~/.claude.json → profile/auth/oauth.json` |
| API key | `apikey.rs` | `ANTHROPIC_API_KEY` from Keychain / AES-GCM encrypted file |
| Bedrock | `bedrock.rs` | `AWS_*` env vars injected from `aws.toml` |
| Vertex AI | `vertex.rs` | `CLOUD_ML_REGION`, `ANTHROPIC_VERTEX_PROJECT_ID` etc. |

## Auto-Switch Daemon

```
cst daemon start → tokio background process
  │
  ├── FileWatcher: notify crate watches history.jsonl
  │     └── on write → detector.rs scans for rate limit patterns
  │
  ├── IPC server: named pipe / Unix socket
  │     └── cst exec wrapper writes rate limit signals here
  │
  ├── Scheduler: chrono-based timer
  │     └── fires auto-switch-back at rate_limit_time + estimate_minutes
  │
  └── On rate limit detected:
        1. key rotation: try next API key in pool
        2. if all keys exhausted: switch to next profile in fallback_chain
        3. write pending-switch file → shell precmd picks it up
        4. macOS notification
        5. schedule switch-back timer
```

## Settings Merge

Three TOML/JSON layers, deep-merged in order (later wins):

1. `~/.claude/settings.json` — global base (managed by main dotfiles)
2. `~/.claude-sentinel/profiles/{p}/settings-override.json` — profile level
3. `~/.claude-sentinel/profiles/{p}/sessions/{s}/settings-override.json` — session level

Result written to `…/sessions/{s}/.claude/settings.json` on each activate.

## TUI Architecture

`cst top` (live dashboard) and `cst` (interactive navigator) both use ratatui + crossterm.

```
cst top
  │
  ├── TopState::load() — initial data load
  ├── run_loop() — 100ms poll loop, 1s refresh cycle
  │     ├── terminal.draw(render) — ratatui frame
  │     ├── crossterm event poll — keyboard (q, r)
  │     └── state.refresh() — re-reads all profiles/stats/scheduler
  │
  └── render()
        ├── render_header() — active profile, daemon status, spinner
        ├── render_body() — profile/session table with token/cost columns
        ├── render_scheduler() — active rate-limit countdown timers
        ├── render_recent_events() — last 5 switch events
        └── render_footer() — key hints

cst (TUI navigator)
  │
  ├── AppState::load() — profiles, sessions, scheduler, history
  ├── 4 tabs: Profiles | Sessions | Auto-Switch | History
  ├── Enter → writes pending-switch + updates GlobalConfig
  └── r → refresh all data
```

## Terminal Integrations

### Starship
`cst starship` outputs a single line: `🛡 profile:session` optionally followed by `⚠ Xh Ym` when a rate-limit timer is active. The shell runs this command each prompt draw via Starship's `custom.cst` module.

### tmux
`cst tmux` outputs a tmux-markup string with `#[fg=...]` colour codes showing the current profile:session. Added to `status-right` with `status-interval 5`.

## Tauri Desktop App

```
apps/desktop/
  src/
    App.tsx               4-tab shell, status bar
    styles/
      neubrutalism.css    Design system (see docs/DESIGN.md)
    components/
      ProfileManager.tsx  Split-pane: profile list + detail
      SessionGrid.tsx     Card grid, click-to-switch
      AutoSwitchConfig.tsx  Daemon control, timers, switch log
      StatsPanel.tsx      ASCII bar chart, token/cost tables
    store/
      profiles.ts         Zustand — profiles, active, CRUD
      daemon.ts           Zustand — daemon status, switch log
  src-tauri/
    src/
      main.rs             Tauri app entry point
      tray.rs             System tray: left-click toggle, menu
      commands/
        profiles.rs       list/get/switch/create/delete
        sessions.rs       list/create/delete
        daemon.rs         status/start/stop/switch-log
        stats.rs          aggregate stats across all sessions
```

The Tauri backend is a thin adapter: all logic delegates to `cst-core`. Tauri commands call the same functions used by `cst-cli`.

## Crate Dependencies

```
cst-core  ←── cst-cli
          ←── apps/desktop/src-tauri (via Tauri commands)
```

`cst-core` has no dependency on `cst-cli`. All domain logic is in `cst-core` for testability.
