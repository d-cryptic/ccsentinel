use anyhow::Result;
use cst_core::{platform, profile::ProfileManager, session::SessionManager};

pub fn run() -> Result<()> {
    let mut ok = true;
    let global = platform::global_claude_dir();
    if !global.exists() {
        eprintln!("✗ ~/.claude/ not found — is Claude Code installed?");
        ok = false;
    } else {
        println!("✓ ~/.claude/ exists");
    }

    let mgr = ProfileManager::default();
    let profiles = mgr.list()?;
    for p in &profiles {
        let smgr = SessionManager::new(platform::profile_dir(&p.name));
        for s in smgr.list().unwrap_or_default() {
            let claude_dir = platform::claude_config_dir(&p.name, &s.name);
            if claude_dir.exists() {
                println!("✓ {}:{} — config dir OK", p.name, s.name);
            } else {
                eprintln!("✗ {}:{} — config dir missing (run: cst sync)", p.name, s.name);
                ok = false;
            }
        }
    }
    if ok { println!("\n✓ All checks passed"); } else { eprintln!("\n✗ Some checks failed"); }
    Ok(())
}

pub fn validate(profile: &str) -> Result<()> {
    let mgr = ProfileManager::default();
    let p = mgr.load(profile)?;
    println!("Profile : {}", p.name);
    println!("Auth    : {}", p.auth_type);
    println!("Created : {}", p.created_at);
    println!("✓ Profile config is valid");
    Ok(())
}
