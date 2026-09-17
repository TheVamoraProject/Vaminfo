use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct LoadAverageModule;

impl Module for LoadAverageModule {
    fn name(&self) -> &'static str { "Load" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let load = System::load_average();
        if !load.one.is_finite() || !load.five.is_finite() || !load.fifteen.is_finite() {
            return None;
        }
        Some(format!(
            "1m {:.2}  5m {:.2}  15m {:.2}",
            load.one, load.five, load.fifteen
        ))
    }
}