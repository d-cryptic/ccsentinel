use anyhow::Result;
use cst_core::profile::{AuthType, ProfileManager};
use cst_core::auth::oauth;
use cst_core::platform;
use std::str::FromStr;

pub async fn new(name: &str, auth: &str, template: Option<&str>) -> Result<()> {
    let auth_type = AuthType::from_str(auth)?;
    let mgr = ProfileManager::default();
    let profile = mgr.create(name, auth_type.clone())?;
    println!("✓ Created profile '{name}' [{auth_type}]");

    // Apply template settings if specified
    if let Some(tmpl_name) = template {
        if let Some(tmpl) = cst_core::templates::find(tmpl_name) {
            let override_path = platform::profile_dir(name).join("settings-override.json");
            std::fs::write(&override_path, serde_json::to_string_pretty(&tmpl.settings_override)?)?;
            println!("✓ Applied template '{tmpl_name}'");
        } else {
            eprintln!("⚠ Template '{tmpl_name}' not found. Run: cst templates");
        }
    }

    // Create default session with symlinks
    let session_mgr = cst_core::session::SessionManager::new(platform::profile_dir(name));
    let global = platform::global_claude_dir();
    session_mgr.create("default", &global)?;
    println!("✓ Created default session");

    if matches!(auth_type, cst_core::profile::AuthType::OAuth) {
        println!("\nNext: run `cst login {name}` to authenticate");
    }
    Ok(())
}

pub fn import(alias: Option<&str>) -> Result<()> {
    let name = alias.unwrap_or("imported");
    let mgr = ProfileManager::default();
    let _ = mgr.create(name, cst_core::profile::AuthType::OAuth)?;
    let auth_dir = platform::profile_dir(name).join("auth");
    oauth::import_current(&auth_dir)?;

    let session_mgr = cst_core::session::SessionManager::new(platform::profile_dir(name));
    session_mgr.create("default", &platform::global_claude_dir())?;

    println!("✓ Imported current ~/.claude.json as profile '{name}'");
    println!("  Run: cst use {name}");
    Ok(())
}

pub fn clone(src: &str, dst: &str) -> Result<()> {
    ProfileManager::default().clone_profile(src, dst)?;
    println!("✓ Cloned '{src}' → '{dst}'");
    Ok(())
}

pub fn remove(name: &str) -> Result<()> {
    ProfileManager::default().delete(name)?;
    println!("✓ Deleted profile '{name}'");
    Ok(())
}

pub fn rename(old: &str, new: &str) -> Result<()> {
    ProfileManager::default().rename(old, new)?;
    println!("✓ Renamed '{old}' → '{new}'");
    Ok(())
}

pub async fn login(profile: Option<&str>) -> Result<()> {
    let name = profile.unwrap_or("default");
    println!("Starting OAuth login for profile '{name}'...");
    // Activate the profile's CLAUDE_CONFIG_DIR, then run `claude /login`
    let config_dir = platform::claude_config_dir(name, "default");
    std::fs::create_dir_all(&config_dir)?;
    let status = tokio::process::Command::new("claude")
        .arg("/login")
        .env("CLAUDE_CONFIG_DIR", &config_dir)
        .status()
        .await?;
    if status.success() {
        // Copy the newly-written ~/.claude.json to the profile's auth dir
        let auth_dir = platform::profile_dir(name).join("auth");
        oauth::import_current(&auth_dir)?;
        println!("✓ Login successful for '{name}'");
    } else {
        eprintln!("✗ Login failed");
    }
    Ok(())
}

pub fn add_key(profile: &str, slot: u8) -> Result<()> {
    use cst_core::auth::apikey::ApiKeyPool;
    print!("Enter API key for '{profile}' slot {slot}: ");
    let key = rpassword_read()?;
    let keys_path = platform::profile_dir(profile).join("auth").join("api_keys.toml");
    let mut pool: ApiKeyPool = if keys_path.exists() {
        let c = std::fs::read_to_string(&keys_path)?;
        toml::from_str(&c)?
    } else {
        ApiKeyPool::default()
    };
    pool.add_key(profile, slot, key.trim(), "")?;
    std::fs::write(&keys_path, toml::to_string_pretty(&pool)?)?;
    println!("✓ Stored key in slot {slot} for '{profile}'");
    Ok(())
}

fn rpassword_read() -> Result<String> {
    // Simple stdin read (no echo hiding in this stub — add rpassword crate later)
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    Ok(stdin.lock().lines().next().unwrap_or(Ok(String::new()))?)
}
