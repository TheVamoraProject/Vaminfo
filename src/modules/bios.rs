use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::fs;

pub struct BiosModule;

impl Module for BiosModule {
    fn name(&self) -> &'static str { "BIOS" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let base = "/sys/class/dmi/id";
        let vendor  = fs::read_to_string(format!("{}/bios_vendor", base)).ok()?.trim().to_string();
        let version = fs::read_to_string(format!("{}/bios_version", base)).unwrap_or_default().trim().to_string();
        let date    = fs::read_to_string(format!("{}/bios_date", base)).unwrap_or_default().trim().to_string();

        if vendor.is_empty() { return None; }

        let mut parts = vec![vendor];
        if !version.is_empty() { parts.push(version); }
        if !date.is_empty()    { parts.push(date); }
        Some(parts.join("  "))
    }
}
