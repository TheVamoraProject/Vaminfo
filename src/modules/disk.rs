use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::{Disks, System};

pub struct DiskModule;

impl Module for DiskModule {
    fn name(&self) -> &'static str { "Disk" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let disks = Disks::new_with_refreshed_list();
        let mut parts: Vec<String> = Vec::new();

        for disk in disks.list() {
            let mount = disk.mount_point().to_string_lossy();
            if mount != "/" && !mount.starts_with("/home") {
                continue;
            }
            let total = disk.total_space();
            let free = disk.available_space();
            let used = total.saturating_sub(free);
            let pct = if total > 0 { used as f64 / total as f64 * 100.0 } else { 0.0 };
            parts.push(format!(
                "{}: {} / {} ({:.1}%)",
                mount,
                fmt_bytes(used),
                fmt_bytes(total),
                pct
            ));
        }

        if parts.is_empty() { None } else { Some(parts.join("  |  ")) }
    }
}

fn fmt_bytes(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else {
        format!("{:.0} MiB", bytes as f64 / MIB as f64)
    }
}
