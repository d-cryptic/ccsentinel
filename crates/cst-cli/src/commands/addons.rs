//! `cst addons` — manage the extensible addons system.
//!
//! Addons declare MCP servers and settings/hooks once and auto-apply them to
//! every profile/session. See `cst_core::addons`.

use anyhow::Result;
use cst_core::{addons, platform, profile::ProfileManager, session::SessionManager, GlobalConfig};
use std::collections::BTreeMap;

pub fn dispatch(action: crate::AddonCommands) -> Result<()> {
    match action {
        crate::AddonCommands::List => list(),
        crate::AddonCommands::Show { name } => show(&name),
        crate::AddonCommands::Init => init(),
        crate::AddonCommands::Enable { names } => enable(&names),
        crate::AddonCommands::Disable { names } => disable(&names),
        crate::AddonCommands::Apply => apply(),
    }
}

/// List available addons (built-in + on-disk), marking which are enabled.
pub fn list() -> Result<()> {
    let enabled = GlobalConfig::load().unwrap_or_default().addons_enabled;

    // Merge built-in and on-disk addons by name (on-disk wins for description).
    let mut by_name: BTreeMap<String, (String, bool)> = BTreeMap::new();
    for a in addons::builtin() {
        by_name.insert(a.name.clone(), (a.description, false));
    }
    for a in addons::load_all()? {
        by_name.insert(a.name.clone(), (a.description, true));
    }

    if by_name.is_empty() {
        println!("No addons available.");
        return Ok(());
    }

    for (name, (description, on_disk)) in &by_name {
        let is_enabled = enabled.iter().any(|n| n == name);
        let marker = if is_enabled { "✓" } else { " " };
        let source = if *on_disk { "file" } else { "builtin" };
        println!("[{marker}] {name:<12} ({source})  {description}");
    }
    Ok(())
}

/// Pretty-print an addon's JSON definition.
pub fn show(name: &str) -> Result<()> {
    let addon = addons::Addon::load(name)?;
    println!("{}", serde_json::to_string_pretty(&addon)?);
    Ok(())
}

/// Write built-in addon files to the claude-sentinel addons dir.
pub fn init() -> Result<()> {
    addons::ensure_builtins_written()?;
    println!(
        "✓ Wrote built-in addons to {}",
        platform::addons_dir().display()
    );
    Ok(())
}

/// Enable one or more addons and apply them to all sessions immediately.
pub fn enable(names: &[String]) -> Result<()> {
    let mut cfg = GlobalConfig::load().unwrap_or_default();
    for name in names {
        // Validate the addon resolves (file or built-in) before enabling.
        addons::Addon::load(name)?;
        if cfg.addons_enabled.iter().any(|n| n == name) {
            println!("• '{name}' already enabled");
        } else {
            cfg.enable_addon(name);
            println!("✓ Enabled '{name}'");
        }
    }
    cfg.save()?;
    let applied = apply_to_all(&cfg.addons_enabled)?;
    println!(
        "✓ Applied {} addon(s) to {applied} sessions",
        cfg.addons_enabled.len()
    );
    Ok(())
}

/// Disable one or more addons (removes from the enabled list).
pub fn disable(names: &[String]) -> Result<()> {
    let mut cfg = GlobalConfig::load().unwrap_or_default();
    for name in names {
        if cfg.addons_enabled.iter().any(|n| n == name) {
            cfg.disable_addon(name);
            println!("✓ Disabled '{name}'");
        } else {
            println!("• '{name}' was not enabled");
        }
    }
    cfg.save()?;
    println!(
        "Note: existing sessions keep already-applied addon entries until you remove\n      them manually or recreate the session. Disable only stops future application."
    );
    Ok(())
}

/// Apply all enabled addons to all profiles/sessions now.
pub fn apply() -> Result<()> {
    let enabled = GlobalConfig::load().unwrap_or_default().addons_enabled;
    if enabled.is_empty() {
        println!("No addons enabled. Enable one with: cst addons enable <name>");
        return Ok(());
    }
    let applied = apply_to_all(&enabled)?;
    println!(
        "✓ Applied {} addon(s) to {applied} sessions: {}",
        enabled.len(),
        enabled.join(", ")
    );
    Ok(())
}

/// Apply the enabled addons to every session across every profile.
/// Returns the number of sessions touched.
fn apply_to_all(enabled: &[String]) -> Result<usize> {
    if enabled.is_empty() {
        return Ok(0);
    }
    let mgr = ProfileManager::default();
    let mut count = 0;
    for p in mgr.list()? {
        let smgr = SessionManager::new(platform::profile_dir(&p.name));
        for s in smgr.list().unwrap_or_else(|e| {
            tracing::warn!("failed to list sessions for profile '{}': {e}", p.name);
            vec![]
        }) {
            let session_dir = platform::session_dir(&p.name, &s.name);
            if let Err(e) = addons::apply_to_session(enabled, &session_dir) {
                tracing::warn!("applying addons to {}:{} failed: {e}", p.name, s.name);
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}
