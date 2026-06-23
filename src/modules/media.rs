use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::process::Command;

pub struct MediaModule;

impl Module for MediaModule {
    fn name(&self) -> &'static str { "Media" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Try playerctl (most common on Linux desktops)
        if let Ok(out) = Command::new("playerctl")
            .args(["metadata", "--format", "{{artist}} - {{title}}"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() && s != " - " {
                    return Some(s);
                }
            }
        }

        // Try playerctl with just title
        if let Ok(out) = Command::new("playerctl")
            .args(["metadata", "title"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return Some(s);
                }
            }
        }

        // Try dbus-send for mpris
        if let Ok(out) = Command::new("dbus-send")
            .args([
                "--print-reply",
                "--dest=org.mpris.MediaPlayer2.playerctld",
                "/org/mpris/MediaPlayer2",
                "org.freedesktop.DBus.Properties.Get",
                "string:org.mpris.MediaPlayer2.Player",
                "string:Metadata",
            ])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout);
                if let Some(line) = s.lines().find(|l| l.contains("xesam:title")) {
                    if let Some(pos) = line.rfind('"') {
                        let start = line[..pos].rfind('"').unwrap_or(0) + 1;
                        let title = &line[start..pos];
                        if !title.is_empty() {
                            return Some(title.to_string());
                        }
                    }
                }
            }
        }

        None
    }
}
