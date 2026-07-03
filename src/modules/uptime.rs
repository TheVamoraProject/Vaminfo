use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct UptimeModule;

impl Module for UptimeModule {
    fn name(&self) -> &'static str { "Uptime" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let secs = read_uptime_secs();
        let days  = secs / 86400;
        let hours = (secs % 86400) / 3600;
        let mins  = (secs % 3600) / 60;

        let mut parts = Vec::new();
        if days  > 0 { parts.push(format!("{}d", days));  }
        if hours > 0 { parts.push(format!("{}h", hours)); }
        parts.push(format!("{}m", mins));

        Some(parts.join(" "))
    }
}

/// Read uptime seconds — tries three sources in order.
pub fn read_uptime_secs() -> u64 {
    // 1. /proc/uptime  (Linux & Android/Termux standard path)
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(first) = content.split_whitespace().next() {
            if let Ok(f) = first.parse::<f64>() {
                if f > 0.0 {
                    return f as u64;
                }
            }
        }
    }

    // 2. /proc/stat btime — "btime <unix_boot_timestamp>"
    //    Works on Android when /proc/uptime permissions are restricted.
    if let Ok(content) = fs::read_to_string("/proc/stat") {
        for line in content.lines() {
            if line.starts_with("btime ") {
                if let Some(ts_str) = line.split_whitespace().nth(1) {
                    if let Ok(btime) = ts_str.parse::<u64>() {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        if now > btime {
                            return now - btime;
                        }
                    }
                }
                break;
            }
        }
    }

    // 3. sysinfo fallback (macOS, BSD, etc.)
    System::uptime()
}
