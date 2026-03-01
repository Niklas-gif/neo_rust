//! This module is responsible for rendering the tui.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Min(20)])
        .split(frame.area());
    frame.render_widget(demo_widget(app), chunks[1]);
    frame.render_widget(demo_ascii_art(app), chunks[0]);
}

fn demo_ascii_art(app: &App) -> Paragraph<'_> {
    let block = Block::bordered().border_set(border::THICK);
    match &app.sys_info.ascii_logo {
        Some(a) => Paragraph::new(Text::from(a.to_string()))
            .left_aligned()
            .block(block),
        None => Paragraph::new(Text::from("No ART"))
            .left_aligned()
            .block(block),
    }
}

fn demo_widget(app: &App) -> Paragraph<'_> {
    let title = Line::from("Neo Rust".bold());
    let instructions = Line::from(vec![" Quit ".into(), "<Q> ".red().bold()]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let sys_text = Text::from(vec![
        Line::from(vec!["OS: ".into(), app.sys_info.os.as_str().red()]),
        Line::from(vec!["CPU: ".into(), app.sys_info.cpu.as_str().red()]),
        Line::from(vec!["User: ".into(), app.sys_info.user.as_str().red()]),
    ]);

    return Paragraph::new(sys_text).right_aligned().block(block);
}
