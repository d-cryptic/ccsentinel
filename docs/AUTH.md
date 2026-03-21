# Auth Types

Claude Sentinel supports 4 authentication methods for Claude Code.

## 1. OAuth (Claude Pro / Max subscription)

**How it works**: Claude Code reads `~/.claude.json` for OAuth tokens. Sentinel swaps this file (via symlink) to the profile's own `auth/oauth.json` on each profile switch.

**Create**:
```bash
cst new work --auth oauth
# Runs: claude /login in isolated profile environment
```

**Files**:
- `~/.claude-sentinel/profiles/work/auth/oauth.json` — OAuth tokens

**Switch mechanism**:
```
~/.claude.json  →  symlink  →  ~/.claude-sentinel/profiles/work/auth/oauth.json
```

## 2. API Key

**How it works**: Sets `ANTHROPIC_API_KEY` env var. Claude Code uses this if set (takes precedence over OAuth).

**Create**:
```bash
cst new api-work --auth api
cst add-key api-work            # prompts, stores in macOS Keychain
cst add-key api-work --slot 2   # second key in the rotation pool
```

**Storage**: macOS Keychain (preferred) → AES-GCM encrypted file (fallback)

**Multi-key rotation**: Profile stores a pool of keys. On rate limit, tries next key before switching profiles.

## 3. AWS Bedrock

**How it works**: Sets AWS credential env vars. Claude Code uses Bedrock when `AWS_*` vars are present.

**Create**:
```bash
cst new bedrock-work --auth bedrock
# Prompts for: AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_DEFAULT_REGION
```

**Env vars injected on activate**:
```
AWS_ACCESS_KEY_ID
AWS_SECRET_ACCESS_KEY
AWS_DEFAULT_REGION
AWS_SESSION_TOKEN     (optional)
ANTHROPIC_MODEL       (e.g., anthropic.claude-3-5-sonnet-20241022-v2:0)
```

**Files**: `~/.claude-sentinel/profiles/{name}/auth/aws.toml` (AES-GCM encrypted)

## 4. Google Vertex AI

**How it works**: Sets Vertex AI env vars. Claude Code uses Vertex when `CLAUDE_CODE_USE_VERTEX=1`.

**Create**:
```bash
cst new vertex-work --auth vertex
# Prompts for: project ID, region
```

**Env vars injected on activate**:
```
CLAUDE_CODE_USE_VERTEX=1
ANTHROPIC_VERTEX_PROJECT_ID
CLOUD_ML_REGION
GOOGLE_APPLICATION_CREDENTIALS   (path to service account JSON)
```

**Files**: `~/.claude-sentinel/profiles/{name}/auth/vertex.toml`

## Credential Security

| Platform | Storage Backend |
|----------|----------------|
| macOS | Keychain Services |
| Linux | libsecret (GNOME) / KWallet (KDE) |
| Windows | Windows Credential Manager |
| Fallback | AES-256-GCM encrypted file |

Keys are never stored in plaintext. The AES key for the fallback is derived from a machine-specific secret.
