use anyhow::Result;
use cst_core::{auth::oauth, platform};

pub async fn run(yes: bool, shell: Option<&str>, start_daemon: bool) -> Result<()> {
    println!("┌─────────────────────────────────────────────┐");
    println!("│  CLAUDE SENTINEL — FIRST RUN SETUP          │");
    println!("└─────────────────────────────────────────────┘");
    println!();

    // Step 1: Init data directory
    platform::ensure_dirs_exist()?;
    println!("[1/4] Initialised ~/.claude-sentinel/  ✓");

    // Step 2: Import existing ~/.claude.json if present
    let global_json = platform::global_claude_json();
    if global_json.exists() && !global_json.is_symlink() {
        let mgr = cst_core::profile::ProfileManager::default();
        if !mgr.exists("default") {
            let _ = mgr.create("default", cst_core::profile::AuthType::OAuth);
            let auth_dir = platform::profile_dir("default").join("auth");
            oauth::import_current(&auth_dir)?;
            let smgr = cst_core::session::SessionManager::new(platform::profile_dir("default"));
            let _ = smgr.create("default", &platform::global_claude_dir());
        }
        println!("[2/4] Imported existing ~/.claude.json as profile 'default'  ✓");
    } else {
        println!("[2/4] No existing ~/.claude.json found — create profiles with: cst new <name>");
    }

    // Step 3: Shell init
    let shell_name = shell.unwrap_or_else(|| {
        let s = std::env::var("SHELL").unwrap_or_default();
        if s.contains("zsh") {
            "zsh"
        } else if s.contains("fish") {
            "fish"
        } else {
            "bash"
        }
    });
    println!("[3/4] Shell init snippet (add to ~/.{shell_name}rc):");
    println!("      eval \"$(cst shell-init)\"");

    // Step 4: Daemon
    if start_daemon && yes {
        println!("[4/4] Daemon: run `cst daemon start` to enable auto-switching");
    } else {
        println!("[4/4] Skipping daemon (run `cst daemon start` when ready)");
    }

    println!();
    println!("✓ Setup complete!");
    println!("  Run: cst list");
    Ok(())
}
