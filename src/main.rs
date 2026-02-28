mod app;
mod sys_info;

use std::io;

const _PLACE_HOLDER: &str = "
┌──────────────────────────────────────────────────┐
│                                                  │
│           xx                                     │
│          xx     xx    xxxxx       x xxxx         │
│        xx x    xx    x    xxx     xxx  xxx       │
│       xx  x  xx      x      xx    x      xx      │
│       x  xx xx      xxxx         xx       x      │
│      x   x xx      xx   xxxx     x        xx     │
│     x    xx        x             x         x     │
│   xxx    xx       x     xx       x        x      │
│  xx      x        xxxxxxxxx      xxx    xx       │
│                                    xxxxx         │
│          xxx                                     │
│         xx x xx    x        xxxx     xxx         │
│        x      x   x     x  x   x       xxxx      │
│       xx      x  x      x  x          xx  xxxx   │
│       x xxxxxx   x     xx   xxxx      x          │
│      xx   xx     x     x        xx    x          │
│      x      xxx  xx  xx      x xxx    xx         │
│               x   xxxx        xx      x          │
│                                     xx           │
│                                     x            │
│                                                  │
└──────────────────────────────────────────────────┘";

//Get OS --> /etc/os-release --> pretty name
//Get CPU --> /proc/cpuinfo --> model name
//Get GPU --> lspci --> 09:00.0 VGA compatible controller: Advanced Micro Devices, Inc. [AMD/ATI] Navi 22 [Radeon RX 6700/6700 XT / 6800M] (rev c1)
//Get RAM --> free -m
//Get user name --> $USER
//Get Kernal TODO

fn main() -> Result<(), io::Error> {
    let linux = sys_info::LinuxInfo::default();

    let mut app = app::App {
        sys_info: linux.sys_info,
        exit: false,
    };
    ratatui::run(|terminal| app.run(terminal))
}
