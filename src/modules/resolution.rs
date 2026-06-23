use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::process::Command;
use std::fs;

pub struct ResolutionModule;

impl Module for ResolutionModule {
    fn name(&self) -> &'static str { "Resolution" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Try xrandr (X11)
        if let Ok(out) = Command::new("xrandr").arg("--query").output() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.contains(" connected") {
                    // e.g. "eDP-1 connected primary 1920x1080+0+0"
                    for token in line.split_whitespace() {
                        if token.contains('x') && token.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                            let res = token.split('+').next().unwrap_or(token);
                            if res.contains('x') {
                                return Some(res.to_string());
                            }
                        }
                    }
                }
            }
        }

        // Try /sys/class/drm modes
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            let mut found: Vec<String> = Vec::new();
            for entry in entries.flatten() {
                let modes_path = entry.path().join("modes");
                if let Ok(content) = fs::read_to_string(&modes_path) {
                    if let Some(first) = content.lines().next() {
                        let mode = first.trim().to_string();
                        if !mode.is_empty() && mode.contains('x') {
                            if !found.contains(&mode) {
                                found.push(mode);
                            }
                        }
                    }
                }
            }
            if !found.is_empty() {
                return Some(found.join("  |  "));
            }
        }

        // Try wlr-randr (Wayland)
        if let Ok(out) = Command::new("wlr-randr").output() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.trim_start().starts_with("Mode:") || line.contains("current") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for p in &parts {
                        if p.contains('x') && p.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                            return Some(p.to_string());
                        }
                    }
                }
            }
        }

        None
    }
}
