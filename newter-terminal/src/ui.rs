//! Frozen ice-glass UI: cool tones, soft surfaces, and crisp highlights.

use ratatui::prelude::*;
use ratatui::widgets::*;

/// Frozen glass palette: deep blue backdrop + frosted surfaces.
pub struct FrozenGlass;

impl FrozenGlass {
    pub const BG: Color = Color::Rgb(0x05, 0x16, 0x2B);
    pub const SURFACE: Color = Color::Rgb(0x14, 0x2A, 0x45);
    pub const SURFACE_ALT: Color = Color::Rgb(0x0F, 0x22, 0x3A);
    pub const SURFACE_LIGHT: Color = Color::Rgb(0x1C, 0x3A, 0x5B);
    pub const BORDER: Color = Color::Rgb(0x7B, 0xC5, 0xEE);
    pub const TEXT: Color = Color::Rgb(0xE7, 0xF7, 0xFF);
    pub const TEXT_DIM: Color = Color::Rgb(0xAD, 0xCF, 0xE2);
    pub const ACCENT: Color = Color::Rgb(0xCB, 0xF0, 0xFF);
    pub const RUN: Color = Color::Rgb(0x9E, 0xEA, 0xFF);
    pub const WARN: Color = Color::Rgb(0xFF, 0xC8, 0xD6);
}

pub fn draw(frame: &mut Frame, app: &crate::App) {
    let area = frame.area();

    // Full-screen deep winter background.
    let bg = Block::default()
        .style(Style::default().bg(FrozenGlass::BG));
    frame.render_widget(bg, area);

    // Main layout.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(5),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .margin(2)
        .split(area);

    // VoiceOS-inspired floating ice nav.
    let nav_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(FrozenGlass::BORDER))
        .style(Style::default().bg(FrozenGlass::SURFACE).fg(FrozenGlass::TEXT));
    frame.render_widget(nav_block, chunks[0]);

    let nav_inner = Rect {
        x: chunks[0].x + 1,
        y: chunks[0].y + 1,
        width: chunks[0].width.saturating_sub(2),
        height: chunks[0].height.saturating_sub(2),
    };

    let nav_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(18), Constraint::Min(20), Constraint::Length(22)])
        .split(nav_inner);

    let logo = Paragraph::new(
        Line::from(" newter.ice")
            .style(Style::default().fg(FrozenGlass::ACCENT).add_modifier(Modifier::BOLD)),
    )
    .alignment(Alignment::Left);
    frame.render_widget(logo, nav_cols[0]);

    let menu = Paragraph::new(
        Line::from(" Pricing    Blog    About ")
            .style(Style::default().fg(FrozenGlass::TEXT_DIM)),
    )
    .alignment(Alignment::Center);
    frame.render_widget(menu, nav_cols[1]);

    let cta = Paragraph::new(
        Line::from("  Download for Windows  ")
            .style(Style::default().fg(FrozenGlass::BG).bg(FrozenGlass::ACCENT).add_modifier(Modifier::BOLD)),
    )
    .alignment(Alignment::Right);
    frame.render_widget(cta, nav_cols[2]);

    // Output area.
    let output_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(FrozenGlass::BORDER))
        .style(Style::default().bg(FrozenGlass::SURFACE_ALT).fg(FrozenGlass::TEXT))
        .title(" output ");
    frame.render_widget(output_block, chunks[1]);

    let output_inner = Rect {
        x: chunks[1].x + 1,
        y: chunks[1].y + 1,
        width: chunks[1].width.saturating_sub(2),
        height: chunks[1].height.saturating_sub(2),
    };

    let lines: Vec<Line> = app.output.iter().map(|s| styled_output_line(s)).collect();
    let output_para = Paragraph::new(lines)
        .scroll((app.output.len().saturating_sub(output_inner.height as usize) as u16, 0));
    frame.render_widget(output_para, output_inner);

    // Input line.
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(FrozenGlass::BORDER))
        .style(Style::default().bg(FrozenGlass::SURFACE_LIGHT).fg(FrozenGlass::TEXT))
        .title(" ice:// ");
    frame.render_widget(input_block, chunks[2]);

    let input_inner = Rect {
        x: chunks[2].x + 1,
        y: chunks[2].y + 1,
        width: chunks[2].width.saturating_sub(2),
        height: chunks[2].height.saturating_sub(2),
    };
    let input_display = if app.input.is_empty() {
        "run newter-compiler/examples/hello.newt".to_string()
    } else {
        app.input.clone()
    };
    let input_para = Paragraph::new(input_display)
        .style(Style::default().fg(if app.input.is_empty() {
            FrozenGlass::TEXT_DIM
        } else {
            FrozenGlass::TEXT
        }));
    frame.render_widget(input_para, input_inner);

    // Cursor on input
    let cursor_x = input_inner.x + app.input.len() as u16 + 1;
    let cursor_y = input_inner.y;
    if cursor_x < input_inner.x + input_inner.width {
        frame.set_cursor_position((cursor_x, cursor_y));
    }

    // Thin glow line/status footer.
    let footer = Paragraph::new(
        Line::from(" voice layer active  •  frozen nav skin ")
            .style(Style::default().fg(FrozenGlass::TEXT_DIM)),
    )
    .alignment(Alignment::Center)
    .style(Style::default().bg(FrozenGlass::BG));
    frame.render_widget(footer, chunks[3]);
}

fn styled_output_line(s: &str) -> Line<'static> {
    if s.starts_with("> ") {
        return Line::from(s.to_string()).style(
            Style::default()
                .fg(FrozenGlass::ACCENT)
                .add_modifier(Modifier::BOLD),
        );
    }
    if s.contains("[run]") || s.contains("started compiler") {
        return Line::from(s.to_string()).style(Style::default().fg(FrozenGlass::RUN));
    }
    if s.contains("error:") || s.contains("unknown:") {
        return Line::from(s.to_string()).style(Style::default().fg(FrozenGlass::WARN));
    }
    Line::from(s.to_string()).style(Style::default().fg(FrozenGlass::TEXT))
}
