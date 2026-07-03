use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::collections::HashMap;
use std::fs;

const RELEASE_FILE: &str = "/etc/VamoraSys/vamora-release.vmf";

pub struct VamoraVersionModule;
pub struct VamoraCodenameModule;
pub struct VamoraBuildModule;
pub struct VamoraArchModule;

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

impl Module for VamoraVersionModule {
    fn name(&self) -> &'static str { "Vam Version" }
    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let m = parse_vmf()?;
        let version = m.get("VERSION")?.clone();
        let vs_ver  = m.get("VAMORASYS_VERSION").cloned().unwrap_or_default();
        if vs_ver.is_empty() { Some(version) } else { Some(format!("{} (VamSys {})", version, vs_ver)) }
    }
}

impl Module for VamoraCodenameModule {
    fn name(&self) -> &'static str { "Codename" }
    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let m = parse_vmf()?;
        m.get("VERSION_CODENAME").cloned().filter(|s| !s.is_empty())
    }
}

impl Module for VamoraBuildModule {
    fn name(&self) -> &'static str { "Build" }
    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let m = parse_vmf()?;
        let build_id = m.get("BUILD_ID").cloned().unwrap_or_default();
        let channel  = m.get("CHANNEL").cloned().unwrap_or_default();
        match (build_id.is_empty(), channel.is_empty()) {
            (false, false) => Some(format!("{} [{}]", build_id, channel)),
            (false, true)  => Some(build_id),
            (true,  false) => Some(channel),
            _              => None,
        }
    }
}

impl Module for VamoraArchModule {
    fn name(&self) -> &'static str { "VMF Arch" }
    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let m = parse_vmf()?;
        let vmf_arch = m.get("ARCHITECTURE")?.clone();
        if vmf_arch.is_empty() { return None; }
        let sys_arch = std::process::Command::new("uname")
            .arg("-m").output().ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();
        if !sys_arch.is_empty() && sys_arch == vmf_arch { None }
        else { Some(format!("{} (system: {})", vmf_arch, sys_arch)) }
    }
}
