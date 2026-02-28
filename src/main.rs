use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use std::io;

use crate::sys_info::SysInfo;

mod sys_info;
const _PLACE_HOLDER: &str = "
┌──────────────────────────────────────────────────┐
│                                                  │
│           xx                                     │
│          xx     xx    xxxxx       x xxxx         │
│        xx x    xx    x    xxx     xxx  xxx       │
│       xx  x  xx      x      xx    x      xx      │
│       x  xx xx      xxxx         xx       x      │
│      x   x xx      xx   xxxx     x        xx     │
│     x    xx        x             x         x     │
│   xxx    xx       x     xx       x        x      │
│  xx      x        xxxxxxxxx      xxx    xx       │
│                                    xxxxx         │
│          xxx                                     │
│         xx x xx    x        xxxx     xxx         │
│        x      x   x     x  x   x       xxxx      │
│       xx      x  x      x  x          xx  xxxx   │
│       x xxxxxx   x     xx   xxxx      x          │
│      xx   xx     x     x        xx    x          │
│      x      xxx  xx  xx      x xxx    xx         │
│               x   xxxx        xx      x          │
│                                     xx           │
│                                     x            │
│                                                  │
└──────────────────────────────────────────────────┘";

//Get OS --> /etc/os-release --> pretty name
//Get CPU --> /proc/cpuinfo --> model name
//Get GPU --> lspci --> 09:00.0 VGA compatible controller: Advanced Micro Devices, Inc. [AMD/ATI] Navi 22 [Radeon RX 6700/6700 XT / 6800M] (rev c1)
//Get RAM --> free -m
//Get user name --> $USER
//Get Kernal TODO

#[derive(Debug)]
pub struct App {
    sys_info: SysInfo,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
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
    }

    fn demo_widget(&self, area: Rect, buf: &mut Buffer) {
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
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.demo_widget(area, buf);
    }
}

fn main() -> Result<(), io::Error> {
    let linux = sys_info::LinuxInfo::default();

    let mut app = App {
        sys_info: linux.sys_info,
        exit: false,
    };
    ratatui::run(|terminal| app.run(terminal))
}
