# TODO — claude-sentinel

## IN PROGRESS

- [ ] Bootstrap: Cargo workspace, devbox, direnv, CLAUDE.md, docs skeleton

## NEXT

- [ ] cst-core: data dir init, config.toml, Profile/Session structs (with TDD)
- [ ] cst-core: profile CRUD — new/list/rm/clone/import (with TDD)
- [ ] cst-core: session CRUD + symlink setup (with TDD)
- [ ] cst-core: auth — OAuth symlink swap (with TDD)
- [ ] cst-core: auth — API key pool + Keychain (with TDD)
- [ ] cst-core: auth — AWS Bedrock + Vertex AI env injection (with TDD)
- [ ] cst-core: 3-layer settings merge (with TDD)
- [ ] cst-core: MCP overrides merge (with TDD)
- [ ] cst-cli: shell-init, _env, cst use wrapper
- [ ] cst-cli: all core commands (use/list/status/new/rm/etc.)
- [ ] cst-core: auto-switch daemon (tokio, rate limit detection)
- [ ] cst-core: quota intelligence (remaining, warnings, scheduler)
- [ ] cst-core: stats tracking + cost estimation
- [ ] cst-cli: TUI (ratatui)
- [ ] cst-cli: cst init first-run wizard
- [ ] cst-cli: cst doctor health check
- [ ] cst-cli: cst top live dashboard
- [ ] apps/desktop: Tauri app setup
- [ ] apps/desktop: neubrutalism CSS design system
- [ ] apps/desktop: menu bar / system tray
- [ ] apps/desktop: 5-tab full management window

## BACKLOG

- [ ] Homebrew tap for easy install
- [ ] GitHub Actions CI (test + build on push)
- [ ] 1Password / Doppler integration for API keys
- [ ] Team profile sharing (git-based config sync)
- [ ] Raycast / Alfred extension for quick switching
- [ ] Smart round-robin: distribute usage across profiles
- [ ] Git remote URL → auto-detect profile
- [ ] cst starship module
- [ ] cst tmux status segment
- [ ] Windows installer (.msi)

## DONE

- [x] Plan: architecture, data model, feature set, tech stack
- [x] Bootstrap: repo directories, Cargo.toml workspace, .gitignore, devbox.json, .envrc, Makefile
