use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::env;

pub struct ShellModule;

impl Module for ShellModule {
    fn name(&self) -> &'static str { "Shell" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        env::var("SHELL")
            .ok()
            .map(|s| {
                std::path::Path::new(&s)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or(s)
            })
    }
}
