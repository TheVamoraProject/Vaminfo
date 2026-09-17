use super::Module;
use crate::config::VaminfoConfig;
use std::fs;
use std::path::Path;
use sysinfo::System;

pub struct TemperatureModule;

impl Module for TemperatureModule {
    fn name(&self) -> &'static str { "Temperature" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let mut readings: Vec<(String, f64)> = Vec::new();

        // Linux thermal zones are available without external commands and
        // cover laptops, desktops, VMs, and many ARM boards.
        if let Ok(entries) = fs::read_dir("/sys/class/thermal") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if !name.starts_with("thermal_zone") {
                    continue;
                }
                let path = entry.path();
                let Some(temp) = read_millidegrees(&path.join("temp")) else { continue };
                let label = fs::read_to_string(path.join("type"))
                    .ok()
                    .map(|value| value.trim().to_string())
                    .filter(|value| !value.is_empty())
                    .unwrap_or(name);
                readings.push((label, temp));
            }
        }

        // Fall back to hwmon when a kernel driver exposes sensors there only.
        if readings.is_empty() {
            if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let chip = fs::read_to_string(path.join("name"))
                        .ok()
                        .map(|value| value.trim().to_string())
                        .unwrap_or_else(|| "sensor".to_string());
                    for index in 1..=10 {
                        let input = path.join(format!("temp{}_input", index));
                        let Some(temp) = read_millidegrees(&input) else { continue };
                        let label = fs::read_to_string(path.join(format!("temp{}_label", index)))
                            .ok()
                            .map(|value| value.trim().to_string())
                            .filter(|value| !value.is_empty())
                            .unwrap_or_else(|| chip.clone());
                        readings.push((label, temp));
                    }
                }
            }
        }

        if readings.is_empty() {
            return None;
        }

        readings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let (label, temp) = &readings[0];
        Some(format!("{} {:.1}°C", label, temp))
    }
}

fn read_millidegrees(path: &Path) -> Option<f64> {
    let raw = fs::read_to_string(path).ok()?;
    let value: f64 = raw.trim().parse().ok()?;
    if !(-100_000.0..=200_000.0).contains(&value) {
        return None;
    }
    Some(value / 1000.0)
}