use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub struct RamModule;

impl Module for RamModule {
    fn name(&self) -> &'static str { "RAM" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let mut s = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        s.refresh_memory();

        let total = s.total_memory();
        let used = s.used_memory();
        let percent = if total > 0 { used as f64 / total as f64 * 100.0 } else { 0.0 };

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
