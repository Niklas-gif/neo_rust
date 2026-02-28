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

use std::{io, process::Command};

//Get OS --> /etc/os-release --> pretty name
//Get CPU --> /proc/cpuinfo --> model name
//Get GPU --> lspci --> 09:00.0 VGA compatible controller: Advanced Micro Devices, Inc. [AMD/ATI] Navi 22 [Radeon RX 6700/6700 XT / 6800M] (rev c1)
//Get RAM --> free -m
//Get user name --> $USER
struct SysInfo {
    os: String,
    user: String,
    cpu: String,
}

fn get_os() -> String {
    let output = Command::new("cat")
        .arg("/etc/os-release")
        .output()
        .expect("Failed to fetch os");
    return String::from_utf8_lossy(&output.stdout).to_string();
}

// better use  fs::read_to_string

fn main() {
    let sys_info = SysInfo {
        os: get_os(),
        user: "".to_string(),
        cpu: "".to_string(),
    };
    println!("Test -> {} ", sys_info.os);
}
