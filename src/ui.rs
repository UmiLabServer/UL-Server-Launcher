use crate::app::{App, ServerStatus};
use ratatui::{
    prelude::*,
    widgets::{
        Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Tabs, Wrap,
    },
};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Header with title
    let title = Paragraph::new("🚀 UmiLab Server Launcher")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(title, chunks[0]);

    // Tabs
    let tab_titles: Vec<Line> = app
        .tabs
        .iter()
        .map(|t| Line::from(*t))
        .collect();
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Navigation"))
        .select(app.current_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunks[1]);
    
    f.render_widget(tabs, main_chunks[0]);

    // Main content based on selected tab
    match app.current_tab {
        0 => render_servers_tab(f, main_chunks[1], app),
        1 => render_logs_tab(f, main_chunks[1], app),
        2 => render_settings_tab(f, main_chunks[1], app),
        _ => {}
    }

    // Footer with help
    let help_text = match app.current_tab {
        0 => "Press ↑↓/jk to navigate, Enter to toggle server, Tab to switch tabs, q to quit",
        1 => "Press Tab to switch tabs, q to quit",
        2 => "Press Tab to switch tabs, q to quit",
        _ => "Press q to quit",
    };
    
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[2]);
}

fn render_servers_tab(f: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec!["Name", "Host", "Port", "Status"])
        .style(Style::default().fg(Color::Yellow))
        .height(1)
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .servers
        .iter()
        .enumerate()
        .map(|(i, server)| {
            let style = if i == app.selected_server {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };

            let status_style = match server.status {
                ServerStatus::Running => Style::default().fg(Color::Green),
                ServerStatus::Stopped => Style::default().fg(Color::Red),
                ServerStatus::Starting => Style::default().fg(Color::Yellow),
                ServerStatus::Error => Style::default().fg(Color::LightRed),
            };

            Row::new(vec![
                Cell::from(server.name.as_str()),
                Cell::from(server.host.as_str()),
                Cell::from(server.port.to_string()),
                Cell::from(server.status.as_str()).style(status_style),
            ])
            .style(style)
        })
        .collect();

    let table = Table::new(rows)
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
            Constraint::Percentage(30),
        ])
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Server Status")
            .border_style(Style::default().fg(Color::White)),
    )
    .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(table, area);
}

fn render_logs_tab(f: &mut Frame, area: Rect, _app: &App) {
    let logs = vec![
        "[2024-01-15 10:30:15] Server started successfully",
        "[2024-01-15 10:30:16] Listening on port 8080",
        "[2024-01-15 10:31:22] New client connected: 192.168.1.100",
        "[2024-01-15 10:32:45] Processing request: GET /api/status",
        "[2024-01-15 10:33:01] Database connection established",
        "[2024-01-15 10:35:12] Warning: High memory usage detected",
        "[2024-01-15 10:36:30] Client disconnected: 192.168.1.100",
        "[2024-01-15 10:37:45] Backup process completed",
    ];

    let log_items: Vec<ListItem> = logs
        .iter()
        .map(|log| {
            let style = if log.contains("Warning") || log.contains("Error") {
                Style::default().fg(Color::Yellow)
            } else if log.contains("successfully") || log.contains("completed") {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*log).style(style)
        })
        .collect();

    let logs_list = List::new(log_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Server Logs")
                .border_style(Style::default().fg(Color::White)),
        );

    f.render_widget(logs_list, area);
}

fn render_settings_tab(f: &mut Frame, area: Rect, _app: &App) {
    let settings_text = vec![
        Line::from("Server Configuration:"),
        Line::from(""),
        Line::from("• Auto-start servers on launch: Enabled"),
        Line::from("• Log retention period: 30 days"),
        Line::from("• Default port range: 8000-9000"),
        Line::from("• Max concurrent connections: 1000"),
        Line::from(""),
        Line::from("Monitoring:"),
        Line::from(""),
        Line::from("• Health check interval: 30 seconds"),
        Line::from("• CPU threshold: 80%"),
        Line::from("• Memory threshold: 90%"),
        Line::from("• Disk space threshold: 85%"),
        Line::from(""),
        Line::from("Notifications:"),
        Line::from(""),
        Line::from("• Email alerts: Enabled"),
        Line::from("• Slack integration: Disabled"),
        Line::from("• Discord webhook: Configured"),
    ];

    let settings = Paragraph::new(settings_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Settings")
                .border_style(Style::default().fg(Color::White)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(settings, area);
}