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
struct SysInfo {
    os: String,
    user: String,
    cpu: String,
}

impl SysInfo {}

pub trait FetchSystemInfos {
    fn get_os() -> String;
    fn get_user() -> String;
    fn get_cput() -> String;
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

fn main() {
    let sys_info = SysInfo {
        os: get_os(),
        user: get_user(),
        cpu: get_cpu(),
    };
    println!("OS -> {} ", sys_info.os);
    println!("CPU -> {}", sys_info.cpu);
    println!("USER -> {}", sys_info.user);
}
