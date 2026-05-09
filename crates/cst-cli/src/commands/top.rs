//! `cst top` — live real-time dashboard (htop-style) for Claude usage.
//!
//! Refreshes every second and shows:
//!  - Active profile:session with auth type
//!  - Quota / rate-limit status per profile
//!  - Token counters (in / out) and estimated cost
//!  - Auto-switch scheduler entries with countdown timers
//!  - Daemon status

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use cst_core::{
    auto_switch::{daemon as daemon_core, scheduler::SchedulerState, switch_log::SwitchLog},
    config::GlobalConfig,
    platform,
    profile::ProfileManager,
    session::SessionManager,
    stats::SessionStats,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame, Terminal,
};
use std::{io, time::Duration};

// ─── Data Model ────────────────────────────────────────────────────────────

struct TopState {
    should_quit: bool,
    /// (profile_name, session_name, stats)
    rows: Vec<ProfileRow>,
    scheduler: SchedulerState,
    recent_events: Vec<String>,
    daemon_running: bool,
    active_profile: String,
    active_session: String,
    tick: u64,
}

struct ProfileRow {
    profile: String,
    session: String,
    auth_type: String,
    tokens_in: u64,
    tokens_out: u64,
    rate_limit_hits: u64,
    cost_usd: f64,
    last_used: String,
}

// ─── Entry Point ────────────────────────────────────────────────────────────

pub async fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut state = TopState::load();
    let mut last_refresh = std::time::Instant::now();

    loop {
        terminal.draw(|f| render(f, &state))?;

        // Refresh data every second
        if last_refresh.elapsed() >= Duration::from_secs(1) {
            state.refresh();
            last_refresh = std::time::Instant::now();
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        state.should_quit = true;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.should_quit = true;
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        state.refresh();
                    }
                    _ => {}
                }
            }
        }

        if state.should_quit {
            break;
        }
    }

    Ok(())
}

// ─── State Loading ──────────────────────────────────────────────────────────

impl TopState {
    fn load() -> Self {
        let mut s = Self {
            should_quit: false,
            rows: Vec::new(),
            scheduler: SchedulerState::default(),
            recent_events: Vec::new(),
            daemon_running: false,
            active_profile: String::new(),
            active_session: String::new(),
            tick: 0,
        };
        s.refresh();
        s
    }

    fn refresh(&mut self) {
        self.tick += 1;

        // Active profile
        if let Ok(cfg) = GlobalConfig::load() {
            self.active_profile = cfg.current_profile.clone();
            self.active_session = cfg.current_session.clone();
        }

        // Daemon status
        self.daemon_running = daemon_core::is_running();

        // Scheduler entries
        if let Ok(sched) = SchedulerState::load() {
            self.scheduler = sched;
        }

        // Recent switch events (last 5)
        let log = SwitchLog::open();
        if let Ok(events) = log.last_n(5) {
            self.recent_events = events
                .iter()
                .map(|e| format!("{} → {} | {}", e.from_profile, e.to_profile, e.reason))
                .collect();
        }

        // Per-profile/session stats
        self.rows.clear();
        let pm = ProfileManager::new(platform::profiles_dir());
        if let Ok(profiles) = pm.list() {
            for p in profiles {
                let profile_dir = platform::profile_dir(&p.name);
                let sm = SessionManager::new(profile_dir.join("sessions"));
                if let Ok(sessions) = sm.list() {
                    if sessions.is_empty() {
                        self.rows.push(ProfileRow {
                            profile: p.name.clone(),
                            session: "—".to_string(),
                            auth_type: p.auth_type.to_string(),
                            tokens_in: 0,
                            tokens_out: 0,
                            rate_limit_hits: 0,
                            cost_usd: 0.0,
                            last_used: "—".to_string(),
                        });
                    } else {
                        for s in sessions {
                            let session_dir = platform::session_dir(&p.name, &s.name);
                            let stats = SessionStats::load(&session_dir).unwrap_or_default();
                            let last_used = stats
                                .last_used
                                .map(|d| d.format("%m-%d %H:%M").to_string())
                                .unwrap_or_else(|| "—".to_string());
                            self.rows.push(ProfileRow {
                                profile: p.name.clone(),
                                session: s.name.clone(),
                                auth_type: p.auth_type.to_string(),
                                tokens_in: stats.tokens_in,
                                tokens_out: stats.tokens_out,
                                rate_limit_hits: stats.rate_limit_hits,
                                cost_usd: stats.estimated_cost_usd,
                                last_used,
                            });
                        }
                    }
                }
            }
        }
    }
}

// ─── Rendering ──────────────────────────────────────────────────────────────

fn render(f: &mut Frame, state: &TopState) {
    let area = f.area();

    // Main layout: header | body | footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(10),   // body
            Constraint::Length(5), // scheduler + recent events
            Constraint::Length(1), // footer
        ])
        .split(area);

    render_header(f, state, chunks[0]);
    render_body(f, state, chunks[1]);
    render_bottom(f, state, chunks[2]);
    render_footer(f, chunks[3]);
}

fn render_header(f: &mut Frame, state: &TopState, area: Rect) {
    let daemon_status = if state.daemon_running {
        Span::styled(
            " ● DAEMON ON ",
            Style::default().fg(Color::Black).bg(Color::White),
        )
    } else {
        Span::styled(" ○ DAEMON OFF ", Style::default().fg(Color::DarkGray))
    };

    let active = if state.active_profile.is_empty() {
        "NO PROFILE ACTIVE".to_string()
    } else {
        format!("{}:{}", state.active_profile, state.active_session)
    };

    let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let spin_char = spinner[(state.tick as usize) % spinner.len()];

    let line = Line::from(vec![
        Span::styled(
            format!(" {} CST TOP ", spin_char),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" │ "),
        Span::styled(
            format!("ACTIVE: {}", active),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        daemon_status,
    ]);

    let header = Paragraph::new(line)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Left);
    f.render_widget(header, area);
}

fn render_body(f: &mut Frame, state: &TopState, area: Rect) {
    let header_cells = [
        "PROFILE",
        "SESSION",
        "AUTH",
        "IN",
        "OUT",
        "RATE LIMITS",
        "COST $",
        "LAST USED",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));

    let header = Row::new(header_cells).height(1);

    let rows: Vec<Row> = state
        .rows
        .iter()
        .map(|r| {
            let is_active = r.profile == state.active_profile && r.session == state.active_session;
            let style = if is_active {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let active_marker = if is_active { "▶ " } else { "  " };

            Row::new(vec![
                Cell::from(format!("{}{}", active_marker, r.profile)).style(style),
                Cell::from(r.session.clone()).style(style),
                Cell::from(r.auth_type.clone()),
                Cell::from(format_tokens(r.tokens_in)),
                Cell::from(format_tokens(r.tokens_out)),
                Cell::from(r.rate_limit_hits.to_string()),
                Cell::from(format!("{:.4}", r.cost_usd)),
                Cell::from(r.last_used.clone()),
            ])
            .height(1)
        })
        .collect();

    let widths = [
        Constraint::Percentage(16),
        Constraint::Percentage(12),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(12),
        Constraint::Percentage(10),
        Constraint::Percentage(20),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" PROFILE USAGE "),
        )
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_widget(table, area);
}

fn render_bottom(f: &mut Frame, state: &TopState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_scheduler(f, state, chunks[0]);
    render_recent_events(f, state, chunks[1]);
}

fn render_scheduler(f: &mut Frame, state: &TopState, area: Rect) {
    let active: Vec<Line> = state
        .scheduler
        .entries
        .iter()
        .filter(|e| !e.switched_back)
        .map(|e| {
            Line::from(vec![
                Span::styled(
                    format!(" {} ", e.profile),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw("→ refills in "),
                Span::styled(
                    e.time_until_refill(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ])
        })
        .collect();

    let content = if active.is_empty() {
        vec![Line::from(Span::styled(
            " NO ACTIVE RATE LIMITS",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        active
    };

    let widget = Paragraph::new(content).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" QUOTA TIMERS "),
    );
    f.render_widget(widget, area);
}

fn render_recent_events(f: &mut Frame, state: &TopState, area: Rect) {
    let lines: Vec<Line> = if state.recent_events.is_empty() {
        vec![Line::from(Span::styled(
            " NO SWITCH EVENTS YET",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        state
            .recent_events
            .iter()
            .map(|e| Line::from(format!(" {}", e)))
            .collect()
    };

    let widget = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" RECENT SWITCHES "),
    );
    f.render_widget(widget, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let text = Paragraph::new(" q quit  r refresh  ↑↓ scroll  (refreshes every 1s)")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Left);
    f.render_widget(text, area);
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn format_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
