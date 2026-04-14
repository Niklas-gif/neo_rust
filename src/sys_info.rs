//! This module is responsible to retrieve information about the system.

pub fn get_os() {
    // Get kernel -> uname -s <- Only works on unix based systems!
    println!("OS->{}",os_info::get());
}

#[derive(Debug, Default)]
///stores the metadata about the system here.
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