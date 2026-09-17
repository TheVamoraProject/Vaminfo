use super::Module;
use crate::config::VaminfoConfig;
use std::fs;
use sysinfo::System;

pub struct BatteryModule;

impl Module for BatteryModule {
    fn name(&self) -> &'static str { "Battery" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let entries = fs::read_dir("/sys/class/power_supply").ok()?;
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with("BAT") {
                continue;
            }

            let path = entry.path();
            let Ok(capacity) = fs::read_to_string(path.join("capacity")) else {
                continue;
            };
            let capacity = capacity.trim();
            if capacity.is_empty() {
                continue;
            }
            let status = fs::read_to_string(path.join("status"))
                .unwrap_or_default()
                .trim()
                .to_string();
            return Some(if status.is_empty() {
                format!("{} {}%", name, capacity)
            } else {
                format!("{} {}%  [{}]", name, capacity, status)
            });
        }
        None
    }
}
