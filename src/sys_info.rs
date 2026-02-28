use std::fs;

#[derive(Debug, Default)]
pub struct SysInfo {
    pub os: String,
    pub user: String,
    pub cpu: String,
}

pub trait GetSysInfo {
    fn get_os() -> String;
    fn get_user() -> String;
    fn get_cpu() -> String;
}

/// Implementation for Linux system
#[derive(Debug)]
pub struct LinuxInfo {
    pub sys_info: SysInfo,
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
