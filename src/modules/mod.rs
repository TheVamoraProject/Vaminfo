pub mod android;
pub mod battery;
pub mod bios;
pub mod birthday_countdown;
pub mod bluetooth;
pub mod color_blocks;
pub mod cpu;
pub mod desktop;
pub mod disk;
pub mod display_server;
pub mod fs_type;
pub mod gpu;
pub mod hostname;
pub mod jokes;
pub mod kernel;
pub mod local_ip;
pub mod media;
pub mod network;
pub mod os_info;
pub mod packages;
pub mod public_ip;
pub mod quotes;
pub mod ram;
pub mod resolution;
pub mod shell;
pub mod sudo;
pub mod swap;
pub mod sys_age;
pub mod temperature;
pub mod terminal;
pub mod theme;
pub mod tty_type;
pub mod uptime;
pub mod vamora_os;
pub mod load_average;

use crate::config::VaminfoConfig;
use sysinfo::System;

pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self, sys: &System, cfg: &VaminfoConfig) -> Option<String>;
}

/// Return a Nerd Font Material Design icon for a module title.
///
/// These are private-use glyphs from Nerd Fonts, not emoji. The four-point
/// star fallback is Vamora's universal unknown-module symbol.
pub fn module_icon(name: &str) -> &'static str {
    match name {
        "OS"          => "󰌽",
        "VamoraSys"   => "󰒋",
        "VMF Version" => "󰏓",
        "Vamora"      => "󰙵",
        "Host"        => "󰟀",
        "Kernel"      => "󰒓",
        "BIOS"        => "󰘚",
        "Android"     => "󰀲",
        "Device"      => "󰍹",
        "CPU"         => "󰻠",
        "GPU"         => "󰢮",
        "Packages"    => "󰏗",
        "RAM"         => "󰍛",
        "Swap"        => "󰓡",
        "Load"        => "󰘚",
        "Temperature" => "󰔏",
        "Disk"        => "󰋊",
        "Battery"     => "󰁹",
        "Bluetooth"   => "󰂯",
        "Uptime"      => "󰥔",
        "System Age"  => "󰃰",
        "Shell"       => "󰆍",
        "Terminal"    => "",
        "TTY"         => "󰆍",
        "DE / WM"     => "󰨇",
        "Display"     => "󰍹",
        "Resolution"  => "󰲏",
        "Theme"       => "󰔎",
        "Filesystem"  => "󰙅",
        "Sudo"        => "󰕥",
        "Local IP"    => "󰩟",
        "Public IP"   => "󰖟",
        "Network"     => "󰌘",
        "Birthday"    => "󰃩",
        "Quote"       => "󰝗",
        "Joke"        => "󰱨",
        _             => "󰫢",
    }
}

// ── Canonical key order ───────────────────────────────────────────────────────

pub const MODULE_KEYS: &[&str] = &[
    "os",
    "vamorasys_version",
    "vmf_version",
    "vamora_version_codename",
    "hostname",
    "kernel",
    "bios",
    "android_version",
    "android_device",
    "cpu",
    "gpu",
    "packages",
    "ram",
    "swap",
    "load_average",
    "temperature",
    "disk",
    "battery",
    "bluetooth",
    "uptime",
    "sys_age",
    "shell",
    "terminal",
    "tty_type",
    "desktop",
    "display_server",
    "resolution",
    "theme",
    "fs_type",
    "sudo_status",
    "local_ip",
    "public_ip",
    "network",
    "birthday_countdown",
    "quotes",
    "jokes",
    "color_blocks_big",
    "color_blocks_small",
];

// ── Effective display order ───────────────────────────────────────────────────
// Uses saved order if present, else MODULE_KEYS default.
// Always appends any keys that exist in MODULE_KEYS but are missing from saved order
// so new modules added in future updates are never silently dropped.

pub fn effective_order(cfg: &VaminfoConfig) -> Vec<String> {
    if cfg.module_order.is_empty() {
        MODULE_KEYS.iter().map(|s| s.to_string()).collect()
    } else {
        let mut order = cfg.module_order.clone();
        for &key in MODULE_KEYS {
            if !order.iter().any(|k| k == key) {
                order.push(key.to_string());
            }
        }
        order
    }
}

// ── Enable/disable state ──────────────────────────────────────────────────────

pub fn is_enabled(cfg: &VaminfoConfig, key: &str) -> bool {
    match key {
        "os"                      => cfg.modules.os,
        "vamorasys_version"       => cfg.modules.vamorasys_version,
        "vmf_version"             => cfg.modules.vmf_version,
        "vamora_version_codename" => cfg.modules.vamora_version_codename,
        "hostname"                => cfg.modules.hostname,
        "kernel"                  => cfg.modules.kernel,
        "bios"                    => cfg.modules.bios,
        "android_version"         => cfg.modules.android_version,
        "android_device"          => cfg.modules.android_device,
        "cpu"                     => cfg.modules.cpu,
        "gpu"                     => cfg.modules.gpu,
        "packages"                => cfg.modules.packages,
        "ram"                     => cfg.modules.ram,
        "swap"                    => cfg.modules.swap,
        "load_average"            => cfg.modules.load_average,
        "temperature"             => cfg.modules.temperature,
        "disk"                    => cfg.modules.disk,
        "battery"                 => cfg.modules.battery,
        "bluetooth"               => cfg.modules.bluetooth,
        "uptime"                  => cfg.modules.uptime,
        "sys_age"                 => cfg.modules.sys_age,
        "shell"                   => cfg.modules.shell,
        "terminal"                => cfg.modules.terminal,
        "tty_type"                => cfg.modules.tty_type,
        "desktop"                 => cfg.modules.desktop,
        "display_server"          => cfg.modules.display_server,
        "resolution"              => cfg.modules.resolution,
        "theme"                   => cfg.modules.theme,
        "fs_type"                 => cfg.modules.fs_type,
        "sudo_status"             => cfg.modules.sudo_status,
        "local_ip"                => cfg.modules.local_ip,
        "public_ip"               => cfg.modules.public_ip,
        "network"                 => cfg.modules.network,
        "birthday_countdown"      => cfg.modules.birthday_countdown,
        "quotes"                  => cfg.modules.quotes,
        "jokes"                   => cfg.modules.jokes,
        "color_blocks_big"        => cfg.modules.color_blocks_big,
        "color_blocks_small"      => cfg.modules.color_blocks_small,
        _                         => false,
    }
}

pub fn set_enabled(cfg: &mut VaminfoConfig, key: &str, val: bool) {
    match key {
        "os"                      => cfg.modules.os = val,
        "vamorasys_version"       => cfg.modules.vamorasys_version = val,
        "vmf_version"             => cfg.modules.vmf_version = val,
        "vamora_version_codename" => cfg.modules.vamora_version_codename = val,
        "hostname"                => cfg.modules.hostname = val,
        "kernel"                  => cfg.modules.kernel = val,
        "bios"                    => cfg.modules.bios = val,
        "android_version"         => cfg.modules.android_version = val,
        "android_device"          => cfg.modules.android_device = val,
        "cpu"                     => cfg.modules.cpu = val,
        "gpu"                     => cfg.modules.gpu = val,
        "packages"                => cfg.modules.packages = val,
        "ram"                     => cfg.modules.ram = val,
        "swap"                    => cfg.modules.swap = val,
        "load_average"            => cfg.modules.load_average = val,
        "temperature"             => cfg.modules.temperature = val,
        "disk"                    => cfg.modules.disk = val,
        "battery"                 => cfg.modules.battery = val,
        "bluetooth"               => cfg.modules.bluetooth = val,
        "uptime"                  => cfg.modules.uptime = val,
        "sys_age"                 => cfg.modules.sys_age = val,
        "shell"                   => cfg.modules.shell = val,
        "terminal"                => cfg.modules.terminal = val,
        "tty_type"                => cfg.modules.tty_type = val,
        "desktop"                 => cfg.modules.desktop = val,
        "display_server"          => cfg.modules.display_server = val,
        "resolution"              => cfg.modules.resolution = val,
        "theme"                   => cfg.modules.theme = val,
        "fs_type"                 => cfg.modules.fs_type = val,
        "sudo_status"             => cfg.modules.sudo_status = val,
        "local_ip"                => cfg.modules.local_ip = val,
        "public_ip"               => cfg.modules.public_ip = val,
        "network"                 => cfg.modules.network = val,
        "birthday_countdown"      => cfg.modules.birthday_countdown = val,
        "quotes"                  => cfg.modules.quotes = val,
        "jokes"                   => cfg.modules.jokes = val,
        "color_blocks_big"        => cfg.modules.color_blocks_big = val,
        "color_blocks_small"      => cfg.modules.color_blocks_small = val,
        _ => {}
    }
}

fn make_module(key: &str) -> Option<Box<dyn Module>> {
    match key {
        "os"                      => Some(Box::new(os_info::OsModule)),
        "vamorasys_version"       => Some(Box::new(vamora_os::VamoraSysVersionModule)),
        "vmf_version"             => Some(Box::new(vamora_os::VmfVersionModule)),
        "vamora_version_codename" => Some(Box::new(vamora_os::VamoraVersionCodenameModule)),
        "hostname"                => Some(Box::new(hostname::HostnameModule)),
        "kernel"                  => Some(Box::new(kernel::KernelModule)),
        "bios"                    => Some(Box::new(bios::BiosModule)),
        "android_version"         => Some(Box::new(android::AndroidVersionModule)),
        "android_device"          => Some(Box::new(android::AndroidDeviceModule)),
        "cpu"                     => Some(Box::new(cpu::CpuModule)),
        "gpu"                     => Some(Box::new(gpu::GpuModule)),
        "packages"                => Some(Box::new(packages::PackagesModule)),
        "ram"                     => Some(Box::new(ram::RamModule)),
        "swap"                    => Some(Box::new(swap::SwapModule)),
        "load_average"            => Some(Box::new(load_average::LoadAverageModule)),
        "temperature"             => Some(Box::new(temperature::TemperatureModule)),
        "disk"                    => Some(Box::new(disk::DiskModule)),
        "battery"                 => Some(Box::new(battery::BatteryModule)),
        "bluetooth"               => Some(Box::new(bluetooth::BluetoothModule)),
        "uptime"                  => Some(Box::new(uptime::UptimeModule)),
        "sys_age"                 => Some(Box::new(sys_age::SysAgeModule)),
        "shell"                   => Some(Box::new(shell::ShellModule)),
        "terminal"                => Some(Box::new(terminal::TerminalModule)),
        "tty_type"                => Some(Box::new(tty_type::TtyTypeModule)),
        "desktop"                 => Some(Box::new(desktop::DesktopModule)),
        "display_server"          => Some(Box::new(display_server::DisplayServerModule)),
        "resolution"              => Some(Box::new(resolution::ResolutionModule)),
        "theme"                   => Some(Box::new(theme::ThemeModule)),
        "fs_type"                 => Some(Box::new(fs_type::FsTypeModule)),
        "sudo_status"             => Some(Box::new(sudo::SudoModule)),
        "local_ip"                => Some(Box::new(local_ip::LocalIpModule)),
        "public_ip"               => Some(Box::new(public_ip::PublicIpModule)),
        "network"                 => Some(Box::new(network::NetworkModule)),
        "birthday_countdown"      => Some(Box::new(birthday_countdown::BirthdayCountdownModule)),
        "quotes"                  => Some(Box::new(quotes::QuotesModule)),
        "jokes"                   => Some(Box::new(jokes::JokesModule)),
        "color_blocks_big"        => Some(Box::new(color_blocks::ColorBlocksBig)),
        "color_blocks_small"      => Some(Box::new(color_blocks::ColorBlocksSmall)),
        _                         => None,
    }
}

// ── Build the ordered, filtered module list ───────────────────────────────────

pub fn build_modules(cfg: &VaminfoConfig) -> Vec<Box<dyn Module>> {
    effective_order(cfg)
        .into_iter()
        .filter(|k| is_enabled(cfg, k))
        .filter_map(|k| make_module(&k))
        .collect()
}

// ── Mini mode (fixed 4 modules, not affected by order) ───────────────────────

pub fn build_mini_modules() -> Vec<Box<dyn Module>> {
    vec![
        Box::new(os_info::OsModule),
        Box::new(hostname::HostnameModule),
        Box::new(ram::RamModule),
        Box::new(uptime::UptimeModule),
    ]
}

// ── JSON export ───────────────────────────────────────────────────────────────

pub fn collect_all(cfg: &VaminfoConfig) -> Vec<(String, String)> {
    let sys = System::new_all();
    build_modules(cfg)
        .iter()
        .filter_map(|m| {
            let name = m.name();
            if name.is_empty() { return None; }
            m.collect(&sys, cfg).map(|v| (name.to_string(), v))
        })
        .collect()
}
