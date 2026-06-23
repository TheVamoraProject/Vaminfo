use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::{Networks, System};

pub struct NetworkModule;

impl Module for NetworkModule {
    fn name(&self) -> &'static str { "Network" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let networks = Networks::new_with_refreshed_list();
        let mut parts: Vec<String> = Vec::new();

        for (name, data) in networks.list() {
            if name == "lo" || name.starts_with("docker") || name.starts_with("br-") {
                continue;
            }
            let rx = fmt_bytes(data.total_received());
            let tx = fmt_bytes(data.total_transmitted());
            parts.push(format!("{}: ↓{} ↑{}", name, rx, tx));
            if parts.len() >= 3 {
                break;
            }
        }

        if parts.is_empty() { None } else { Some(parts.join("  |  ")) }
    }
}

fn fmt_bytes(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    const KIB: u64 = 1024;
    if bytes >= GIB {
        format!("{:.1}G", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1}M", bytes as f64 / MIB as f64)
    } else {
        format!("{:.0}K", bytes as f64 / KIB as f64)
    }
}
