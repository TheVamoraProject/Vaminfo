use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::collections::HashMap;
use std::fs;

const RELEASE_FILE: &str = "/etc/VamoraSys/vamora-release.vmf";

// ── 3 modules ────────────────────────────────────────────────────────────────

/// VamoraSys version  →  VAMORASYS_VERSION field
pub struct VamoraSysVersionModule;

/// VMF package version  →  VERSION field
pub struct VmfVersionModule;

/// Vamora combined version + codename  →  "1.0 Orion"
pub struct VamoraVersionCodenameModule;

// ── Shared parser ─────────────────────────────────────────────────────────────

pub fn parse_vmf() -> Option<HashMap<String, String>> {
    let content = fs::read_to_string(RELEASE_FILE).ok()?;
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some(eq) = line.find('=') {
            let key = line[..eq].trim().to_string();
            let val = line[eq + 1..].trim().trim_matches('"').to_string();
            map.insert(key, val);
        }
    }
    if map.is_empty() { None } else { Some(map) }
}

// ── Impls ─────────────────────────────────────────────────────────────────────

impl Module for VamoraSysVersionModule {
    fn name(&self) -> &'static str { "VamoraSys" }
    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let m = parse_vmf()?;
        m.get("VAMORASYS_VERSION").cloned().filter(|s| !s.is_empty())
    }
}

impl Module for VmfVersionModule {
    fn name(&self) -> &'static str { "VMF Version" }
    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let m = parse_vmf()?;
        m.get("VERSION").cloned().filter(|s| !s.is_empty())
    }
}

impl Module for VamoraVersionCodenameModule {
    fn name(&self) -> &'static str { "Vamora" }
    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let m = parse_vmf()?;
        let version  = m.get("VERSION").cloned().unwrap_or_default();
        let codename = m.get("VERSION_CODENAME").cloned().unwrap_or_default();
        match (version.is_empty(), codename.is_empty()) {
            (false, false) => Some(format!("{} {}", version, codename)),
            (false, true)  => Some(version),
            (true,  false) => Some(codename),
            _              => None,
        }
    }
}
