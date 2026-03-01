mod app;
mod sys_info;
mod ui;

use std::io;

fn main() -> Result<(), io::Error> {
    let linux = sys_info::LinuxInfo::default();

    let mut app = app::App {
        sys_info: linux.sys_info,
        exit: false,
    };
    ratatui::run(|terminal| app.run(terminal))
}
