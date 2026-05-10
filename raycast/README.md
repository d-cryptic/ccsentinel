# Claude Sentinel — Raycast Script Commands

Raycast script commands for [claude-sentinel](https://github.com/d-cryptic/ccsentinel).

## Install

1. Open Raycast → Preferences → Extensions → Script Commands
2. Click **Add Directories** and point it at this `raycast/` folder
3. Make scripts executable: `chmod +x raycast/*.sh`
4. The commands appear in Raycast immediately

## Commands

| Command | Description |
|---------|-------------|
| **Switch Claude Profile** | Switch to a `profile[:session]` (leave blank to show current) |
| **Claude Sentinel Status** | Show active profile, session, auth type, daemon status |
| **Claude Quota Remaining** | Token counts, cost estimate, rate-limit timers |
| **List Claude Profiles** | All profiles with their sessions |

## Requirements

- `cst` binary in PATH: `cargo install cst-cli` or via Homebrew tap
- Shell integration active: `eval "$(cst shell-init)"` in your rc file

## Tip

The **Switch Claude Profile** command opens a text field. Type a profile name
(`work`, `personal`) or a `profile:session` pair (`work:backend`) and press Enter.
