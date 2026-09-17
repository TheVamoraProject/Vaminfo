use super::Module;
use crate::config::VaminfoConfig;
use std::fs;
use std::process::Command;
use sysinfo::System;

pub struct PackagesModule;

impl Module for PackagesModule {
    fn name(&self) -> &'static str { "Packages" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Prefer package databases over package-manager commands: this keeps
        // the fetch fast and avoids refresh/network operations.
        if let Ok(content) = fs::read_to_string("/var/lib/dpkg/status") {
            let count = content
                .lines()
                .filter(|line| *line == "Status: install ok installed")
                .count();
            if count > 0 {
                return Some(format!("dpkg: {}", count));
            }
        }

        if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
            let count = entries.flatten().filter(|entry| entry.path().is_dir()).count();
            if count > 0 {
                return Some(format!("pacman: {}", count));
            }
        }

        if let Ok(content) = fs::read_to_string("/lib/apk/db/installed") {
            let count = content.lines().filter(|line| line.starts_with("P:")).count();
            if count > 0 {
                return Some(format!("apk: {}", count));
            }
        }

        if let Ok(entries) = fs::read_dir("/var/db/xbps") {
            let count = entries.flatten().filter(|entry| {
                entry.path()
                    .extension()
                    .map(|ext| ext == "plist")
                    .unwrap_or(false)
            }).count();
            if count > 0 {
                return Some(format!("xbps: {}", count));
            }
        }

        if command_exists("rpm") {
            if let Ok(output) = Command::new("rpm").args(["-qa", "--qf", "."]).output() {
                if output.status.success() {
                    let count = output.stdout.iter().filter(|byte| **byte == b'.').count();
                    if count > 0 {
                        return Some(format!("rpm: {}", count));
                    }
                }
            }
        }

        None
    }
}

fn command_exists(command: &str) -> bool {
    std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).any(|dir| dir.join(command).is_file()))
        .unwrap_or(false)
}