use anyhow::Result;
use cst_core::auto_switch::config::AutoSwitchConfig;
use cst_core::auto_switch::switch_log::SwitchLog;
use cst_core::platform;
use cst_core::GlobalConfig;

pub async fn dispatch(action: crate::AutoSwitchCommands) -> Result<()> {
    match action {
        crate::AutoSwitchCommands::Configure { profile } => configure(&profile),
        crate::AutoSwitchCommands::Log => log(),
        crate::AutoSwitchCommands::Test { profile } => test_chain(&profile),
    }
}

pub fn configure(profile: &str) -> Result<()> {
    cst_core::profile::validate_profile_name(profile)?;
    let profile_dir = platform::profile_dir(profile);
    if !profile_dir.exists() {
        anyhow::bail!("profile '{profile}' not found");
    }
    let cfg = AutoSwitchConfig::load(&profile_dir)?;
    println!("Configuring auto-switch for profile: {profile}");
    println!("  schedule.active_hours: {}", cfg
        .schedule
        .as_ref()
        .map(|s| s.active_hours.as_str())
        .unwrap_or("(unset)"));
    println!("  schedule.timezone: {}", cfg
        .schedule
        .as_ref()
        .map(|s| s.timezone.as_str())
        .unwrap_or("(unset)"));
    println!("  schedule.fallback: {}", cfg
        .schedule
        .as_ref()
        .map(|s| s.fallback.as_str())
        .unwrap_or("(unset)"));
    println!("  auto_switch_back: {}", cfg.auto_switch_back);
    println!();
    println!(
        "Note: profile switches are time-based only. Rate-limit signals do \
         not trigger switches."
    );
    if !profile_dir.join("auto-switch.toml").exists() {
        cfg.save(&profile_dir)?;
        println!(
            "Created {}/auto-switch.toml — edit it to configure your active_hours schedule.",
            profile_dir.display()
        );
    } else {
        println!(
            "Edit {}/auto-switch.toml to update settings.",
            profile_dir.display()
        );
    }
    Ok(())
}

pub fn log() -> Result<()> {
    let log = SwitchLog::open();
    let events = log.read_all()?;
    if events.is_empty() {
        println!("No auto-switch events recorded.");
        return Ok(());
    }
    println!(
        "{:<24} {:<20} {:<20} {}",
        "TIMESTAMP", "FROM", "TO", "REASON"
    );
    println!("{}", "─".repeat(80));
    for ev in &events {
        println!(
            "{:<24} {:<20} {:<20} {}",
            ev.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            format!("{}:{}", ev.from_profile, ev.from_session),
            format!("{}:{}", ev.to_profile, ev.to_session),
            format!("{} — {}", ev.reason, ev.detail),
        );
    }
    Ok(())
}

pub fn test_chain(profile: &str) -> Result<()> {
    cst_core::profile::validate_profile_name(profile)?;
    let profile_dir = platform::profile_dir(profile);
    if !profile_dir.exists() {
        anyhow::bail!("profile '{profile}' not found");
    }
    let cfg = AutoSwitchConfig::load(&profile_dir)?;
    println!("Dry-run auto-switch settings for: {profile}");
    println!("  auto_switch_back: {}", cfg.auto_switch_back);
    println!(
        "  note: fallback_chain is deprecated — rate-limit-triggered switches \
         have been removed. Use [schedule] for time-based switching."
    );
    if cfg.fallback_chain.is_empty() {
        println!("  fallback_chain: [empty]");
    } else {
        println!("  fallback_chain (deprecated, ignored by daemon):");
        let current = GlobalConfig::load()
            .map(|c| c.current_profile)
            .unwrap_or_default();
        for (i, p) in cfg.fallback_chain.iter().enumerate() {
            let marker = if p == &current { " ← current" } else { "" };
            let exists_mark = if platform::profile_dir(p).exists() {
                "✓"
            } else {
                "✗ NOT FOUND"
            };
            println!("    {}. {} {} {}", i + 1, p, exists_mark, marker);
        }
    }
    if let Some(sched) = &cfg.schedule {
        println!(
            "  schedule: {} ({}) → fallback: {}",
            sched.active_hours, sched.timezone, sched.fallback
        );
    }
    Ok(())
}

pub fn pause(minutes: Option<u64>) -> Result<()> {
    let pause_file = platform::data_dir().join("auto-switch-paused");
    if let Some(parent) = pause_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    match minutes {
        Some(m) => {
            // Cap to 1 year to prevent u64→i64 silent wrap producing a past timestamp.
            const MAX_PAUSE_MINUTES: u64 = 60 * 24 * 365;
            let capped = m.min(MAX_PAUSE_MINUTES) as i64;
            let resume_at = chrono::Utc::now() + chrono::Duration::minutes(capped);
            std::fs::write(&pause_file, resume_at.to_rfc3339())?;
            println!(
                "Auto-switch paused for {} minutes (resumes at {}).",
                capped,
                resume_at.format("%H:%M UTC")
            );
        }
        None => {
            std::fs::write(&pause_file, "indefinite")?;
            println!("Auto-switch paused indefinitely. Run `cst unpause` to resume.");
        }
    }
    Ok(())
}

pub fn unpause() -> Result<()> {
    let pause_file = platform::data_dir().join("auto-switch-paused");
    if pause_file.exists() {
        std::fs::remove_file(&pause_file)?;
        println!("Auto-switch resumed.");
    } else {
        println!("Auto-switch is not paused.");
    }
    Ok(())
}
