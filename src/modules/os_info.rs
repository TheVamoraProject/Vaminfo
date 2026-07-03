use super::Module;
use crate::config::VaminfoConfig;
use crate::modules::android::{getprop, is_android};
use crate::modules::vamora_os::parse_vmf;
use sysinfo::System;

pub struct OsModule;

impl Module for OsModule {
    fn name(&self) -> &'static str { "OS" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let arch = System::cpu_arch().unwrap_or_else(|| "unknown".to_string());

        // 1. VamoraOS — use PRETTY_NAME if present
        if let Some(vmf) = parse_vmf() {
            if let Some(pretty) = vmf.get("PRETTY_NAME") {
                if !pretty.is_empty() {
                    return Some(format!("{} {}", pretty, arch));
                }
            }
        }

        // 2. Android — show "Android X.X <arch>"
        if is_android() {
            let ver = getprop("ro.build.version.release").unwrap_or_else(|| "?".to_string());
            return Some(format!("Android {} {}", ver, arch));
        }

        // 3. Standard Linux /etc/os-release via sysinfo
        let name    = System::name().unwrap_or_else(|| "Unknown".to_string());
        let version = System::os_version().unwrap_or_default();
        if version.is_empty() {
            Some(format!("{} {}", name, arch))
        } else {
            Some(format!("{} {} {}", name, version, arch))
        }
    }
}
