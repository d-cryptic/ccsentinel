//! Session management Tauri commands.

use cst_core::platform;
use cst_core::session::SessionManager;
use cst_core::{validate_profile_name, validate_session_name};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionDto {
    pub name: String,
    pub tag: String,
    pub archived: bool,
    pub last_used: Option<String>,
}

#[tauri::command]
pub fn list_sessions(profile: String) -> Result<Vec<SessionDto>, String> {
    validate_profile_name(&profile).map_err(|e| e.to_string())?;
    let profile_dir = platform::profile_dir(&profile);
    let mgr = SessionManager::new(profile_dir);
    let sessions = mgr.list().map_err(|e| e.to_string())?;
    Ok(sessions
        .into_iter()
        .map(|s| SessionDto {
            name: s.name,
            tag: s.description,
            archived: s.archived,
            last_used: s.last_used.map(|dt| dt.to_rfc3339()),
        })
        .collect())
}

#[tauri::command]
pub fn create_session(profile: String, name: String) -> Result<SessionDto, String> {
    validate_profile_name(&profile).map_err(|e| e.to_string())?;
    validate_session_name(&name).map_err(|e| e.to_string())?;
    let profile_dir = platform::profile_dir(&profile);
    let global_dir = platform::global_claude_dir();
    let mgr = SessionManager::new(profile_dir);
    let s = mgr.create(&name, &global_dir).map_err(|e| e.to_string())?;
    Ok(SessionDto {
        name: s.name,
        tag: s.description,
        archived: s.archived,
        last_used: s.last_used.map(|dt| dt.to_rfc3339()),
    })
}

#[tauri::command]
pub fn delete_session(profile: String, name: String) -> Result<(), String> {
    validate_profile_name(&profile).map_err(|e| e.to_string())?;
    validate_session_name(&name).map_err(|e| e.to_string())?;
    let profile_dir = platform::profile_dir(&profile);
    let mgr = SessionManager::new(profile_dir);
    mgr.delete(&name).map_err(|e| e.to_string())
}
