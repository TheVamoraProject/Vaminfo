use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::fs;

pub struct FsTypeModule;

impl Module for FsTypeModule {
    fn name(&self) -> &'static str { "Filesystem" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Parse /proc/mounts for the root filesystem entry
        if let Ok(content) = fs::read_to_string("/proc/mounts") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                // fields: device mountpoint fstype options dump pass
                if parts.len() >= 3 && parts[1] == "/" {
                    let fstype = parts[2];
                    // Skip pseudo filesystems
                    if !matches!(fstype, "tmpfs" | "proc" | "sysfs" | "devtmpfs" | "none") {
                        return Some(fstype.to_string());
                    }
                }
            }
        }

        // Fallback: `df -T /` — third column on second line
        if let Ok(out) = std::process::Command::new("df")
            .args(["-T", "/"])
            .output()
        {
            if let Ok(s) = std::str::from_utf8(&out.stdout) {
                if let Some(data_line) = s.lines().nth(1) {
                    let parts: Vec<&str> = data_line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return Some(parts[1].to_string());
                    }
                }
            }
        }

        None
    }
}
