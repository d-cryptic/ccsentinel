//! TUI rendering — ratatui widgets.
//!
//! Editorial-minimal palette: a single muted-teal accent for active/selected
//! state and titles, soft grays for everything else, hairline borders. Matches
//! the desktop and web design language.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
    Frame,
};

use super::model::{AppState, Tab};

// ── Palette ──────────────────────────────────────────────────────────────
const ACCENT: Color = Color::Rgb(94, 196, 182); // muted teal
const TEXT: Color = Color::Rgb(220, 224, 230); // primary
const MUTED: Color = Color::Rgb(154, 163, 174); // secondary
const FAINT: Color = Color::Rgb(106, 114, 125); // borders / tertiary
const SEL_BG: Color = Color::Rgb(22, 27, 34); // cursor row background

/// A hairline-bordered block with a faint border and a muted title.
fn panel(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(FAINT))
        .title(Span::styled(
            title,
            Style::default().fg(MUTED).add_modifier(Modifier::BOLD),
        ))
}

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
    let titles: Vec<Line> = Tab::all().iter().map(|t| Line::from(t.title())).collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(FAINT))
                .title(Span::styled(
                    " Claude Sentinel ",
                    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
                )),
        )
        .select(state.tab.index())
        .style(Style::default().fg(MUTED))
        .highlight_style(
            Style::default()
                .fg(ACCENT)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        );

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
    let items: Vec<ListItem> = state
        .profiles
        .iter()
        .map(|p| {
            let active_marker = if p.is_active { "● " } else { "  " };
            let label = format!("{}{} ({})", active_marker, p.name, p.auth_type);
            let style = if p.is_active {
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(TEXT)
            };
            ListItem::new(label).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_profile));

    let list = List::new(items)
        .block(panel(" Profiles "))
        .highlight_style(Style::default().bg(SEL_BG))
        .highlight_symbol("→ ");

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // Profile detail panel
    let detail_lines: Vec<Line> = if let Some(p) = state.profiles.get(state.selected_profile) {
        let status = if p.is_active {
            Span::styled(
                format!("active · {}:{}", p.name, state.current_session),
                Style::default().fg(ACCENT),
            )
        } else {
            Span::styled("inactive", Style::default().fg(FAINT))
        };

        let mut lines = vec![
            kv("Name", &p.name),
            kv("Auth", &p.auth_type),
            Line::from(vec![
                Span::styled("  Status    ", Style::default().fg(FAINT)),
                status,
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  Sessions",
                Style::default().fg(MUTED).add_modifier(Modifier::BOLD),
            )),
        ];
        if p.sessions.is_empty() {
            lines.push(Line::from(Span::styled(
                "    (no sessions)",
                Style::default().fg(FAINT),
            )));
        } else {
            for s in &p.sessions {
                lines.push(Line::from(vec![
                    Span::styled("    • ", Style::default().fg(ACCENT)),
                    Span::styled(s.clone(), Style::default().fg(TEXT)),
                ]));
            }
        }
        lines
    } else {
        vec![Line::from(Span::styled(
            "No profile selected.",
            Style::default().fg(FAINT),
        ))]
    };

    let detail_widget = Paragraph::new(detail_lines)
        .block(panel(" Detail "))
        .alignment(Alignment::Left);

    f.render_widget(detail_widget, chunks[1]);
}

/// A right-padded key/value detail line.
fn kv<'a>(key: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("  {key:<9} "), Style::default().fg(FAINT)),
        Span::styled(value.to_string(), Style::default().fg(TEXT)),
    ])
}

fn render_sessions(f: &mut Frame, state: &AppState, area: Rect) {
    let profile_name = state.selected_profile_name().unwrap_or("—");
    let sessions = state.selected_profile_sessions();

    let items: Vec<ListItem> = sessions
        .iter()
        .map(|s| {
            let active =
                state.current_profile == profile_name && state.current_session == s.as_str();
            let marker = if active { "● " } else { "  " };
            let style = if active {
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(TEXT)
            };
            ListItem::new(format!("{}{}", marker, s)).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_session));

    let title = format!(" Sessions — {} ", profile_name);
    let list = List::new(items)
        .block(panel(&title))
        .highlight_style(Style::default().bg(SEL_BG))
        .highlight_symbol("→ ");

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_auto_switch(f: &mut Frame, state: &AppState, area: Rect) {
    let mut lines = vec![
        Line::from(Span::styled(
            " Auto-switch status",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    let has_entries = state.scheduler.entries.iter().any(|e| !e.switched_back);

    if !has_entries {
        lines.push(Line::from(Span::styled(
            " No active rate-limit timers.",
            Style::default().fg(MUTED),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            " Configure: cst auto-switch configure <profile>",
            Style::default().fg(FAINT),
        )));
        lines.push(Line::from(Span::styled(
            " Start daemon: cst daemon start",
            Style::default().fg(FAINT),
        )));
    } else {
        for entry in state.scheduler.entries.iter().filter(|e| !e.switched_back) {
            lines.push(Line::from(vec![
                Span::styled(" Profile: ", Style::default().fg(FAINT)),
                Span::styled(
                    entry.profile.clone(),
                    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(detail_kv(
                "   Rate limited at: ",
                &entry.detected_at.format("%H:%M:%S UTC").to_string(),
            ));
            lines.push(detail_kv(
                "   Refill in:       ",
                &entry.time_until_refill(),
            ));
            lines.push(detail_kv(
                "   Auto-switch back: ",
                &entry.auto_switch_back.to_string(),
            ));
            lines.push(Line::from(""));
        }
    }

    let para = Paragraph::new(lines).block(panel(" Auto-switch "));
    f.render_widget(para, area);
}

fn detail_kv<'a>(key: &'a str, value: &str) -> Line<'a> {
    Line::from(vec![
        Span::styled(key, Style::default().fg(FAINT)),
        Span::styled(value.to_string(), Style::default().fg(TEXT)),
    ])
}

fn render_history(f: &mut Frame, state: &AppState, area: Rect) {
    let items: Vec<ListItem> = if state.history_lines.is_empty() {
        vec![ListItem::new(Span::styled(
            " No switch history recorded yet.",
            Style::default().fg(FAINT),
        ))]
    } else {
        state
            .history_lines
            .iter()
            .map(|l| ListItem::new(Span::styled(format!(" {}", l), Style::default().fg(TEXT))))
            .collect()
    };

    let list = List::new(items).block(panel(" Switch history (last 30) "));
    f.render_widget(list, area);
}

fn render_footer(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let active_span = if state.current_profile.is_empty() {
        Span::styled("no active profile", Style::default().fg(FAINT))
    } else {
        Span::styled(
            format!("{}:{}", state.current_profile, state.current_session),
            Style::default().fg(ACCENT),
        )
    };
    let status_widget = Paragraph::new(Line::from(vec![
        Span::styled(" Active: ", Style::default().fg(FAINT)),
        active_span,
    ]));

    let help_widget = Paragraph::new(" ↑↓ move   Tab/←→ switch tab   Enter select   q quit")
        .style(Style::default().fg(FAINT))
        .alignment(Alignment::Right);

    f.render_widget(status_widget, chunks[0]);
    f.render_widget(help_widget, chunks[1]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::model::{AppState, ProfileRow, Tab};
    use cst_core::auto_switch::scheduler::SchedulerState;
    use ratatui::{backend::TestBackend, buffer::Buffer, Terminal};

    fn mock_state(tab: Tab) -> AppState {
        AppState {
            tab,
            profiles: vec![
                ProfileRow {
                    name: "work".into(),
                    auth_type: "oauth".into(),
                    sessions: vec!["backend".into(), "frontend".into()],
                    is_active: true,
                },
                ProfileRow {
                    name: "personal".into(),
                    auth_type: "api".into(),
                    sessions: vec!["default".into()],
                    is_active: false,
                },
                ProfileRow {
                    name: "research".into(),
                    auth_type: "vertex".into(),
                    sessions: vec![],
                    is_active: false,
                },
            ],
            selected_profile: 0,
            selected_session: 0,
            current_profile: "work".into(),
            current_session: "backend".into(),
            scheduler: SchedulerState::default(),
            history_lines: vec![
                "06-22 09:00 │ team-shared:default → work:backend │ Schedule".into(),
                "06-21 18:00 │ work:backend → team-shared:default │ Schedule".into(),
            ],
            should_quit: false,
            status_message: String::new(),
        }
    }

    /// Renders every tab without panicking. When `CST_TUI_SNAPSHOT` points at a
    /// file path, also writes a colour-accurate HTML snapshot of all tabs for
    /// design review.
    #[test]
    fn render_all_tabs() {
        let tabs = [Tab::Profiles, Tab::Sessions, Tab::AutoSwitch, Tab::History];
        let mut html = String::new();
        for tab in tabs {
            let state = mock_state(tab.clone());
            let backend = TestBackend::new(96, 22);
            let mut terminal = Terminal::new(backend).unwrap();
            terminal.draw(|f| render(f, &state)).unwrap();
            html.push_str(&format!(
                "<h2>{}</h2>{}",
                tab.title(),
                buffer_to_html(terminal.backend().buffer())
            ));
        }

        if let Ok(path) = std::env::var("CST_TUI_SNAPSHOT") {
            let doc = format!(
                "<!doctype html><meta charset=utf-8><style>\
                 body{{background:#0b0e12;margin:24px;font-family:'JetBrains Mono',ui-monospace,monospace}}\
                 h2{{color:#9aa3ae;font:600 13px/1 ui-sans-serif,system-ui;letter-spacing:.14em;text-transform:uppercase;margin:22px 0 8px}}\
                 pre{{margin:0;line-height:1.25;font-size:13px;border:1px solid #222933;border-radius:10px;padding:14px;width:fit-content}}\
                 </style>{html}"
            );
            std::fs::write(&path, doc).unwrap();
        }
    }

    fn css_color(c: Color, fallback: &str) -> String {
        match c {
            Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
            Color::Reset => fallback.to_string(),
            _ => fallback.to_string(),
        }
    }

    fn buffer_to_html(buf: &Buffer) -> String {
        let w = buf.area.width;
        let h = buf.area.height;
        let mut out = String::from("<pre>");
        for y in 0..h {
            for x in 0..w {
                let cell = &buf.content[(y * w + x) as usize];
                let fg = css_color(cell.fg, "#eceff3");
                let bold = cell.modifier.contains(Modifier::BOLD);
                let underline = cell.modifier.contains(Modifier::UNDERLINED);
                let bg = match cell.bg {
                    Color::Rgb(r, g, b) => format!(";background:#{r:02x}{g:02x}{b:02x}"),
                    _ => String::new(),
                };
                let sym = cell
                    .symbol()
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;");
                out.push_str(&format!(
                    "<span style=\"color:{fg}{bg}{}{}\">{sym}</span>",
                    if bold { ";font-weight:700" } else { "" },
                    if underline {
                        ";text-decoration:underline"
                    } else {
                        ""
                    },
                ));
            }
            out.push('\n');
        }
        out.push_str("</pre>");
        out
    }
}
