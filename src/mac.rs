use std::process::Command;

use crate::sys_info::{self, GetSysInfo, SysInfo};


#[derive(Debug)]
/// Implementation for Mac.
pub struct MacInfo {
    pub sys_info: SysInfo,
}

impl sys_info::GetSysInfo for MacInfo {
    
    fn get_os_info() -> String {
        //sw_vers
        let sw_vers = Command::new("sw_vers").output().expect("sw_verse failed");
        //TODO formating
        String::from_utf8_lossy(&sw_vers.stdout).to_string()
    }
    
    fn get_user() -> String {
        let user = std::env::var("USER").ok();
        match user {
            Some(user) => return user,
            None => return "Unknown".to_string(),
        };
    }
    
    fn get_cpu() -> String {
        todo!()
    }
    
    fn get_gpu() -> String {
        todo!()
    }
}

impl Default for MacInfo {
    fn default() -> Self {
        Self {
            sys_info: SysInfo {
                os: MacInfo::get_os_info(),
                user: MacInfo::get_user(),
                cpu: todo!(),
                gpu: todo!(),
                ascii_logo: todo!(),
            },
        }
    }
}