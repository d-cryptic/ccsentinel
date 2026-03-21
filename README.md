# 🛡 Claude Sentinel

> Intelligent Claude Code account, profile, and session manager

**`cst`** — the CLI tool that manages multiple Claude Code accounts, automatically switches profiles on rate limits, and isolates your sessions.

## Features

- **All auth types**: Claude Pro/Max (OAuth), API key, AWS Bedrock, Google Vertex AI
- **Multi-key rotation**: Exhausts API key pool before switching profiles
- **Auto-switch on rate limits**: Automatically falls back to configured profile when quota exhausted
- **Auto-switch back**: Schedules return to primary profile when quota refills
- **Time-based switching**: "Use work profile 9am-6pm, personal otherwise"
- **Per-profile sessions**: Isolated conversation history, settings, and MCP config
- **3-layer settings merge**: Global base + profile overrides + session overrides
- **Shared global config**: agents, rules, skills, commands auto-symlinked to all sessions
- **Beautiful TUI**: Interactive profile/session navigator
- **Desktop app**: Menu bar (macOS) / system tray — neubrutalism B&W design
- **Cross-platform**: macOS, Linux, Windows

## Install

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/d-cryptic/claude-sentinel/main/install.sh | sh

# Cargo
cargo install cst-cli

# First run (imports existing Claude Code config)
cst init
```

## Quick Start

```bash
# Create profiles
cst new personal --auth oauth    # Claude subscription
cst new work --auth oauth        # Another subscription
cst new api-acct --auth api      # API key

# Switch
cst use work
cst use work:backend             # named session

# Auto-switch on rate limits
cst auto-switch configure work   # set fallback chain
cst daemon start

# Check quota
cst remaining

# Interactive TUI
cst
```

## Documentation

- [Install Guide](docs/INSTALL.md)
- [Usage Reference](docs/USAGE.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Auth Types](docs/AUTH.md)
- [Auto-Switch](docs/AUTO-SWITCH.md)

## License

MIT
