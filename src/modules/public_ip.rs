use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::process::Command;

pub struct PublicIpModule;

impl Module for PublicIpModule {
    fn name(&self) -> &'static str { "Public IP" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Try several lightweight endpoints with a short timeout
        let endpoints = [
            "https://api.ipify.org",
            "https://ipv4.icanhazip.com",
            "https://checkip.amazonaws.com",
        ];

        for url in &endpoints {
            if let Ok(out) = Command::new("curl")
                .args(["-s", "--max-time", "3", "--connect-timeout", "2", url])
                .output()
            {
                if out.status.success() {
                    let ip = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    if !ip.is_empty() && ip.len() < 50 {
                        return Some(ip);
                    }
                }
            }
        }

        // Try wget as fallback
        if let Ok(out) = Command::new("wget")
            .args(["-qO-", "--timeout=3", "https://api.ipify.org"])
            .output()
        {
            if out.status.success() {
                let ip = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !ip.is_empty() && ip.len() < 50 {
                    return Some(ip);
                }
            }
        }

        None
    }
}
