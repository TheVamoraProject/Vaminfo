use super::Module;
use crate::config::VaminfoConfig;
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
                    // Try vendor + device via modalias
                    let vendor_path = entry.path().join("device/vendor");
                    let device_path = entry.path().join("device/device");
                    if let (Ok(v), Ok(d)) = (
                        fs::read_to_string(&vendor_path),
                        fs::read_to_string(&device_path),
                    ) {
                        let v = v.trim().to_string();
                        let d = d.trim().to_string();
                        if !v.is_empty() && !d.is_empty() {
                            return Some(format!("GPU [{} {}]", v, d));
                        }
                    }
                }
            }
        }
        // Fallback: not detected
        None
    }
}
