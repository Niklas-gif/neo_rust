//! This module is responsible for rendering tui stuff and handling user input for now.
//! However we will move the logic ui to its own module.

use crate::sys_info;

pub enum AppState {
    Main,
    Exiting,
}

#[derive(Debug)]
pub struct App {
    pub sys_info: sys_info::SysInfo,
    pub exit: bool,
}

impl App {
    /*pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }*/

    pub fn exit(&mut self) {
        self.exit = true;
    }

    /*fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }*/

    /*fn demo_widget(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Neo Rust".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".red().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let sys_text = Text::from(vec![
            Line::from(vec!["OS: ".into(), self.sys_info.os.as_str().red()]),
            Line::from(vec!["CPU: ".into(), self.sys_info.cpu.as_str().red()]),
            Line::from(vec!["User: ".into(), self.sys_info.user.as_str().red()]),
        ]);

        Paragraph::new(sys_text)
            .right_aligned()
            .block(block)
            .render(area, buf);
    }*/
}

/*impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.demo_widget(area, buf);
    }
}*/
