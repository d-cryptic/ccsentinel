//! TUI application state model.

use cst_core::auto_switch::scheduler::SchedulerState;
use cst_core::auto_switch::switch_log::SwitchLog;
use cst_core::config::GlobalConfig;
use cst_core::platform;
use cst_core::profile::ProfileManager;
use cst_core::session::SessionManager;

/// Which tab is currently selected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Profiles,
    Sessions,
    AutoSwitch,
    History,
}

impl Tab {
    pub fn all() -> &'static [Tab] {
        &[Tab::Profiles, Tab::Sessions, Tab::AutoSwitch, Tab::History]
    }

    pub fn title(&self) -> &'static str {
        match self {
            Tab::Profiles => "PROFILES",
            Tab::Sessions => "SESSIONS",
            Tab::AutoSwitch => "AUTO-SWITCH",
            Tab::History => "HISTORY",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Tab::Profiles => 0,
            Tab::Sessions => 1,
            Tab::AutoSwitch => 2,
            Tab::History => 3,
        }
    }
}

/// A profile row for display.
#[derive(Debug, Clone)]
pub struct ProfileRow {
    pub name: String,
    pub auth_type: String,
    pub sessions: Vec<String>,
    pub is_active: bool,
}

/// Application state.
pub struct AppState {
    pub tab: Tab,
    pub profiles: Vec<ProfileRow>,
    pub selected_profile: usize,
    pub selected_session: usize,
    pub current_profile: String,
    pub current_session: String,
    pub scheduler: SchedulerState,
    pub history_lines: Vec<String>,
    pub should_quit: bool,
    pub status_message: String,
}

impl AppState {
    /// Load all state from disk.
    pub fn load() -> Self {
        let cfg = GlobalConfig::load().unwrap_or_default();
        let profiles = load_profile_rows(&cfg);
        let history_lines = load_history_lines();
        let scheduler = SchedulerState::load().unwrap_or_default();

        Self {
            tab: Tab::Profiles,
            profiles,
            selected_profile: 0,
            selected_session: 0,
            current_profile: cfg.current_profile,
            current_session: cfg.current_session,
            scheduler,
            history_lines,
            should_quit: false,
            status_message: String::new(),
        }
    }

    /// Reload profiles from disk (after a switch).
    pub fn refresh(&mut self) {
        let cfg = GlobalConfig::load().unwrap_or_default();
        self.profiles = load_profile_rows(&cfg);
        self.current_profile = cfg.current_profile;
        self.current_session = cfg.current_session;
        self.history_lines = load_history_lines();
        self.scheduler = SchedulerState::load().unwrap_or_default();
    }

    /// Currently highlighted profile name.
    pub fn selected_profile_name(&self) -> Option<&str> {
        self.profiles.get(self.selected_profile).map(|p| p.name.as_str())
    }

    /// Currently highlighted sessions for selected profile.
    pub fn selected_profile_sessions(&self) -> &[String] {
        self.profiles
            .get(self.selected_profile)
            .map(|p| p.sessions.as_slice())
            .unwrap_or(&[])
    }

    pub fn next_tab(&mut self) {
        let idx = self.tab.index();
        self.tab = Tab::all()[(idx + 1) % Tab::all().len()].clone();
    }

    pub fn prev_tab(&mut self) {
        let idx = self.tab.index();
        let len = Tab::all().len();
        self.tab = Tab::all()[(idx + len - 1) % len].clone();
    }

    pub fn move_down(&mut self) {
        match self.tab {
            Tab::Profiles => {
                if !self.profiles.is_empty() {
                    self.selected_profile = (self.selected_profile + 1) % self.profiles.len();
                    self.selected_session = 0;
                }
            }
            Tab::Sessions => {
                let sessions = self.selected_profile_sessions();
                if !sessions.is_empty() {
                    self.selected_session = (self.selected_session + 1) % sessions.len();
                }
            }
            _ => {}
        }
    }

    pub fn move_up(&mut self) {
        match self.tab {
            Tab::Profiles => {
                if !self.profiles.is_empty() {
                    let len = self.profiles.len();
                    self.selected_profile = (self.selected_profile + len - 1) % len;
                    self.selected_session = 0;
                }
            }
            Tab::Sessions => {
                let len = self.selected_profile_sessions().len();
                if len > 0 {
                    self.selected_session = (self.selected_session + len - 1) % len;
                }
            }
            _ => {}
        }
    }
}

fn load_profile_rows(cfg: &GlobalConfig) -> Vec<ProfileRow> {
    let mgr = ProfileManager::new(platform::profiles_dir());
    let profiles = mgr.list().unwrap_or_default();
    profiles.into_iter().map(|p| {
        let profile_dir = platform::profile_dir(&p.name);
        let auth_type = format!("{}", p.auth_type);
        let smgr = SessionManager::new(profile_dir.join("sessions"));
        let sessions = smgr.list().unwrap_or_default()
            .into_iter().map(|s| s.name).collect();
        let is_active = p.name == cfg.current_profile;
        ProfileRow { name: p.name, auth_type, sessions, is_active }
    }).collect()
}

fn load_history_lines() -> Vec<String> {
    let log = SwitchLog::open();
    log.last_n(30).unwrap_or_default()
        .into_iter()
        .map(|ev| format!(
            "{} │ {}:{} → {}:{} │ {}",
            ev.timestamp.format("%m-%d %H:%M"),
            ev.from_profile, ev.from_session,
            ev.to_profile, ev.to_session,
            ev.reason,
        ))
        .collect()
}
