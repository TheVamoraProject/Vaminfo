use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct SwapModule;

impl Module for SwapModule {
    fn name(&self) -> &'static str { "Swap" }

    fn collect(&self, sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let total = sys.total_swap();
        if total == 0 {
            return Some("disabled".to_string());
        }

        let used = sys.used_swap();
        let percent = used as f64 / total as f64 * 100.0;
        Some(format!(
            "{} / {} ({:.1}%)",
            fmt_bytes(used),
            fmt_bytes(total),
            percent
        ))
    }
}

fn fmt_bytes(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else {
        format!("{:.0} MiB", bytes as f64 / MIB as f64)
    }
}