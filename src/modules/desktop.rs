use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::env;

pub struct DesktopModule;

impl Module for DesktopModule {
    fn name(&self) -> &'static str { "DE / WM" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let de = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .or_else(|_| env::var("GDMSESSION"))
            .ok();

        let wm = env::var("WAYLAND_DISPLAY")
            .map(|_| "Wayland".to_string())
            .or_else(|_| env::var("DISPLAY").map(|_| "X11".to_string()))
            .ok();

        match (de, wm) {
            (Some(de), Some(wm)) => Some(format!("{} ({})", de, wm)),
            (Some(de), None)     => Some(de),
            (None, Some(wm))     => Some(wm),
            _                    => None,
        }
    }
}
