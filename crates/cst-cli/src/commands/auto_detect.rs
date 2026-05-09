//! `cst _auto-detect <dir> <current_profile_session>` — hidden command.
//!
//! Called by the shell precmd hook. Checks for a `.cstrc` in the current
//! directory tree and prints env exports if a different profile should be active.

use anyhow::Result;
use cst_core::{auto_detect, config::GlobalConfig};

/// Hidden command used by the shell precmd hook.
///
/// Prints nothing if the active profile already matches what `.cstrc` requests.
/// Prints env exports (same format as `cst _env`) when a switch is needed.
pub fn check(dir: &str, current: &str) -> Result<()> {
    let path = std::path::Path::new(dir);

    let Some(detected) = auto_detect::detect(path) else {
        return Ok(());
    };

    let target_session = detected.session.as_deref().unwrap_or("default");
    let target_ps = format!("{}:{}", detected.profile, target_session);

    // Only emit exports when the target differs from what is currently active.
    if current == target_ps {
        return Ok(());
    }

    // Print a comment so the user sees what triggered the switch.
    eprintln!("⚡ cst: auto-detect → {}", target_ps);

    // Delegate to the shell _env machinery.
    crate::commands::shell::env_cmd(&target_ps)
}

/// `cst auto-detect status` — show what would be activated in `<dir>`.
pub fn status(dir: &str) -> Result<()> {
    let path = std::path::Path::new(dir);
    match auto_detect::detect(path) {
        None => {
            println!("No .cstrc found in {} or any parent directory.", dir);
        }
        Some(r) => {
            let session = r.session.as_deref().unwrap_or("default");
            println!("Profile : {}:{}", r.profile, session);
            match &r.source {
                auto_detect::DetectSource::CstRc(p) => {
                    println!("Source  : .cstrc at {}", p.display());
                }
                auto_detect::DetectSource::GitRemote { pattern } => {
                    println!("Source  : git remote matched pattern \"{}\"", pattern);
                }
            }

            let cfg = GlobalConfig::load().unwrap_or_default();
            let current = format!("{}:{}", cfg.current_profile, cfg.current_session);
            if current == format!("{}:{}", r.profile, session) {
                println!("Status  : already active");
            } else {
                println!(
                    "Status  : would switch from {} → {}:{}",
                    current, r.profile, session
                );
            }
        }
    }
    Ok(())
}
