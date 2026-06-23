use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct UptimeModule;

impl Module for UptimeModule {
    fn name(&self) -> &'static str { "Uptime" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let secs = System::uptime();
        let days = secs / 86400;
        let hours = (secs % 86400) / 3600;
        let mins = (secs % 3600) / 60;

        let mut parts = Vec::new();
        if days > 0 { parts.push(format!("{}d", days)); }
        if hours > 0 { parts.push(format!("{}h", hours)); }
        parts.push(format!("{}m", mins));

        Some(parts.join(" "))
    }
}
