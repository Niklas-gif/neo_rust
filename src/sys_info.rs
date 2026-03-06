//! This module is responsible to retrieve information about the system.
use regex::Regex;
use std::{fs, process::Command};

pub fn get_os() {
    // Get kernel -> uname -s
    todo!()
}

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
    fn get_os_info() -> String;
    fn get_user() -> String;
    fn get_cpu() -> String;
    fn get_gpu() -> String;
    //fn get_memory() -> String;
}

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

enum GPUVendor {
    AMD,
    NVIDIA,
    INTEL,
    MCST,
    VIRTUALBOX,
}

impl GPUVendor {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Advanced" => Some(GPUVendor::AMD),
            "NVIDIA" => Some(GPUVendor::NVIDIA),
            "Intel" => Some(GPUVendor::INTEL),
            "MCST" => Some(GPUVendor::MCST),
            "VirtualBox" => Some(GPUVendor::VIRTUALBOX),
            _ => None,
        }
    }
}

impl GetSysInfo for LinuxInfo {
    fn get_os_info() -> String {
        // Get Distro
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
        // TODO set vendor
        let gpu_string = regex.find(&stdout);
        return gpu_string.map(|m| m.as_str()).unwrap_or("None").to_string();
    }
}

impl Default for LinuxInfo {
    fn default() -> Self {
        Self {
            sys_info: SysInfo {
                os: LinuxInfo::get_os_info(),
                user: LinuxInfo::get_user(),
                cpu: LinuxInfo::get_cpu(),
                gpu: LinuxInfo::get_gpu(),
                ascii_logo: Some(String::from(LINUX_ART)),
            },
        }
    }
}
