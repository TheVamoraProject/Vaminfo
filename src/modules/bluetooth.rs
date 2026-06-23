use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::fs;

pub struct BluetoothModule;

impl Module for BluetoothModule {
    fn name(&self) -> &'static str { "Bluetooth" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let bt_dir = std::path::Path::new("/sys/class/bluetooth");
        if !bt_dir.exists() {
            return None;
        }

        let entries: Vec<_> = fs::read_dir(bt_dir)
            .ok()?
            .flatten()
            .collect();

        if entries.is_empty() {
            return None;
        }

        let mut adapters: Vec<String> = Vec::new();
        for entry in &entries {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("hci") {
                let state_path = entry.path().join("power/runtime_status");
                let enabled_path = entry.path().join("blocked");
                let address_path = entry.path().join("address");

                let state = if let Ok(s) = fs::read_to_string(&state_path) {
                    let s = s.trim().to_string();
                    if s == "active" { "on" } else { "off" }.to_string()
                } else {
                    // fallback: check if blocked
                    if let Ok(b) = fs::read_to_string(&enabled_path) {
                        if b.trim() == "0" { "on".to_string() } else { "off".to_string() }
                    } else {
                        "on".to_string()
                    }
                };

                let addr = fs::read_to_string(&address_path)
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                if addr.is_empty() {
                    adapters.push(format!("{} [{}]", name, state));
                } else {
                    adapters.push(format!("{} [{}] {}", name, state, addr));
                }
            }
        }

        if adapters.is_empty() { None } else { Some(adapters.join("  ")) }
    }
}
