mod app;
mod sys_info;
mod ui;
use crate::app::App;
use crate::ui::ui;

use std::{error::Error, io};

use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    let linux = sys_info::LinuxInfo::default();

    let mut app = app::App {
        sys_info: linux.sys_info,
        exit: false,
    };

    enable_raw_mode()?;
    let mut stderr = io::stderr();
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    run(&mut terminal, &mut app);
    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    while !app.exit {
        terminal.draw(|frame| ui(frame, app));
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match key.code {
                KeyCode::Char('q') => app.exit(),
                _ => {}
            }
        }
    }
    return Ok(true);
}
