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
pub mod public_ip;
pub mod quotes;
pub mod ram;
pub mod resolution;
pub mod shell;
pub mod sudo;
pub mod sys_age;
pub mod terminal;
pub mod theme;
pub mod tty_type;
pub mod uptime;
pub mod vamora_os;

use crate::config::VaminfoConfig;
use sysinfo::System;

pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self, sys: &System, cfg: &VaminfoConfig) -> Option<String>;
}

pub fn build_modules(cfg: &VaminfoConfig) -> Vec<Box<dyn Module>> {
    let mut m: Vec<Box<dyn Module>> = Vec::new();

    // ── OS (smart: VamoraOS PRETTY_NAME / Android / Linux) ───────────────────
    if cfg.modules.os { m.push(Box::new(os_info::OsModule)); }

    // ── VamoraOS sub-details (only visible when release file exists) ──────────
    if cfg.modules.vamora_os {
        m.push(Box::new(vamora_os::VamoraVersionModule));
        m.push(Box::new(vamora_os::VamoraCodenameModule));
        m.push(Box::new(vamora_os::VamoraBuildModule));
        m.push(Box::new(vamora_os::VamoraArchModule));
    }

    // ── System identity ───────────────────────────────────────────────────────
    if cfg.modules.hostname { m.push(Box::new(hostname::HostnameModule)); }
    if cfg.modules.kernel   { m.push(Box::new(kernel::KernelModule)); }
    if cfg.modules.bios     { m.push(Box::new(bios::BiosModule)); }

    // ── Android sub-details (SDK/patch + device, only on Android) ────────────
    if cfg.modules.android_version { m.push(Box::new(android::AndroidVersionModule)); }
    if cfg.modules.android_device  { m.push(Box::new(android::AndroidDeviceModule)); }

    // ── Hardware ──────────────────────────────────────────────────────────────
    if cfg.modules.cpu       { m.push(Box::new(cpu::CpuModule)); }
    if cfg.modules.gpu       { m.push(Box::new(gpu::GpuModule)); }
    if cfg.modules.ram       { m.push(Box::new(ram::RamModule)); }
    if cfg.modules.disk      { m.push(Box::new(disk::DiskModule)); }
    if cfg.modules.battery   { m.push(Box::new(battery::BatteryModule)); }
    if cfg.modules.bluetooth { m.push(Box::new(bluetooth::BluetoothModule)); }

    // ── Time ──────────────────────────────────────────────────────────────────
    if cfg.modules.uptime  { m.push(Box::new(uptime::UptimeModule)); }
    if cfg.modules.sys_age { m.push(Box::new(sys_age::SysAgeModule)); }

    // ── Environment ───────────────────────────────────────────────────────────
    if cfg.modules.shell          { m.push(Box::new(shell::ShellModule)); }
    if cfg.modules.terminal       { m.push(Box::new(terminal::TerminalModule)); }
    if cfg.modules.tty_type       { m.push(Box::new(tty_type::TtyTypeModule)); }
    if cfg.modules.desktop        { m.push(Box::new(desktop::DesktopModule)); }
    if cfg.modules.display_server { m.push(Box::new(display_server::DisplayServerModule)); }
    if cfg.modules.resolution     { m.push(Box::new(resolution::ResolutionModule)); }
    if cfg.modules.theme          { m.push(Box::new(theme::ThemeModule)); }
    if cfg.modules.fs_type        { m.push(Box::new(fs_type::FsTypeModule)); }
    if cfg.modules.sudo_status    { m.push(Box::new(sudo::SudoModule)); }

    // ── Network ───────────────────────────────────────────────────────────────
    if cfg.modules.local_ip  { m.push(Box::new(local_ip::LocalIpModule)); }
    if cfg.modules.public_ip { m.push(Box::new(public_ip::PublicIpModule)); }
    if cfg.modules.network   { m.push(Box::new(network::NetworkModule)); }

    // ── Fun / Optional ────────────────────────────────────────────────────────
    if cfg.modules.birthday_countdown { m.push(Box::new(birthday_countdown::BirthdayCountdownModule)); }
    if cfg.modules.media   { m.push(Box::new(media::MediaModule)); }
    if cfg.modules.quotes  { m.push(Box::new(quotes::QuotesModule)); }
    if cfg.modules.jokes   { m.push(Box::new(jokes::JokesModule)); }

    // ── Color blocks always last ──────────────────────────────────────────────
    if cfg.modules.color_blocks_small { m.push(Box::new(color_blocks::ColorBlocksSmall)); }
    if cfg.modules.color_blocks_big   { m.push(Box::new(color_blocks::ColorBlocksBig)); }

    m
}

/// Mini mode: OS (smart) + Host + RAM + Uptime (no ASCII art)
pub fn build_mini_modules() -> Vec<Box<dyn Module>> {
    vec![
        Box::new(os_info::OsModule),
        Box::new(hostname::HostnameModule),
        Box::new(ram::RamModule),
        Box::new(uptime::UptimeModule),
    ]
}

/// Collect (module_name, value) pairs for JSON export
pub fn collect_all(cfg: &VaminfoConfig) -> Vec<(String, String)> {
    let sys = System::new_all();
    build_modules(cfg)
        .iter()
        .filter_map(|module| {
            let name = module.name();
            if name.is_empty() { return None; }
            module.collect(&sys, cfg).map(|v| (name.to_string(), v))
        })
        .collect()
}
