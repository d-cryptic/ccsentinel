//! Shell integration — generate `shell-init` code and `_env` exports.

use std::collections::HashMap;

/// Shell types supported for init code generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellKind {
    Zsh,
    Bash,
    Fish,
    PowerShell,
}

impl ShellKind {
    /// Detect shell from `$SHELL` env var.
    pub fn detect() -> Self {
        let shell = std::env::var("SHELL").unwrap_or_default();
        if shell.contains("zsh") {
            Self::Zsh
        } else if shell.contains("fish") {
            Self::Fish
        } else if shell.contains("bash") {
            Self::Bash
        } else {
            Self::Bash // safe default on Unix
        }
    }
}

/// Generate the shell init code (emitted by `cst shell-init`).
/// The user adds `eval "$(cst shell-init)"` to their rc file.
pub fn shell_init_code(shell: &ShellKind) -> String {
    match shell {
        ShellKind::Zsh | ShellKind::Bash => {
            r#"
# claude-sentinel shell integration
cst() {
    case "$1" in
        use)
            if [ -z "$2" ]; then
                command cst tui
            else
                eval "$(command cst _env "$2" 2>&1)"
            fi
            ;;
        switch-all)
            # switch-all also switches the current shell immediately
            command cst switch-all "$2" "$3"
            if [ -n "$3" ]; then
                eval "$(command cst _env "${3}:${CST_CURRENT#*:}" 2>&1)"
            fi
            ;;
        *)
            command cst "$@"
            ;;
    esac
}

# Auto-switch check: runs before each prompt
_cst_check_switch() {
    # 1. One-shot pending switch (daemon-initiated for this specific shell)
    local switch_file="${HOME}/.claude-sentinel/pending-switch"
    if [ -f "$switch_file" ]; then
        eval "$(cat "$switch_file")" 2>/dev/null
        rm -f "$switch_file"
        printf '⚡ claude-sentinel: switched to %s\n' "$CST_CURRENT" >&2
    fi

    # 2. Broadcast switch (switch-all — applies to all shells running the from-profile)
    if [ -n "${CST_CURRENT:-}" ]; then
        local _cst_bc
        _cst_bc="$(command cst _broadcast-switch "${CST_CURRENT}" "${CST_BROADCAST_ID:-}" 2>/dev/null)"
        if [ -n "$_cst_bc" ]; then
            eval "$_cst_bc"
            printf '⚡ claude-sentinel: broadcast → %s\n' "$CST_CURRENT" >&2
        fi
    fi

    # 3. .cstrc auto-detect (direnv-style per-project profile selection)
    local _cst_ad
    _cst_ad="$(command cst _auto-detect "${PWD}" "${CST_CURRENT:-}" 2>/dev/null)"
    if [ -n "$_cst_ad" ]; then
        eval "$_cst_ad"
    fi
}

if [ -n "$ZSH_VERSION" ]; then
    precmd_functions+=(_cst_check_switch)
elif [ -n "$BASH_VERSION" ]; then
    PROMPT_COMMAND="${PROMPT_COMMAND:+${PROMPT_COMMAND}; }_cst_check_switch"
fi
"#
            .trim()
            .to_string()
        }
        ShellKind::Fish => {
            r#"
# claude-sentinel shell integration (fish)
function cst
    if test "$argv[1]" = "use"
        if test -z "$argv[2]"
            command cst tui
        else
            eval (command cst _env "$argv[2]" 2>&1)
        end
    else
        command cst $argv
    end
end

function _cst_check_switch --on-event fish_prompt
    set switch_file "$HOME/.claude-sentinel/pending-switch"
    if test -f $switch_file
        eval (cat $switch_file) 2>/dev/null
        rm -f $switch_file
        echo "⚡ claude-sentinel: switched to $CST_CURRENT" >&2
    end
    if test -n "$CST_CURRENT"
        set _cst_bc (command cst _broadcast-switch "$CST_CURRENT" "$CST_BROADCAST_ID" 2>/dev/null)
        if test -n "$_cst_bc"
            eval $_cst_bc
            echo "⚡ claude-sentinel: broadcast → $CST_CURRENT" >&2
        end
    end
    set _cst_ad (command cst _auto-detect "$PWD" "$CST_CURRENT" 2>/dev/null)
    if test -n "$_cst_ad"
        eval $_cst_ad
    end
end
"#
            .trim()
            .to_string()
        }
        ShellKind::PowerShell => {
            r#"
# claude-sentinel shell integration (PowerShell)
function cst {
    if ($args[0] -eq "use") {
        if (-not $args[1]) {
            & cst.exe tui
        } else {
            Invoke-Expression (& cst.exe _env $args[1] 2>&1)
        }
    } else {
        & cst.exe @args
    }
}
"#
            .trim()
            .to_string()
        }
    }
}

/// Escape a value for use inside single-quoted POSIX shell strings.
///
/// Replaces `'` with `'\''` (close quote, escaped literal apostrophe, reopen quote).
/// Safe for bash, zsh, and fish (fish interprets `\'` outside quotes as a literal `'`).
pub(crate) fn shell_escape_single_quote(val: &str) -> String {
    val.replace('\'', r"'\''")
}

/// Escape a value for use inside PowerShell single-quoted strings.
/// Replaces `'` with `''` (PowerShell doubling convention).
fn powershell_escape_single_quote(val: &str) -> String {
    val.replace('\'', "''")
}

/// Returns `true` if `key` is a valid POSIX environment variable name.
/// POSIX names match `[A-Za-z_][A-Za-z0-9_]*`.
pub(crate) fn is_valid_env_key(key: &str) -> bool {
    let mut chars = key.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Generate env export lines (emitted by `cst _env <profile:session>`).
/// The shell function eval's this output.
///
/// An empty value (`""`) is treated as a signal to **unset** the variable
/// rather than export it as empty. This is used for `ANTHROPIC_API_KEY` when
/// activating an OAuth profile, ensuring the key is fully absent from the
/// Claude Code process rather than set to an empty string (which `std::env::var`
/// would still return as `Ok("")`).
pub fn env_exports(env_vars: &HashMap<String, String>, shell: &ShellKind) -> String {
    let mut lines = Vec::new();
    for (key, val) in env_vars {
        let line = if val.is_empty() {
            // Unset the variable so it is fully absent from the child process.
            match shell {
                ShellKind::Zsh | ShellKind::Bash => format!("unset {key}"),
                ShellKind::Fish => format!("set -e {key}"),
                ShellKind::PowerShell => format!("Remove-Item Env:{key} -ErrorAction SilentlyContinue"),
            }
        } else {
            match shell {
                ShellKind::Zsh | ShellKind::Bash => {
                    format!("export {key}='{}'", shell_escape_single_quote(val))
                }
                ShellKind::Fish => {
                    format!("set -gx {key} '{}'", shell_escape_single_quote(val))
                }
                ShellKind::PowerShell => {
                    format!("$env:{key} = '{}'", powershell_escape_single_quote(val))
                }
            }
        };
        lines.push(line);
    }
    lines.sort(); // deterministic output
    lines.join("\n")
}

/// Parse `"profile:session"` into `(profile, session)`.
/// If no `:` is present, session defaults to `"default"`.
pub fn parse_profile_session(input: &str) -> (String, String) {
    match input.split_once(':') {
        Some((p, s)) => (p.to_string(), s.to_string()),
        None => (input.to_string(), "default".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_profile_session_with_colon() {
        let (p, s) = parse_profile_session("work:backend");
        assert_eq!(p, "work");
        assert_eq!(s, "backend");
    }

    #[test]
    fn test_parse_profile_session_without_colon() {
        let (p, s) = parse_profile_session("personal");
        assert_eq!(p, "personal");
        assert_eq!(s, "default");
    }

    #[test]
    fn test_env_exports_bash_format() {
        let mut vars = HashMap::new();
        vars.insert(
            "CLAUDE_CONFIG_DIR".to_string(),
            "/home/user/.claude-sentinel/...".to_string(),
        );
        let output = env_exports(&vars, &ShellKind::Bash);
        assert!(output.contains("export CLAUDE_CONFIG_DIR="));
    }

    #[test]
    fn test_env_exports_fish_format() {
        let mut vars = HashMap::new();
        vars.insert("CST_CURRENT".to_string(), "work:backend".to_string());
        let output = env_exports(&vars, &ShellKind::Fish);
        assert!(output.contains("set -gx CST_CURRENT"));
    }

    #[test]
    fn test_shell_init_code_contains_function() {
        let code = shell_init_code(&ShellKind::Zsh);
        assert!(code.contains("function cst") || code.contains("cst()"));
        assert!(code.contains("_cst_check_switch"));
    }

    #[test]
    fn test_env_exports_empty_value_emits_unset_bash() {
        let mut vars = HashMap::new();
        vars.insert("ANTHROPIC_API_KEY".to_string(), String::new());
        let output = env_exports(&vars, &ShellKind::Bash);
        assert_eq!(output, "unset ANTHROPIC_API_KEY");
        assert!(!output.contains("export"), "empty value must not export");
    }

    #[test]
    fn test_env_exports_empty_value_emits_unset_fish() {
        let mut vars = HashMap::new();
        vars.insert("ANTHROPIC_API_KEY".to_string(), String::new());
        let output = env_exports(&vars, &ShellKind::Fish);
        assert_eq!(output, "set -e ANTHROPIC_API_KEY");
    }

    #[test]
    fn test_env_exports_empty_value_emits_unset_powershell() {
        let mut vars = HashMap::new();
        vars.insert("ANTHROPIC_API_KEY".to_string(), String::new());
        let output = env_exports(&vars, &ShellKind::PowerShell);
        assert!(output.contains("Remove-Item Env:ANTHROPIC_API_KEY"));
    }

    #[test]
    fn test_shell_escape_no_quotes() {
        assert_eq!(shell_escape_single_quote("hello"), "hello");
        assert_eq!(
            shell_escape_single_quote("/home/user/.config"),
            "/home/user/.config"
        );
    }

    #[test]
    fn test_shell_escape_single_quote() {
        assert_eq!(shell_escape_single_quote("it's"), r"it'\''s");
    }

    #[test]
    fn test_env_exports_escapes_single_quotes_bash() {
        let mut vars = HashMap::new();
        vars.insert("MY_VAR".to_string(), "it's a test".to_string());
        let output = env_exports(&vars, &ShellKind::Bash);
        assert!(
            output.contains(r"'\''"),
            "should contain escaped single quote"
        );
        assert!(
            !output.contains("'it's"),
            "must not contain raw unescaped quote"
        );
    }

    #[test]
    fn test_env_exports_escapes_single_quotes_powershell() {
        let mut vars = HashMap::new();
        vars.insert("MY_VAR".to_string(), "it's a test".to_string());
        let output = env_exports(&vars, &ShellKind::PowerShell);
        assert!(
            output.contains("it''s"),
            "should use PowerShell double-quote escaping"
        );
    }

    #[test]
    fn test_env_exports_escapes_single_quotes_fish() {
        let mut vars = HashMap::new();
        vars.insert("MY_VAR".to_string(), "it's a test".to_string());
        let output = env_exports(&vars, &ShellKind::Fish);
        assert!(
            output.contains(r"'\''"),
            "fish should also use the POSIX '\\'' escape"
        );
        assert!(
            !output.contains("'it's"),
            "must not contain raw unescaped quote"
        );
    }

    #[test]
    fn test_is_valid_env_key_accepts_valid_names() {
        assert!(is_valid_env_key("FOO"));
        assert!(is_valid_env_key("_BAR"));
        assert!(is_valid_env_key("CLAUDE_CODE_MAX_OUTPUT_TOKENS"));
        assert!(is_valid_env_key("foo123"));
    }

    #[test]
    fn test_is_valid_env_key_rejects_invalid_names() {
        assert!(!is_valid_env_key(""));
        assert!(!is_valid_env_key("1FOO"));
        assert!(!is_valid_env_key("FOO=bar;id"));
        assert!(!is_valid_env_key("FOO BAR"));
        assert!(!is_valid_env_key("FOO$"));
    }
}
