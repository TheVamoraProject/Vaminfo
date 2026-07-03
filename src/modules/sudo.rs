use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::process::Command;

pub struct SudoModule;

impl Module for SudoModule {
    fn name(&self) -> &'static str { "Sudo" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Non-interactive sudo check — no password prompt, no side effects
        let has_sudo = Command::new("sudo")
            .args(["-n", "true"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if has_sudo {
            return Some("yes (passwordless)".to_string());
        }

        // Check if user is in sudo/wheel/admin group
        let user = std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .unwrap_or_default();

        if is_in_sudo_group(&user) {
            return Some("yes (group member)".to_string());
        }

        // Check if root (uid 0)
        if let Ok(out) = Command::new("id").arg("-u").output() {
            let uid = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if uid == "0" {
                return Some("root".to_string());
            }
        }

        Some("no".to_string())
    }
}

fn is_in_sudo_group(user: &str) -> bool {
    if user.is_empty() { return false; }
    let content = std::fs::read_to_string("/etc/group").unwrap_or_default();
    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 { continue; }
        let group_name = parts[0];
        if matches!(group_name, "sudo" | "wheel" | "admin" | "sudoers") {
            let members = parts[3];
            if members.split(',').any(|m| m.trim() == user) {
                return true;
            }
        }
    }
    false
}
