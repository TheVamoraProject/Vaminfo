use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::fs;

pub struct TtyTypeModule;

impl Module for TtyTypeModule {
    fn name(&self) -> &'static str { "TTY" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Resolve /proc/self/fd/0 -> device path
        if let Ok(target) = fs::read_link("/proc/self/fd/0") {
            let path = target.to_string_lossy().to_string();
            if path.starts_with("/dev/pts/") {
                let n = path.trim_start_matches("/dev/pts/");
                return Some(format!("PTY (pts/{})", n));
            }
            if path.starts_with("/dev/tty") {
                return Some(format!("TTY ({})", path.trim_start_matches("/dev/")));
            }
            if path == "/dev/null" {
                return Some("none (piped/null)".to_string());
            }
            return Some(path);
        }

        // Fallback: check TERM env
        if let Ok(term) = std::env::var("TERM") {
            if term == "linux" {
                return Some("TTY".to_string());
            }
            return Some(format!("PTY ({})", term));
        }

        None
    }
}
