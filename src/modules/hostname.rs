use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct HostnameModule;

impl Module for HostnameModule {
    fn name(&self) -> &'static str { "Host" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        System::host_name()
    }
}
