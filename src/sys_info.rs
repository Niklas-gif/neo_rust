//! This module is responsible to retrieve information about the system.

use std::fs;

//Get OS --> /etc/os-release --> pretty name
//Get CPU --> /proc/cpuinfo --> model name
//Get GPU --> lspci --> 09:00.0 VGA compatible controller: Advanced Micro Devices, Inc. [AMD/ATI] Navi 22 [Radeon RX 6700/6700 XT / 6800M] (rev c1)
//Get RAM --> free -m
//Get user name --> $USER
//Get Kernal TODO

#[derive(Debug, Default)]
///we store the metadata about the system here.
pub struct SysInfo {
    pub os: String,
    pub user: String,
    pub cpu: String,
}

///When Adding a new OS implement this trait.
pub trait GetSysInfo {
    fn get_os() -> String;
    fn get_user() -> String;
    fn get_cpu() -> String;
}

#[derive(Debug)]
/// Implementation for Linux.
pub struct LinuxInfo {
    pub sys_info: SysInfo,
}

impl LinuxInfo {
    ///Helper function for reading data using fs.
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

impl GetSysInfo for LinuxInfo {
    fn get_os() -> String {
        return LinuxInfo::parse_fs("/etc/os-release", "PRETTY_NAME=", Some('"'));
    }

    fn get_cpu() -> String {
        return LinuxInfo::parse_fs("/proc/cpuinfo", "model name", None);
    }

    fn get_user() -> String {
        let user = std::env::var("USER").ok();
        match user {
            Some(user) => return user,
            None => return "Unkown".to_string(),
        };
    }
}

impl Default for LinuxInfo {
    fn default() -> Self {
        Self {
            sys_info: SysInfo {
                os: LinuxInfo::get_os(),
                user: LinuxInfo::get_user(),
                cpu: LinuxInfo::get_cpu(),
            },
        }
    }
}
