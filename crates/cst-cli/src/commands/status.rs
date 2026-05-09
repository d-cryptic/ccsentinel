use anyhow::Result;
use cst_core::{platform, validate_profile_name, validate_session_name, GlobalConfig};

pub fn run() -> Result<()> {
    let cfg = GlobalConfig::load()?;
    if cfg.current_profile.is_empty() {
        println!("No active profile. Run: cst use <profile>");
        return Ok(());
    }
    validate_profile_name(&cfg.current_profile)?;
    validate_session_name(&cfg.current_session)?;
    let profile_dir = platform::profile_dir(&cfg.current_profile);
    let profile_toml = profile_dir.join("profile.toml");
    let auth_type = if profile_toml.exists() {
        let contents = std::fs::read_to_string(&profile_toml)?;
        let p: cst_core::profile::Profile = toml::from_str(&contents)?;
        p.auth_type.to_string()
    } else {
        "unknown".to_string()
    };
    println!("Profile : {}", cfg.current_profile);
    println!("Session : {}", cfg.current_session);
    println!("Auth    : {auth_type}");
    println!(
        "Config  : {}",
        platform::claude_config_dir(&cfg.current_profile, &cfg.current_session).display()
    );
    Ok(())
}
