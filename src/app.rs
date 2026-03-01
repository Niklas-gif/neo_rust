//! This module is responsible for keeping the Appstate.

use crate::sys_info;

#[derive(Debug)]
pub struct App {
    pub sys_info: sys_info::SysInfo,
    pub exit: bool,
}

impl App {
    pub fn exit(&mut self) {
        self.exit = true;
    }
}
