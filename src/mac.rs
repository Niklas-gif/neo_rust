use crate::sys_info::{self, GetSysInfo, SysInfo};


#[derive(Debug)]
/// Implementation for Mac.
pub struct MacInfo {
    pub sys_info: SysInfo,
}

impl sys_info::GetSysInfo for MacInfo {
    
    fn get_os_info() -> String {
        todo!()
    }
    
    fn get_user() -> String {
        todo!()
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
                os: todo!(),
                user: todo!(),
                cpu: todo!(),
                gpu: todo!(),
                ascii_logo: todo!(),
            },
        }
    }
}