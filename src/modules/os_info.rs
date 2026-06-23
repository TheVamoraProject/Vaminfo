use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct OsModule;

impl Module for OsModule {
    fn name(&self) -> &'static str { "OS" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let version = System::os_version().unwrap_or_default();
        let arch = System::cpu_arch().unwrap_or_else(|| "unknown".to_string());
        if version.is_empty() {
            Some(format!("{} {}", name, arch))
        } else {
            Some(format!("{} {} {}", name, version, arch))
        }
    }
}
