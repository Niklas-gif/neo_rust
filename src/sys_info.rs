//! This module is responsible to retrieve information about the system.
use regex::Regex;
use std::{fs, process::Command};

#[derive(Debug, Default)]
///we store the metadata about the system here.
pub struct SysInfo {
    pub os: String,
    pub user: String,
    pub cpu: String,
    pub gpu: String,
    pub ascii_logo: Option<String>,
}

///When Adding a new OS implement this trait.
pub trait GetSysInfo {
    fn get_os() -> String;
    fn get_user() -> String;
    fn get_cpu() -> String;
    fn get_gpu() -> String;
}

//Get OS --> /etc/os-release --> pretty name
//Get CPU --> /proc/cpuinfo --> model name
//Get GPU --> lspci --> 09:00.0 VGA compatible controller: Advanced Micro Devices, Inc. [AMD/ATI] Navi 22 [Radeon RX 6700/6700 XT / 6800M] (rev c1)
//Get RAM --> free -m
//Get user name --> $USER
//Get Kernal TODO

const LINUX_ART: &str = r#"
             _nnnn_
            dGGGGMMb
           @p~qp~~qMb
           M|@||@) M|
           @,----.JM|
          JS^\__/  qKL
         dZP        qKRb
        dZP          qKKb
        fZP            SMMb
        HZM            MMMM
        FqM            MMMM
        __| ".        |\dS"qML
        |    `.       | `' \Zq
        _)      \.___.,|     .'
        \____   )MMMMMP|   .'
        `-'       `--' hjm "#;

#[derive(Debug)]
/// Implementation for Linux.
pub struct LinuxInfo {
    pub sys_info: SysInfo,
}

impl LinuxInfo {
    ///Helper function for reading data using fs.
    fn parse_fs(path: &str, begin: &str, trim: Option<char>) -> String {
        let file_sys_output = fs::read_to_string(path).expect(&format!("couldn't read {path}"));

        for line in file_sys_output.lines() {
            if line.starts_with(begin) {
                match trim {
                    Some(c) => return line[begin.len()..].trim_matches(c).to_string(),
                    None => return line[begin.len()..].to_string(),
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
            None => return "Unknown".to_string(),
        };
    }

    fn get_gpu() -> String {
        let lspci_out = Command::new("lspci").output().expect("lspci failed");
        let regex = Regex::new(r"VGA.*[^\]]*.*[^\]]*").unwrap();
        let stdout = String::from_utf8_lossy(&lspci_out.stdout).to_string();
        //TODO formating
        let gpu_string = regex.find(&stdout);
        return gpu_string.map(|m| m.as_str()).unwrap_or("None").to_string();
    }
}

impl Default for LinuxInfo {
    fn default() -> Self {
        Self {
            sys_info: SysInfo {
                os: LinuxInfo::get_os(),
                user: LinuxInfo::get_user(),
                cpu: LinuxInfo::get_cpu(),
                gpu: LinuxInfo::get_gpu(),
                ascii_logo: Some(String::from(LINUX_ART)),
            },
        }
    }
}
