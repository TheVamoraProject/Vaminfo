use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// ── Default-value helpers ─────────────────────────────────────────────────────
// serde needs free functions for #[serde(default = "...")]

fn default_ascii_file()  -> String { "ascii1.vtxt".to_string() }
fn default_ascii_color() -> String { "blue".to_string() }
fn default_title_color() -> String { "bright_blue".to_string() }
fn default_key_color()   -> String { "bright_blue".to_string() }
fn default_value_color() -> String { "white".to_string() }
fn default_separator()   -> String { "-".to_string() }
fn default_true()        -> bool   { true }

// ── Top-level config ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaminfoConfig {
    #[serde(default = "default_ascii_file")]
    pub ascii_file:     String,
    #[serde(default = "default_ascii_color")]
    pub ascii_color:    String,
    #[serde(default = "default_title_color")]
    pub title_color:    String,
    #[serde(default = "default_key_color")]
    pub key_color:      String,
    #[serde(default = "default_value_color")]
    pub value_color:    String,
    #[serde(default = "default_separator")]
    pub separator:      String,
    #[serde(default)]
    pub mini_mode:      bool,
    #[serde(default = "default_true")]
    pub show_title:     bool,
    #[serde(default = "default_true")]
    pub show_separator: bool,
    /// Render Nerd Font icons before module titles.
    #[serde(default = "default_true")]
    pub icons_enabled:  bool,
    #[serde(default)]
    pub greetings:      GreetingsConfig,
    #[serde(default)]
    pub modules:        ModuleConfig,
    /// Saved display order — empty means use the built-in default order.
    #[serde(default)]
    pub module_order:   Vec<String>,
}

// ── Greetings / Events ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreetingsConfig {
    #[serde(default)]
    pub enabled:  bool,
    #[serde(default)]
    pub birthday: String,
    #[serde(default)]
    pub events:   Vec<GreetingEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreetingEvent {
    pub date:    String,
    pub name:    String,
    pub message: String,
}

// ── Module toggles ────────────────────────────────────────────────────────────
//
// Fields that default ON  → #[serde(default = "default_true")]
// Fields that default OFF → #[serde(default)]   (bool default = false)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    // Core
    #[serde(default = "default_true")]  pub hostname:           bool,
    #[serde(default = "default_true")]  pub os:                 bool,
    #[serde(default = "default_true")]  pub kernel:             bool,
    #[serde(default = "default_true")]  pub bios:               bool,
    #[serde(default = "default_true")]  pub cpu:                bool,
    #[serde(default = "default_true")]  pub gpu:                bool,
    #[serde(default = "default_true")]  pub packages:           bool,
    #[serde(default = "default_true")]  pub ram:                bool,
    #[serde(default = "default_true")]  pub swap:                bool,
    #[serde(default = "default_true")]  pub load_average:        bool,
    #[serde(default = "default_true")]  pub temperature:         bool,
    #[serde(default = "default_true")]  pub disk:               bool,
    #[serde(default)]                   pub uptime:             bool,
    #[serde(default = "default_true")]  pub shell:              bool,
    #[serde(default = "default_true")]  pub terminal:           bool,
    #[serde(default = "default_true")]  pub desktop:            bool,
    #[serde(default = "default_true")]  pub resolution:         bool,
    #[serde(default = "default_true")]  pub display_server:     bool,
    #[serde(default = "default_true")]  pub theme:              bool,
    #[serde(default = "default_true")]  pub tty_type:           bool,
    #[serde(default = "default_true")]  pub fs_type:            bool,
    #[serde(default = "default_true")]  pub sys_age:            bool,
    // Network
    #[serde(default = "default_true")]  pub local_ip:           bool,
    #[serde(default)]                   pub public_ip:          bool,
    #[serde(default = "default_true")]  pub network:            bool,
    // Hardware
    #[serde(default = "default_true")]  pub bluetooth:          bool,
    #[serde(default = "default_true")]  pub battery:            bool,
    // Android (only shows on Android / Termux)
    #[serde(default = "default_true")]  pub android_version:    bool,
    #[serde(default = "default_true")]  pub android_device:     bool,
    // System
    #[serde(default = "default_true")]  pub sudo_status:        bool,
    #[serde(default)]                   pub birthday_countdown:      bool,
    // VamoraOS (each only shows if /etc/VamoraSys/vamora-release.vmf exists)
    #[serde(default = "default_true")]  pub vamorasys_version:       bool,
    #[serde(default = "default_true")]  pub vmf_version:             bool,
    #[serde(default = "default_true")]  pub vamora_version_codename: bool,
    // Fun / Optional
    #[serde(default)]                   pub quotes:             bool,
    #[serde(default)]                   pub jokes:              bool,
    // Display
    #[serde(default = "default_true")]  pub color_blocks_big:   bool,
    #[serde(default)]                   pub color_blocks_small: bool,
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
            packages:           true,
            ram:                true,
            swap:               true,
            load_average:       true,
            temperature:        true,
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
            public_ip:          false,
            network:            true,
            bluetooth:          true,
            battery:            true,
            android_version:    true,
            android_device:     true,
            sudo_status:             true,
            birthday_countdown:      false,
            vamorasys_version:       true,
            vmf_version:             true,
            vamora_version_codename: true,
            quotes:             false,
            jokes:              false,
            color_blocks_big:   true,
            color_blocks_small: false,
        }
    }
}

impl Default for VaminfoConfig {
    fn default() -> Self {
        Self {
            ascii_file:    "ascii1.vtxt".to_string(),
            ascii_color:   "blue".to_string(),
            title_color:   "bright_blue".to_string(),
            key_color:     "bright_blue".to_string(),
            value_color:   "white".to_string(),
            separator:     "-".to_string(),
            mini_mode:     false,
            show_title:    true,
            show_separator: true,
            icons_enabled: true,
            greetings:     GreetingsConfig::default(),
            modules:       ModuleConfig::default(),
            module_order:  vec![],
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
                Err(e) => {
                    eprintln!("[vaminfo] Config parse error ({}), using defaults.", e);
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
        // toml + serde: unknown keys are silently ignored (no deny_unknown_fields),
        // and every field has #[serde(default = ...)] so missing keys use proper defaults.
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
