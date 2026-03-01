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
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .split(frame.area());
    frame.render_widget(title_block(), chunks[0]);
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
