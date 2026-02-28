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

use std::{fs, io};

//Get OS --> /etc/os-release --> pretty name
//Get CPU --> /proc/cpuinfo --> model name
//Get GPU --> lspci --> 09:00.0 VGA compatible controller: Advanced Micro Devices, Inc. [AMD/ATI] Navi 22 [Radeon RX 6700/6700 XT / 6800M] (rev c1)
//Get RAM --> free -m
//Get user name --> $USER
//Get Kernal TODO

#[derive(Debug, Default)]
struct SysInfo {
    os: String,
    user: String,
    cpu: String,
}

impl SysInfo {}

pub trait FetchSystemInfos {
    fn get_os() -> String;
    fn get_user() -> String;
    fn get_cpu() -> String;
}

struct LinuxInfo {}
//LINUX!
/*fn get_os() -> String {
    let file_sys_output =
        fs::read_to_string("/etc/os-release").expect("couldn't read /ect/os-release !");

    for line in file_sys_output.lines() {
        if line.starts_with("PRETTY_NAME=") {
            return line["PRETTY_NAME=".len()..].trim_matches('"').to_string();
        }
    }
    return "Unknown OS".to_string();
}*/

/*fn get_cpu() -> String {
    let file_sys_output =
        fs::read_to_string("/proc/cpuinfo").expect("couldn't read /proc/cpuinfo !");

    for line in file_sys_output.lines() {
        if line.starts_with("model name	:") {
            return line["model name	:".len()..].to_string();
        }
    }
    return "Unknown CPU".to_string();
}*/
fn parse_fs(path: &str, begin: &str, trim: Option<char>) -> String {
    let file_sys_output = fs::read_to_string(path).expect(&format!("couldn't read {path}"));

    match trim {
        Some(c) => {
            for line in file_sys_output.lines() {
                if line.starts_with(begin) {
                    return line[begin.len()..].trim_matches(c).to_string();
                }
            }
        }
        None => {
            for line in file_sys_output.lines() {
                if line.starts_with(begin) {
                    return line[begin.len()..].to_string();
                }
            }
        }
    }
    return "Unknown".to_string();
}

fn get_os() -> String {
    return parse_fs("/etc/os-release", "PRETTY_NAME=", Some('"'));
}

fn get_cpu() -> String {
    return parse_fs("/proc/cpuinfo", "model name", None);
}

fn get_user() -> String {
    let user = std::env::var("USER").ok();
    match user {
        Some(user) => return user,
        None => return "Unkown".to_string(),
    };
}

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
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Neo Rust".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".red().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        //TODO Remove clones
        let sys_text = Text::from(vec![Line::from(vec![
            "OS: ".into(),
            self.sys_info.os.as_str().red(),
            "CPU: ".into(),
            self.sys_info.cpu.as_str().red(),
            "User: ".into(),
            self.sys_info.user.as_str().red(),
        ])]);

        Paragraph::new(sys_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> Result<(), io::Error> {
    let sys_info = SysInfo {
        os: get_os(),
        user: get_user(),
        cpu: get_cpu(),
    };

    let mut app = App {
        sys_info: sys_info,
        exit: false,
    };
    ratatui::run(|terminal| app.run(terminal))
}
