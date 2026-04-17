use std::{process::Command};

use crate::sys_info::{self, GetSysInfo, SysInfo};


const MAC_ART: &str = r#"
                                                  
                                ##                
                            ######                
                            ######                
                          ######                  
                          ##                      
                                                  
            ##############################        
          ################################        
          ##############################          
        ##############################            
        ##############################            
        ##############################            
        ##############################            
        ##############################            
        ################################          
        ################################          
          ##################################      
          ##################################      
          ################################        
            ##############################        
              ##########################          
                ##########  ##########            
                  ##                              
                                                  
"#;

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
        let cpu_info = Command::new("cpuinfo").output().expect("cpuinfo failed");
        let result = String::from_utf8_lossy(&cpu_info.stdout).to_string();
        for line in result.lines() {
            if line.starts_with("Brand Raw:") {
                return line["Brand Raw:".len()..].to_string()
            }
        }
        return "Did not find line 'Brand Raw:'".to_string()
    }
    
    fn get_gpu() -> String {
        //TODO
        "metal".to_string()
    }
}

impl Default for MacInfo {
    fn default() -> Self {
        Self {
            sys_info: SysInfo {
                os: MacInfo::get_os_info(),
                user: MacInfo::get_user(),
                cpu: MacInfo::get_cpu(),
                gpu: MacInfo::get_gpu(),
                ascii_logo: Some(String::from(MAC_ART)),
            },
        }
    }
}