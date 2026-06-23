use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::env;

pub struct TerminalModule;

impl Module for TerminalModule {
    fn name(&self) -> &'static str { "Terminal" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Check known env vars set by specific terminals
        if env::var("TILIX_ID").is_ok() {
            return Some("Tilix".to_string());
        }
        if env::var("KONSOLE_VERSION").is_ok() {
            return Some("Konsole".to_string());
        }
        if let Ok(v) = env::var("KITTY_WINDOW_ID") {
            if !v.is_empty() { return Some("Kitty".to_string()); }
        }
        if env::var("ALACRITTY_SOCKET").is_ok() || env::var("ALACRITTY_LOG").is_ok() {
            return Some("Alacritty".to_string());
        }
        if let Ok(v) = env::var("WEZTERM_EXECUTABLE") {
            if !v.is_empty() { return Some("WezTerm".to_string()); }
        }
        if env::var("TMUX").is_ok() {
            let inner = detect_inner_term();
            return Some(format!("tmux ({})", inner));
        }
        if let Ok(t) = env::var("TERM_PROGRAM") {
            let name = match t.as_str() {
                "iTerm.app"        => "iTerm2",
                "Apple_Terminal"   => "Apple Terminal",
                "vscode"           => "VS Code",
                "WarpTerminal"     => "Warp",
                "Hyper"            => "Hyper",
                _                  => &t,
            };
            return Some(name.to_string());
        }
        if let Ok(t) = env::var("TERMINAL") {
            if !t.is_empty() {
                return Some(
                    std::path::Path::new(&t)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or(t),
                );
            }
        }
        if let Ok(t) = env::var("TERM") {
            let name = match t.as_str() {
                "xterm-256color" | "xterm" => "xterm",
                "rxvt-unicode-256color" | "rxvt-unicode" | "rxvt" => "rxvt",
                "screen-256color" | "screen" => "GNU Screen",
                "linux" => "Linux Console",
                _ => &t,
            };
            return Some(name.to_string());
        }

        Some("Vamora Terminal".to_string())
    }
}

fn detect_inner_term() -> String {
    if let Ok(t) = env::var("TERM_PROGRAM") {
        return t;
    }
    if let Ok(t) = env::var("TERMINAL") {
        if !t.is_empty() {
            return std::path::Path::new(&t)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or(t);
        }
    }
    "unknown".to_string()
}
