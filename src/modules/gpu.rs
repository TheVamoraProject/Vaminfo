use super::Module;
use crate::config::VaminfoConfig;
use std::process::Command;
use sysinfo::System;

pub struct GpuModule;

impl Module for GpuModule {
    fn name(&self) -> &'static str { "GPU" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Try reading from /sys/class/drm (Linux)
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            if let Ok(entries) = fs::read_dir("/sys/class/drm") {
                for entry in entries.flatten() {
                    let path = entry.path().join("device/product_name");
                    if let Ok(name) = fs::read_to_string(&path) {
                        let name = name.trim().to_string();
                        if !name.is_empty() {
                            return Some(name);
                        }
                    }
                    // If only sysfs IDs are available, report the driver
                    // rather than leaking an unhelpful raw PCI code.
                    let driver_path = entry.path().join("device/driver");
                    let driver = fs::read_link(&driver_path)
                        .ok()
                        .and_then(|path| path.file_name().map(|name| name.to_string_lossy().to_string()));
                    if let Some(driver) = driver.filter(|value| !value.is_empty()) {
                        return Some(format!("{} graphics", driver));
                    }
                }
            }
        }

        // lspci provides a real model name when the DRM sysfs entry only
        // exposes numeric vendor/device IDs.
        if let Ok(out) = Command::new("lspci").arg("-nn").output() {
            if out.status.success() {
                for line in String::from_utf8_lossy(&out.stdout).lines() {
                    let lower = line.to_ascii_lowercase();
                    if !(lower.contains("vga compatible controller")
                        || lower.contains("3d controller")
                        || lower.contains("display controller"))
                    {
                        continue;
                    }
                    if let Some((_, model)) = line.split_once(": ") {
                        let model = model.split(" [").next().unwrap_or(model).trim();
                        if !model.is_empty() {
                            return Some(model.to_string());
                        }
                    }
                }
            }
        }

        None
    }
}
