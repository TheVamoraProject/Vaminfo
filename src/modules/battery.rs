use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct BatteryModule;

impl Module for BatteryModule {
    fn name(&self) -> &'static str { "Battery" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let bat_paths = [
                "/sys/class/power_supply/BAT0",
                "/sys/class/power_supply/BAT1",
            ];
            for bat in &bat_paths {
                let cap_path = format!("{}/capacity", bat);
                let status_path = format!("{}/status", bat);
                if let Ok(cap) = fs::read_to_string(&cap_path) {
                    let cap = cap.trim().to_string();
                    let status = fs::read_to_string(&status_path)
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    return Some(format!("{}%  [{}]", cap, status));
                }
            }
        }
        None
    }
}
