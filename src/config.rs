use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// ── Top-level config ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaminfoConfig {
    pub ascii_file:       String,
    pub ascii_color:      String,
    pub title_color:      String,
    pub key_color:        String,
    pub value_color:      String,
    pub separator:        String,
    pub mini_mode:        bool,
    pub show_title:       bool,
    pub show_separator:   bool,
    #[serde(default)]
    pub greetings:        GreetingsConfig,
    pub modules:          ModuleConfig,
}

// ── Greetings / Events ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreetingsConfig {
    #[serde(default)]
    pub enabled:  bool,
    /// MM-DD, e.g. "07-15"
    #[serde(default)]
    pub birthday: String,
    #[serde(default)]
    pub events:   Vec<GreetingEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreetingEvent {
    /// MM-DD
    pub date:    String,
    pub name:    String,
    pub message: String,
}

// ── Module toggles ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    // Core
    pub hostname:           bool,
    pub os:                 bool,
    pub kernel:             bool,
    pub bios:               bool,
    pub cpu:                bool,
    pub gpu:                bool,
    pub ram:                bool,
    pub disk:               bool,
    pub uptime:             bool,
    pub shell:              bool,
    pub terminal:           bool,
    pub desktop:            bool,
    pub resolution:         bool,
    pub display_server:     bool,
    pub theme:              bool,
    pub tty_type:           bool,
    pub fs_type:            bool,
    pub sys_age:            bool,
    // Network
    pub local_ip:           bool,
    pub public_ip:          bool,
    pub network:            bool,
    // Hardware
    pub bluetooth:          bool,
    pub battery:            bool,
    // Android
    pub android_version:    bool,
    pub android_device:     bool,
    // System
    pub sudo_status:        bool,
    pub birthday_countdown: bool,
    pub vamora_os:          bool,
    // Fun
    pub quotes:             bool,
    pub jokes:              bool,
    pub media:              bool,
    // Display
    pub color_blocks_big:   bool,
    pub color_blocks_small: bool,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            hostname:           true,
            os:                 true,
            kernel:             true,
            bios:               true,
            cpu:                true,
            gpu:                true,
            ram:                true,
            disk:               true,
            uptime:             false,
            shell:              true,
            terminal:           true,
            desktop:            true,
            resolution:         true,
            display_server:     true,
            theme:              true,
            tty_type:           true,
            fs_type:            true,
            sys_age:            true,
            local_ip:           true,
            public_ip:          false,  // makes a network request
            network:            true,
            bluetooth:          true,
            battery:            true,
            android_version:    true,   // only shows on Android
            android_device:     true,   // only shows on Android
            sudo_status:        true,
            birthday_countdown: false,  // requires birthday set in greetings config
            vamora_os:          true,   // only shows if /etc/VamoraSys/vamora-release.vmf exists
            quotes:             false,
            jokes:              false,
            media:              false,
            color_blocks_big:   true,
            color_blocks_small: false,
        }
    }
}

impl Default for VaminfoConfig {
    fn default() -> Self {
        Self {
            ascii_file:      "ascii1.vtxt".to_string(),
            ascii_color:     "blue".to_string(),
            title_color:     "bright_blue".to_string(),
            key_color:       "bright_blue".to_string(),
            value_color:     "white".to_string(),
            separator:       "-".to_string(),
            mini_mode:       false,
            show_title:      true,
            show_separator:  true,
            greetings:       GreetingsConfig::default(),
            modules:         ModuleConfig::default(),
        }
    }
}

// ── Paths ─────────────────────────────────────────────────────────────────────

pub fn config_path() -> PathBuf {
    dirs_home()
        .join(".VamoraSys")
        .join("apps")
        .join("vaminfo")
        .join("config.vmf")
}

pub fn art_dir() -> PathBuf {
    dirs_home()
        .join(".VamoraSys")
        .join("apps")
        .join("vaminfo")
        .join("art")
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

// ── Load / Save ───────────────────────────────────────────────────────────────

impl VaminfoConfig {
    pub fn load_or_create() -> Self {
        let path = config_path();
        if path.exists() {
            match Self::load_from(&path) {
                Ok(cfg) => return cfg,
                Err(_) => {
                    eprintln!("[vaminfo] Config parse error -- using defaults.");
                    return Self::default();
                }
            }
        }
        let cfg = Self::default();
        cfg.save_creating_dirs();
        cfg
    }

    fn load_from(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let cfg: Self = toml::from_str(&content)?;
        Ok(cfg)
    }

    pub fn save(&self) {
        self.save_creating_dirs();
    }

    fn save_creating_dirs(&self) {
        let path = config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::create_dir_all(art_dir());
        let content = toml::to_string_pretty(self).unwrap_or_default();
        let _ = fs::write(&path, content);
    }
}
