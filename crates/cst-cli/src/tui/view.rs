//! TUI rendering — ratatui widgets.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
    Frame,
};

use super::model::{AppState, Tab};

/// Main render function — called every frame.
pub fn render(f: &mut Frame, state: &AppState) {
    let area = f.area();

    // Outer layout: title bar + tab bar + content + footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // tab bar
            Constraint::Min(0),    // content
            Constraint::Length(2), // footer / status
        ])
        .split(area);

    render_tab_bar(f, state, chunks[0]);
    render_content(f, state, chunks[1]);
    render_footer(f, state, chunks[2]);
}

fn render_tab_bar(f: &mut Frame, state: &AppState, area: Rect) {
    let titles: Vec<Line> = Tab::all()
        .iter()
        .map(|t| Line::from(t.title()))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" CLAUDE SENTINEL "))
        .select(state.tab.index())
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD));

    f.render_widget(tabs, area);
}

fn render_content(f: &mut Frame, state: &AppState, area: Rect) {
    match &state.tab {
        Tab::Profiles => render_profiles(f, state, area),
        Tab::Sessions => render_sessions(f, state, area),
        Tab::AutoSwitch => render_auto_switch(f, state, area),
        Tab::History => render_history(f, state, area),
    }
}

fn render_profiles(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    // Profile list
    let items: Vec<ListItem> = state.profiles.iter().map(|p| {
        let active_marker = if p.is_active { "▶ " } else { "  " };
        let label = format!("{}{} ({})", active_marker, p.name, p.auth_type);
        let style = if p.is_active {
            Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(label).style(style)
    }).collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_profile));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" PROFILES "))
        .highlight_symbol("→ ");

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // Profile detail panel
    let detail = if let Some(p) = state.profiles.get(state.selected_profile) {
        let sessions_str = if p.sessions.is_empty() {
            "  (no sessions)".to_string()
        } else {
            p.sessions.iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let active_str = if p.is_active {
            format!("  ACTIVE  ({}: {})", p.name, state.current_session)
        } else {
            "  inactive".to_string()
        };

        format!(
            "Name:      {}\nAuth:      {}\nStatus:    {}\n\nSessions:\n{}",
            p.name, p.auth_type, active_str, sessions_str
        )
    } else {
        "No profile selected.".to_string()
    };

    let detail_widget = Paragraph::new(detail)
        .block(Block::default().borders(Borders::ALL).title(" DETAIL "))
        .alignment(Alignment::Left);

    f.render_widget(detail_widget, chunks[1]);
}

fn render_sessions(f: &mut Frame, state: &AppState, area: Rect) {
    let profile_name = state.selected_profile_name().unwrap_or("—");
    let sessions = state.selected_profile_sessions();

    let items: Vec<ListItem> = sessions.iter().enumerate().map(|(i, s)| {
        let active = state.current_profile == profile_name && state.current_session == s.as_str();
        let marker = if active { "▶ " } else { "  " };
        let style = if i == state.selected_session {
            Style::default().fg(Color::Black).bg(Color::White)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(format!("{}{}", marker, s)).style(style)
    }).collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_session));

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" SESSIONS — {} ", profile_name)))
        .highlight_symbol("→ ");

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_auto_switch(f: &mut Frame, state: &AppState, area: Rect) {
    let mut lines = vec![
        Line::from(Span::styled(" AUTO-SWITCH STATUS", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];

    let has_entries = state.scheduler.entries.iter().any(|e| !e.switched_back);

    if !has_entries {
        lines.push(Line::from(" No active rate-limit timers."));
        lines.push(Line::from(""));
        lines.push(Line::from(" Configure: cst auto-switch configure <profile>"));
        lines.push(Line::from(" Start daemon: cst daemon start"));
    } else {
        for entry in state.scheduler.entries.iter().filter(|e| !e.switched_back) {
            lines.push(Line::from(format!(" Profile: {}", entry.profile)));
            lines.push(Line::from(format!(
                "   Rate limited at: {}",
                entry.detected_at.format("%H:%M:%S UTC")
            )));
            lines.push(Line::from(format!(
                "   Refill in:       {}",
                entry.time_until_refill()
            )));
            lines.push(Line::from(format!(
                "   Auto-switch back: {}",
                entry.auto_switch_back
            )));
            lines.push(Line::from(""));
        }
    }

    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" AUTO-SWITCH "));
    f.render_widget(para, area);
}

fn render_history(f: &mut Frame, state: &AppState, area: Rect) {
    let items: Vec<ListItem> = if state.history_lines.is_empty() {
        vec![ListItem::new(" No switch history recorded yet.")]
    } else {
        state.history_lines.iter()
            .map(|l| ListItem::new(format!(" {}", l)))
            .collect()
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" SWITCH HISTORY (last 30) "));
    f.render_widget(list, area);
}

fn render_footer(f: &mut Frame, state: &AppState, area: Rect) {
    let active = if state.current_profile.is_empty() {
        "no active profile".to_string()
    } else {
        format!("{}:{}", state.current_profile, state.current_session)
    };

    let help = " ↑↓ move  Tab/←→ switch tab  Enter select  q quit";

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let status_widget = Paragraph::new(format!(" Active: {}", active))
        .style(Style::default().fg(Color::White));
    let help_widget = Paragraph::new(help)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Right);

    f.render_widget(status_widget, chunks[0]);
    f.render_widget(help_widget, chunks[1]);
}
