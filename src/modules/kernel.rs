use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct KernelModule;

impl Module for KernelModule {
    fn name(&self) -> &'static str { "Kernel" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        System::kernel_version()
    }
}
