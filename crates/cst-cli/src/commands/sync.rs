use anyhow::Result;
use cst_core::{platform, profile::ProfileManager, session::SessionManager};

pub fn run() -> Result<()> {
    let global_dir = platform::global_claude_dir();
    let mgr = ProfileManager::default();
    let profiles = mgr.list()?;
    let mut count = 0;
    for p in &profiles {
        let smgr = SessionManager::new(platform::profile_dir(&p.name));
        for s in smgr.list().unwrap_or_default() {
            smgr.sync_symlinks(&s.name, &global_dir)?;
            count += 1;
        }
    }
    println!("✓ Synced symlinks for {count} sessions");
    Ok(())
}
