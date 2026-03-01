use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    style::{Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Min(20)])
        .split(frame.area());
    frame.render_widget(demo_widget(app), chunks[1]);
}

fn title_block() -> Block<'static> {
    /*let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let neo_rust = Paragraph::new(Text::styled("Neo Rust", Style::default().fg(Color::Green)))
        .block(title_block);*/
    let title = Line::from("Neo Rust".bold());
    let instructions = Line::from(vec![" Quit ".into(), "<Q> ".red().bold()]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);
    return block;
}

pub fn demo_widget(app: &App) -> Paragraph<'_> {
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
