# CLI Usage Reference

> **Disclaimer:** Claude Sentinel is an independent, open-source tool. It is not affiliated with, endorsed by, or associated with Anthropic PBC. "Claude" and "Claude Code" are trademarks of Anthropic PBC. This tool interacts with Claude Code through officially documented configuration mechanisms (`CLAUDE_CONFIG_DIR`, `ANTHROPIC_API_KEY`) only.

Full CLI reference for `cst` -- the Claude Sentinel command-line tool.

## Quick Start

```bash
# First-time setup
cst init

# Add to ~/.zshrc or ~/.bashrc (done automatically by init)
eval "$(cst shell-init)"

# Create your first profile (imports current ~/.claude.json)
cst import --as personal

# Switch to it
cst use personal

# Check status
cst status
```

## Interactive Interfaces

### TUI (Terminal UI)

```bash
cst         # launch TUI (default when no subcommand given)
cst tui     # explicit launch
```

The TUI has 4 tabs:

| Tab | Contents |
|-----|----------|
| PROFILES | Profile list (40%) with detail panel (60%) showing name, auth type, status, sessions |
| SESSIONS | Session list for the selected profile with active marker |
| AUTO-SWITCH | Scheduled activation entries with countdown to next switch |
| HISTORY | Last 30 switch events with timestamps and reasons |

**Keybindings**:

| Key | Action |
|-----|--------|
| `Tab` / `Right` | Next tab |
| `Shift+Tab` / `Left` | Previous tab |
| `j` / `Down` | Move down in list |
| `k` / `Up` | Move up in list |
| `Enter` | Activate selected profile:session |
| `r` | Refresh data from disk |
| `q` / `Ctrl+C` | Quit |

Selecting a profile with `Enter` writes a pending-switch file and updates global config. The shell precmd hook picks it up on the next prompt.

### Live Dashboard (`cst top`)

```bash
cst top
```

htop-style real-time dashboard that auto-refreshes every 1 second. Layout:

```
+----------------------------------------------------------------------+
| CST TOP | ACTIVE: work:backend          DAEMON ON                    |
+----------------------------------------------------------------------+
| PROFILE   SESSION   AUTH   IN     OUT    COST $  LAST USED          |
| work      backend   oauth  15.2k  4.5k   0.0124  03-22 19:00        |
| personal  default   oauth  8.1k   2.3k   0.0000  03-21 18:45        |
| api-work  deploy    api    120.5k 45.2k  1.2340  03-22 17:10        |
+----------------------------------------------------------------------+
| SCHEDULE                  | RECENT SWITCHES                         |
| personal -> active in 3h2m | personal -> work | schedule             |
+----------------------------------------------------------------------+
| q quit  r refresh  (refreshes every 1s)                              |
+----------------------------------------------------------------------+
```

Columns in the profile table:
- **PROFILE** / **SESSION** -- name (active row shown with bold marker)
- **AUTH** -- oauth, api, bedrock, vertex
- **IN** / **OUT** -- token counts (formatted as k/M)
- **COST $** -- estimated API cost in USD
- **LAST USED** -- timestamp (MM-DD HH:MM)

**Keybindings**: `q` quit, `r` force refresh.

## Profile Management

### `cst new <name>`

Create a new profile.

```bash
cst new personal                           # OAuth (default)
cst new work --auth oauth --template max   # Max plan template
cst new api-backup --auth api              # API key (stored in Keychain)
cst new bedrock-work --auth bedrock        # AWS Bedrock
cst new vertex-work --auth vertex          # Google Vertex AI
```

**Auth types**: `oauth`, `api`, `bedrock`, `vertex`

**Templates** (applied as base settings override): `pro`, `max`, `api`, `bedrock`, `vertex`

### `cst import [--as <name>]`

Import current `~/.claude.json` as a named profile.

```bash
cst import --as personal
cst import           # defaults to "default"
```

### `cst clone <src> <dst>`

Clone a profile (copies config, not credentials).

```bash
cst clone work work-staging
```

### `cst rm <name>`

Delete a profile and all its sessions.

### `cst rename <old> <new>`

Rename a profile.

### `cst login [<profile>]`

Re-run OAuth login for a profile. Defaults to current profile.

### `cst add-key <profile> [--slot N]`

Add an API key to a profile's key pool. Prompts securely; stores in macOS Keychain / libsecret / WinCred by default.

```bash
cst add-key api-work            # slot 1 (default) — stored in keychain
cst add-key api-work --slot 2   # second key for rotation
```

**External secret providers** — instead of the keychain, keys can live in 1Password or Doppler. Specify the source reference in `auth/api_keys.toml`:

```toml
# 1Password: op:// URI
[[keys]]
slot = 1
source = { provider = "one_password", reference = "op://Personal/Claude API/credential" }
note = "personal claude pro key"

# Doppler secret
[[keys]]
slot = 2
source = { provider = "doppler", secret_name = "ANTHROPIC_API_KEY", project = "myapp", config = "prd" }

# Plain env var (useful in CI)
[[keys]]
slot = 3
source = { provider = "env_var", var_name = "ANTHROPIC_API_KEY_BACKUP" }
```

Both `op` (1Password CLI) and `doppler` must be installed and authenticated. `cst validate <profile>` shows which provider each slot uses.

### `cst validate <profile>`

Validate a profile's config and credentials.

### `cst templates`

List built-in profile templates.

## Switching Profiles

### `cst use <profile>[:<session>]`

Switch to a profile. When called through the shell function (via `eval "$(cst shell-init)"`), updates the current shell's env vars.

```bash
cst use work
cst use work:backend
cst use personal:default
```

### `cst switch-all <from> <to>`

Broadcast a profile switch to **every open shell** currently running profile `from`. Each shell picks it up at its next prompt via the `_cst_check_switch` precmd hook.

```bash
cst switch-all work personal
# ✓ Broadcast queued: work → personal (expires in 5 min)
# All shells on 'work' will switch at their next prompt

# To also switch the current shell immediately:
cst use personal
```

How it works:
1. Writes `~/.claude-sentinel/broadcast-switch.json` with a 5-minute TTL and a unique ID
2. Every shell's precmd hook calls `cst _broadcast-switch $CST_CURRENT $CST_BROADCAST_ID`
3. If the shell's current profile matches `from` and the broadcast hasn't been applied yet, it gets the env exports
4. Each shell tracks `CST_BROADCAST_ID` so it never applies the same broadcast twice
5. The file expires after 5 minutes and is cleaned up automatically

### `cst run <profile:session> -- <cmd>`

Run a command with a specific profile without changing the current shell state.

```bash
cst run work:backend -- claude
cst run api-backup -- claude --dangerously-skip-permissions
```

## Session Management

### `cst session new <name> [--tag <desc>]`

Create a new session under the current profile.

```bash
cst session new backend --tag "API work"
cst session new frontend
```

### `cst session list [<profile>]`

List sessions for a profile (defaults to current).

### `cst session rm <name>`

Delete a session.

### `cst session tag <name> <description>`

Update a session's description tag.

### `cst session archive <name>`

Archive a session (hides from active list, keeps history).

### `cst session switch <session> --to <profile>`

Activate a specific session under a **different profile**. Creates the session in the target profile if it doesn't exist, then writes a pending-switch so the current shell picks it up.

```bash
# Run 'backend' session under api-backup's credentials instead of work's
cst session switch backend --to api-backup
# ✓ Created session 'backend' in profile 'api-backup'  (if new)
# ✓ Session 'backend' switched: work → api-backup

# Apply immediately in the current shell
cst use api-backup:backend
```

Use case: you want to run the `backend` session under different credentials (e.g., a dedicated API-key profile) while keeping other sessions on `work`.

## Team Profile Sharing

Share profile configs (settings, MCP overrides, env overlays, auto-switch rules) with your team via a shared git remote. Credentials are **never** synced.

### Setup

```bash
# Connect to a shared git remote
cst team init git@github.com:myorg/claude-profiles.git

# Push your profiles
cst team push

# On another machine — pull the team profiles
cst team pull

# Show sync status
cst team status
```

### What is synced

| File | Description |
|------|-------------|
| `profile.toml` | Profile metadata (auth type, description) |
| `settings-override.json` | Claude settings overrides |
| `mcp-override.json` | MCP server add/disable list |
| `auto-switch.toml` | Time-based schedule (`active_hours`, `timezone`, `fallback`) |
| `env.toml` | Per-session extra environment variables |

### What is never synced

- `auth/` — OAuth tokens, API keys, AWS/Vertex credentials
- `stats.json`, `history.jsonl` — usage data (stays local)

### Filter which profiles to sync

Add to `~/.claude-sentinel/team-sync.toml`:

```toml
# Only sync these profiles (empty = sync all)
include_profiles = ["work", "work-staging"]

# Always exclude these
exclude_profiles = ["personal"]
```

## Auto-Detect (`.cstrc`)

Claude Sentinel supports direnv-style per-project profile selection via a `.cstrc` file.

### How it works

When you `cd` into a directory, the shell precmd hook calls `cst _auto-detect $PWD` and automatically activates the matching profile — no manual `cst use` required.

The hook walks from the current directory up to the filesystem root looking for the nearest `.cstrc`. If found, it checks git remote URL patterns first, then falls back to the explicit `profile` field.

### `.cstrc` format

```toml
# ~/.../my-project/.cstrc

# Explicit profile (always applied when in this directory tree)
profile = "work"
session = "backend"

# Optional: more specific git remote URL patterns (checked first)
# Use * as a wildcard. Both SSH and HTTPS URLs are normalised automatically.
[[auto_detect]]
git_remote_pattern = "github.com/mycompany/*"
profile = "work"
session = "backend"

[[auto_detect]]
git_remote_pattern = "github.com/personal/*"
profile = "personal"
```

Git URL normalisation:
- `git@github.com:org/repo.git` → `github.com/org/repo`
- `https://github.com/org/repo.git` → `github.com/org/repo`

Pattern examples:
- `github.com/myco/*` — all repos in the `myco` org
- `*.myco.internal/*` — any host ending in `.myco.internal`

### `cst auto-detect-status [<dir>]`

Preview what `.cstrc` would activate in a directory without switching:

```bash
cst auto-detect-status        # current directory
cst auto-detect-status ~/work/api

# Output example:
# Profile : work:backend
# Source  : .cstrc at /home/user/work/.cstrc
# Status  : would switch from personal:default → work:backend
```

## Switch History

### `cst history`

Show switch history with reasons (manual, schedule, auto-detect).

### `cst why`

Explain why the current profile is active.

## Status and Diagnostics

### `cst status`

Show current profile:session, auth type, and quota status.

```
Profile : work
Session : backend
Auth    : oauth
Daemon  : running
```

### `cst list`

List all profiles and their sessions.

### `cst remaining`

Show usage for the active profile — token counts, estimated cost, and a cross-profile summary.

Token counts are read live from Claude Code's `history.jsonl` when available (labelled `(live)`), falling back to the cached `stats.json`.

```
Profile  : work:backend

── Token Usage (current session) (live) ──────────────────────────
  Tokens in   : 15.2k
  Tokens out  : 4.5k
  Total       : 19.7k
  Last used   : 2026-05-10 19:00 UTC

── All Sessions (work) ─────────────────────────────────────
  Tokens in   : 45.3k
  Tokens out  : 12.1k

── All Profiles ────────────────────────────────────────────
  work [ACTIVE]        in:   45.3k  out:   12.1k
  personal             in:    8.1k  out:    2.3k
  api-backup           in:  120.5k  out:   45.2k
```

### `cst stats [<profile:session>]`

Show detailed usage statistics. Includes: session count, tokens in/out, estimated API cost.

```bash
cst stats
cst stats work:backend
```

### `cst doctor`

Full health check — 5 check groups with pass/fail output:

| Group | What is checked |
|-------|----------------|
| Claude Code | `claude` binary in PATH, `~/.claude/`, `~/.claude.json` |
| Data Directory | `~/.claude-sentinel/`, profiles dir, `config.toml` |
| Profiles & Sessions | Profile config, auth files, session `.claude/` symlinks |
| Daemon | PID file health, running state, stale broadcast files |
| Shell Integration | `eval "$(cst shell-init)"` present in rc file |

```bash
cst doctor           # full check, exits 1 on failures
cst validate <name>  # per-profile credential + session detail
```

### `cst sync`

Rebuild symlinks from `~/.claude/` to all sessions.

## Scheduler Daemon

The daemon evaluates each profile's `[schedule]` (`active_hours`, `timezone`, `fallback`) and writes a pending-switch when the active window changes. It does not react to API errors or rate limit signals.

### Daemon lifecycle

```bash
cst daemon start
cst daemon stop
cst daemon restart
cst daemon status
cst daemon logs              # tail daemon log output
```

### Schedule configuration

```bash
cst auto-switch configure work        # interactive wizard (active_hours, timezone, fallback)
cst auto-switch log                   # history of all scheduled switches
cst auto-switch test work             # dry-run: show what would happen
```

### Pause scheduled switching

```bash
cst pause                    # pause indefinitely
cst pause --minutes 60       # pause for 1 hour
cst unpause                  # resume
```

See [AUTO-SWITCH.md](AUTO-SWITCH.md) for the full configuration reference.

## Shell Integration

### `cst shell-init [--shell <shell>]`

Outputs shell init code. Add to your rc file:

```bash
# ~/.zshrc or ~/.bashrc
eval "$(cst shell-init)"
```

Supports: `zsh`, `bash`, `fish`, `powershell`. Auto-detects if `--shell` is omitted.

The init code installs:
1. A `cst` shell function that wraps `cst use` to eval env exports in the current shell
2. A `precmd` hook (`_cst_check_switch`) that runs on every prompt with three steps:
   - **Step 1** — one-shot pending switch from daemon (this shell only)
   - **Step 2** — broadcast switch from `cst switch-all` (applies to all matching shells)
   - **Step 3** — `.cstrc` auto-detect for the current directory
3. `CST_CURRENT` variable showing active `profile:session`

### Starship Prompt Module

```bash
cst starship                # output for Starship custom module
cst starship --config       # print starship.toml config snippet
```

Shows `profile:session` (e.g., `work:backend`).

Add to `~/.config/starship.toml`:

```toml
[custom.cst]
command = "cst starship"
when = true
format = "[$output]($style) "
style = "bold white"
shell = ["sh"]
```

### tmux Status Bar Segment

```bash
cst tmux                    # output for tmux status-right
cst tmux --config           # print tmux.conf config snippet
```

Shows the active profile:session with tmux color markup.

Add to `~/.config/tmux/tmux.conf`:

```
set -g status-right "#(cst tmux) | %H:%M"
set -g status-interval 5
```

## Tab Completions

### `cst completions <shell>`

Generate shell tab completions.

```bash
cst completions zsh   > ~/.zfunc/_cst
cst completions bash  > /usr/local/etc/bash_completion.d/cst
cst completions fish  > ~/.config/fish/completions/cst.fish
```

## First-Run Setup

### `cst init [--yes] [--shell <s>] [--no-daemon]`

Interactive first-run wizard. Detects existing Claude Code install, imports `~/.claude.json`, configures shell, and optionally starts the daemon.

```bash
cst init                              # interactive
cst init --yes --shell zsh            # non-interactive, accept defaults
cst init --yes --shell zsh --no-daemon   # skip daemon start
```

## Environment Variables

| Variable | Set by | Purpose |
|----------|--------|---------|
| `CLAUDE_CONFIG_DIR` | `cst use` | Points Claude Code to per-session config dir |
| `CST_CURRENT` | `cst use` | Current `profile:session` (for prompts and scripts) |
| `ANTHROPIC_API_KEY` | `cst use` (api profiles) | API key loaded from Keychain |
| `AWS_ACCESS_KEY_ID` | `cst use` (bedrock) | AWS credential for Bedrock |
| `AWS_SECRET_ACCESS_KEY` | `cst use` (bedrock) | AWS credential for Bedrock |
| `AWS_DEFAULT_REGION` | `cst use` (bedrock) | AWS region |
| `ANTHROPIC_MODEL` | `cst use` (bedrock) | Bedrock model ID |
| `CLAUDE_CODE_USE_VERTEX` | `cst use` (vertex) | Enables Vertex AI mode |
| `ANTHROPIC_VERTEX_PROJECT_ID` | `cst use` (vertex) | GCP project ID |
| `CLOUD_ML_REGION` | `cst use` (vertex) | GCP region |
| `RUST_LOG` | User | Control log verbosity (e.g., `RUST_LOG=cst=debug`) |
