use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct CpuModule;

impl Module for CpuModule {
    fn name(&self) -> &'static str { "CPU" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let mut s = System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
        );
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        s.refresh_cpu_usage();

        let cpus = s.cpus();
        if cpus.is_empty() {
            return None;
        }

        let brand = cpus[0].brand().trim().to_string();
        let cores = cpus.len();
        let usage: f32 = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cores as f32;
        let freq = cpus[0].frequency();

        Some(format!(
            "{} ({} cores) @ {:.0} MHz  [{:.1}% load]",
            brand, cores, freq, usage
        ))
    }
}
