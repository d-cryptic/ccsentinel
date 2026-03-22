//! Terminal integration output — Starship module and tmux status segment.

use anyhow::Result;
use cst_core::{auto_switch::scheduler::SchedulerState, config::GlobalConfig};

/// Print a Starship custom module config block that shows current profile + quota.
///
/// Add to `~/.config/starship.toml`:
/// ```toml
/// [custom.cst]
/// command = "cst starship"
/// when = true
/// format = "[$output]($style) "
/// style = "bold white"
/// ```
pub fn starship() -> Result<()> {
    let cfg = GlobalConfig::load().unwrap_or_default();

    if cfg.current_profile.is_empty() {
        // Nothing active — print nothing so the module is invisible
        return Ok(());
    }

    let quota_indicator = build_quota_indicator();
    let output = format!("🛡 {}:{}{}", cfg.current_profile, cfg.current_session, quota_indicator);
    print!("{}", output);
    Ok(())
}

/// Print a tmux status-bar segment showing current profile + quota status.
///
/// Add to `~/.config/tmux/tmux.conf`:
/// ```
/// set -g status-right "#(cst tmux) ..."
/// set -g status-interval 5
/// ```
pub fn tmux_segment() -> Result<()> {
    let cfg = GlobalConfig::load().unwrap_or_default();

    if cfg.current_profile.is_empty() {
        print!("#[fg=colour240]no profile#[default]");
        return Ok(());
    }

    let quota_indicator = build_quota_indicator();

    // tmux uses #[fg=...] for colour markup
    print!(
        "#[fg=colour255,bold]{}:{}#[default]{}",
        cfg.current_profile, cfg.current_session, quota_indicator
    );
    Ok(())
}

/// Print the Starship TOML config snippet (for `cst starship --config`).
pub fn starship_config() -> Result<()> {
    println!(
        r#"# Add this to ~/.config/starship.toml

[custom.cst]
command = "cst starship"
when = true
format = "[$output]($style) "
style = "bold white"
shell = ["sh"]
"#
    );
    Ok(())
}

/// Print the tmux config snippet (for `cst tmux --config`).
pub fn tmux_config() -> Result<()> {
    // Note: the time format uses strftime-style tokens which look like Rust prefixes.
    // We build the string at runtime to avoid the compiler treating %H:%M as a token.
    let snippet = concat!(
        "# Add this to ~/.config/tmux/tmux.conf\n\n",
        "set -g status-right \"#(cst tmux) | %",
        "H:%",
        "M\"\n",
        "set -g status-interval 5\n",
    );
    println!("{}", snippet);
    Ok(())
}

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Build a compact quota indicator string.
/// Returns "" if no rate limits are active.
/// Returns " ⚠ Xh Ym" if a rate-limit timer is running.
fn build_quota_indicator() -> String {
    if let Ok(sched) = SchedulerState::load() {
        let active: Vec<_> = sched.entries.iter().filter(|e| !e.switched_back).collect();
        if let Some(entry) = active.first() {
            let remaining = entry.time_until_refill();
            return format!(" ⚠ {}", remaining);
        }
    }
    String::new()
}
