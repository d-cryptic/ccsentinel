use anyhow::Result;
use cst_core::{platform, session::SessionManager, GlobalConfig};

pub async fn dispatch(action: crate::SessionCommands) -> Result<()> {
    match action {
        crate::SessionCommands::New { name, tag } => new(&name, tag.as_deref()),
        crate::SessionCommands::List { profile } => list(profile.as_deref()),
        crate::SessionCommands::Rm { name } => remove(&name),
        crate::SessionCommands::Tag { name, description } => tag(&name, &description),
        crate::SessionCommands::Archive { name } => archive(&name),
    }
}

fn current_profile() -> Result<String> {
    let cfg = GlobalConfig::load()?;
    if cfg.current_profile.is_empty() {
        anyhow::bail!("No active profile. Run: cst use <profile>");
    }
    Ok(cfg.current_profile)
}

pub fn new(name: &str, tag: Option<&str>) -> Result<()> {
    let profile = current_profile()?;
    let mgr = SessionManager::new(platform::profile_dir(&profile));
    let session = mgr.create(name, &platform::global_claude_dir())?;
    if let Some(desc) = tag {
        mgr.tag(name, desc)?;
    }
    println!("✓ Created session '{name}' in profile '{profile}'");
    Ok(())
}

pub fn list(profile: Option<&str>) -> Result<()> {
    let profile = match profile {
        Some(p) => p.to_string(),
        None => current_profile()?,
    };
    let mgr = SessionManager::new(platform::profile_dir(&profile));
    let sessions = mgr.list()?;
    let current = GlobalConfig::load().unwrap_or_default();
    for s in &sessions {
        let active = current.current_profile == profile && current.current_session == s.name;
        let marker = if active { "✓" } else { " " };
        let tag = if s.description.is_empty() { String::new() } else { format!(" — {}", s.description) };
        println!("[{marker}] {}{tag}", s.name);
    }
    Ok(())
}

pub fn remove(name: &str) -> Result<()> {
    let profile = current_profile()?;
    SessionManager::new(platform::profile_dir(&profile)).delete(name)?;
    println!("✓ Deleted session '{name}'");
    Ok(())
}

pub fn tag(name: &str, description: &str) -> Result<()> {
    let profile = current_profile()?;
    SessionManager::new(platform::profile_dir(&profile)).tag(name, description)?;
    println!("✓ Tagged '{name}': {description}");
    Ok(())
}

pub fn archive(name: &str) -> Result<()> {
    let profile = current_profile()?;
    SessionManager::new(platform::profile_dir(&profile)).archive(name)?;
    println!("✓ Archived '{name}'");
    Ok(())
}
