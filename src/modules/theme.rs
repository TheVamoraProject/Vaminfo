use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::fs;

pub struct ThemeModule;

impl Module for ThemeModule {
    fn name(&self) -> &'static str { "Theme" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // 1. $GTK_THEME env var
        if let Ok(t) = std::env::var("GTK_THEME") {
            if !t.trim().is_empty() {
                return Some(t.trim().to_string());
            }
        }

        // 2. GTK 4 settings
        if let Some(t) = read_gtk_ini("gtk-4.0") {
            return Some(t);
        }

        // 3. GTK 3 settings
        if let Some(t) = read_gtk_ini("gtk-3.0") {
            return Some(t);
        }

        // 4. GTK 2 settings
        if let Some(home) = std::env::var("HOME").ok() {
            let p = format!("{}/.gtkrc-2.0", home);
            if let Ok(content) = fs::read_to_string(&p) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("gtk-theme-name") {
                        if let Some(val) = line.split('=').nth(1) {
                            let v = val.trim().trim_matches('"').to_string();
                            if !v.is_empty() { return Some(v); }
                        }
                    }
                }
            }
        }

        // 5. KDE / Plasma: check kdeglobals
        if let Some(home) = std::env::var("HOME").ok() {
            let p = format!("{}/.config/kdeglobals", home);
            if let Ok(content) = fs::read_to_string(&p) {
                let mut in_general = false;
                for line in content.lines() {
                    let line = line.trim();
                    if line == "[General]" { in_general = true; continue; }
                    if line.starts_with('[') { in_general = false; }
                    if in_general && line.starts_with("ColorScheme=") {
                        if let Some(val) = line.split('=').nth(1) {
                            let v = val.trim().to_string();
                            if !v.is_empty() { return Some(format!("{} (KDE)", v)); }
                        }
                    }
                }
            }
        }

        None
    }
}

fn read_gtk_ini(dir: &str) -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let path = format!("{}/.config/{}/settings.ini", home, dir);
    let content = fs::read_to_string(&path).ok()?;
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("gtk-theme-name") {
            if let Some(val) = line.split('=').nth(1) {
                let v = val.trim().trim_matches('"').to_string();
                if !v.is_empty() { return Some(v); }
            }
        }
    }
    None
}
