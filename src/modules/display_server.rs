use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct DisplayServerModule;

impl Module for DisplayServerModule {
    fn name(&self) -> &'static str { "Display" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Wayland check
        if let Ok(d) = std::env::var("WAYLAND_DISPLAY") {
            if !d.is_empty() {
                let compositor = detect_wayland_compositor();
                return Some(if compositor.is_empty() {
                    format!("Wayland ({})", d)
                } else {
                    format!("Wayland ({}) [{}]", d, compositor)
                });
            }
        }

        // Mir
        if std::env::var("MIR_SOCKET").is_ok() {
            return Some("Mir".to_string());
        }

        // X11
        if let Ok(d) = std::env::var("DISPLAY") {
            if !d.is_empty() {
                return Some(format!("X11 ({})", d));
            }
        }

        // macOS Quartz
        #[cfg(target_os = "macos")]
        return Some("Quartz (macOS)".to_string());

        // Android
        if is_android() {
            return Some("SurfaceFlinger (Android)".to_string());
        }

        Some("TTY (no display server)".to_string())
    }
}

fn detect_wayland_compositor() -> String {
    // Check common compositor indicators
    if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() { return "Hyprland".to_string(); }
    if std::env::var("SWAYSOCK").is_ok()                    { return "Sway".to_string(); }
    if std::env::var("KDE_FULL_SESSION").is_ok()            { return "KWin".to_string(); }
    if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
        if de.to_lowercase().contains("gnome")   { return "Mutter".to_string(); }
        if de.to_lowercase().contains("kde")     { return "KWin".to_string(); }
        if de.to_lowercase().contains("sway")    { return "Sway".to_string(); }
    }
    String::new()
}

fn is_android() -> bool {
    std::env::var("TERMUX_VERSION").is_ok()
        || std::path::Path::new("/system/build.prop").exists()
}
