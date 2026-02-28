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

use std::{fs, io, process::Command};

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

impl SysInfo {}

fn get_os() -> String {
    let file_sys_output =
        fs::read_to_string("/etc/os-release").expect("couldn't read /ect/os-release !");

    for line in file_sys_output.lines() {
        if line.starts_with("PRETTY_NAME=") {
            return line["PRETTY_NAME=".len()..].trim_matches('"').to_string();
        }
    }
    return "Unknown OS".to_string();
}

fn get_cpu() -> String {
    let file_sys_output =
        fs::read_to_string("/proc/cpuinfo").expect("couldn't read /proc/cpuinfo !");

    for line in file_sys_output.lines() {
        if line.starts_with("model name	:") {
            return line["model name	:".len()..].to_string();
        }
    }
    return "Unknown CPU".to_string();
}

fn parse_fs(path: &str, begin: &str, trim: Option<char>, err: Option<&str>) -> String {
    return "".to_string();
}
// better use  fs::read_to_string

fn main() {
    let sys_info = SysInfo {
        os: get_os(),
        user: "".to_string(),
        cpu: get_cpu(),
    };
    println!("OS -> {} ", sys_info.os);
    println!("CPU -> {}", sys_info.cpu);
}
