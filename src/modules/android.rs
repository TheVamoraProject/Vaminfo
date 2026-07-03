use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::process::Command;

pub struct AndroidVersionModule;
pub struct AndroidDeviceModule;

pub fn getprop(key: &str) -> Option<String> {
    Command::new("getprop")
        .arg(key)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn is_android() -> bool {
    std::env::var("TERMUX_VERSION").is_ok()
        || std::path::Path::new("/system/build.prop").exists()
        || std::path::Path::new("/data/data/com.termux").exists()
}

/// Shows SDK level + security patch detail (not the main OS name — that's in OsModule).
impl Module for AndroidVersionModule {
    fn name(&self) -> &'static str { "Android" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        if !is_android() { return None; }
        let sdk      = getprop("ro.build.version.sdk").unwrap_or_default();
        let security = getprop("ro.build.version.security_patch").unwrap_or_default();
        match (sdk.is_empty(), security.is_empty()) {
            (false, false) => Some(format!("SDK {} | Patch {}", sdk, security)),
            (false, true)  => Some(format!("SDK {}", sdk)),
            (true,  false) => Some(format!("Patch {}", security)),
            _              => None,
        }
    }
}

impl Module for AndroidDeviceModule {
    fn name(&self) -> &'static str { "Device" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        if !is_android() { return None; }
        let brand  = getprop("ro.product.brand").unwrap_or_default();
        let model  = getprop("ro.product.model").unwrap_or_else(|| "Unknown".to_string());
        let device = getprop("ro.product.device").unwrap_or_default();
        let mut s = if brand.is_empty() { model.clone() } else { format!("{} {}", brand, model) };
        if !device.is_empty() && device != model { s.push_str(&format!(" ({})", device)); }
        Some(s)
    }
}
