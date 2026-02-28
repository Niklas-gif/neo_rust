use std::fs;

use crate::{get_cpu, get_os, get_user};

// TODO create seperate struct which gets used bye windows mac linux
pub trait SystemInfo {
    fn get_os(&self) -> String;
    fn get_user(&self) -> String;
    fn get_cpu(&self) -> String;
}

#[derive(Debug)]
pub struct LinuxInfo {
    pub os: String,
    pub user: String,
    pub cpu: String,
}

impl Default for LinuxInfo {
    fn default() -> Self {
        Self {
            os: get_os(),
            user: get_user(),
            cpu: get_cpu(),
        }
    }
}

impl LinuxInfo {
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
}

impl SystemInfo for LinuxInfo {
    fn get_os(&self) -> String {
        return LinuxInfo::parse_fs("/etc/os-release", "PRETTY_NAME=", Some('"'));
    }

    fn get_cpu(&self) -> String {
        return LinuxInfo::parse_fs("/proc/cpuinfo", "model name", None);
    }

    fn get_user(&self) -> String {
        let user = std::env::var("USER").ok();
        match user {
            Some(user) => return user,
            None => return "Unkown".to_string(),
        };
    }
}
