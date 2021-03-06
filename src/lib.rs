//! Prown is a project manager and a file watcher
//!
//! WiP

extern crate app_dirs;
extern crate glob;
extern crate notify;
extern crate toml;

mod error;
mod parser;
pub mod project;
mod prown;

use app_dirs::AppInfo;

pub const APP_INFO: AppInfo = AppInfo {
    name: "prown",
    author: "notkild",
};
