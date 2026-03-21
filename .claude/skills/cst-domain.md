---
name: cst-domain
description: Domain model and concepts for claude-sentinel — Profile, Session, Auth, Daemon
type: reference
---

# claude-sentinel Domain Model

## Core Concepts

**Profile**: An account identity with a specific auth type and optional settings overrides.
- Has a name (slug), auth type, description, color label, created_at timestamp
- Contains one or more Sessions
- Has profile-level settings-override.json and auto-switch.toml

**Session**: An isolated Claude Code workspace within a Profile.
- Has its own `CLAUDE_CONFIG_DIR` target directory
- Has its own project conversation history and history.jsonl
- Has session-level settings-override.json
- Shares global agents/rules/skills/commands via symlinks

**Auth types**:
- `oauth` — Claude Pro/Max subscription (OAuth tokens in `~/.claude.json`)
- `api` — ANTHROPIC_API_KEY (stored in OS Keychain or encrypted file)
- `bedrock` — AWS Bedrock (AWS_ACCESS_KEY_ID etc.)
- `vertex` — Google Vertex AI (CLOUD_ML_REGION, ANTHROPIC_VERTEX_PROJECT_ID)

**Data directory**: `~/.claude-sentinel/` (platform-resolved via `dirs::data_dir()`)

**Current state**: `~/.claude-sentinel/config.toml` — which profile:session is active

## Key Paths

```rust
// In cst-core::config
fn data_dir() -> PathBuf  // ~/.claude-sentinel/
fn profiles_dir() -> PathBuf  // ~/.claude-sentinel/profiles/
fn profile_dir(name: &str) -> PathBuf
fn session_dir(profile: &str, session: &str) -> PathBuf
fn claude_config_dir(profile: &str, session: &str) -> PathBuf  // CLAUDE_CONFIG_DIR target
fn global_claude_dir() -> PathBuf  // ~/.claude/ (shared base)
fn global_claude_json() -> PathBuf  // ~/.claude.json (OAuth creds)
```

## Settings Merge Order

1. `global_claude_dir()/settings.json` — base (never modified)
2. `profile_dir(p)/settings-override.json` — profile level
3. `session_dir(p, s)/settings-override.json` — session level
Output: `claude_config_dir(p, s)/settings.json`

## Shared Symlinks (per session)

These dirs in each session's `.claude/` are symlinked to `~/.claude/`:
- `agents/` → `~/.claude/agents/`
- `rules/` → `~/.claude/rules/`
- `skills/` → `~/.claude/skills/`
- `commands/` → `~/.claude/commands/`
- `hooks.json` → `~/.claude/hooks.json`
- `statusline.sh` → `~/.claude/statusline.sh`
- `CLAUDE.md` → `~/.claude/CLAUDE.md`
