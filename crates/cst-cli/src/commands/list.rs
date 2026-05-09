use anyhow::Result;
use cst_core::profile::ProfileManager;
use cst_core::session::SessionManager;
use cst_core::GlobalConfig;

pub fn run() -> Result<()> {
    let mgr = ProfileManager::default();
    let profiles = mgr.list()?;
    let current = GlobalConfig::load().unwrap_or_default();

    if profiles.is_empty() {
        println!("No profiles. Run: cst new <name>");
        return Ok(());
    }

    for p in &profiles {
        let session_mgr = SessionManager::new(cst_core::platform::profile_dir(&p.name));
        let sessions = session_mgr.list().unwrap_or_default();
        let active = current.current_profile == p.name;
        let marker = if active { "▶" } else { " " };
        println!(
            "{marker} {name}  [{auth}]",
            name = p.name,
            auth = p.auth_type
        );
        for s in &sessions {
            let s_active = active && current.current_session == s.name;
            let s_marker = if s_active { "  ✓" } else { "   " };
            println!("{s_marker} {}", s.name);
        }
    }
    Ok(())
}
