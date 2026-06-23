pub mod battery;
pub mod bios;
pub mod bluetooth;
pub mod color_blocks;
pub mod cpu;
pub mod desktop;
pub mod disk;
pub mod gpu;
pub mod hostname;
pub mod kernel;
pub mod local_ip;
pub mod media;
pub mod network;
pub mod os_info;
pub mod ram;
pub mod resolution;
pub mod shell;
pub mod terminal;
pub mod theme;
pub mod uptime;

use crate::config::VaminfoConfig;
use sysinfo::System;

pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self, sys: &System, cfg: &VaminfoConfig) -> Option<String>;
}

pub fn build_modules(cfg: &VaminfoConfig) -> Vec<Box<dyn Module>> {
    let mut modules: Vec<Box<dyn Module>> = Vec::new();

    if cfg.modules.hostname   { modules.push(Box::new(hostname::HostnameModule)); }
    if cfg.modules.os         { modules.push(Box::new(os_info::OsModule)); }
    if cfg.modules.kernel     { modules.push(Box::new(kernel::KernelModule)); }
    if cfg.modules.bios       { modules.push(Box::new(bios::BiosModule)); }
    if cfg.modules.cpu        { modules.push(Box::new(cpu::CpuModule)); }
    if cfg.modules.gpu        { modules.push(Box::new(gpu::GpuModule)); }
    if cfg.modules.ram        { modules.push(Box::new(ram::RamModule)); }
    if cfg.modules.disk       { modules.push(Box::new(disk::DiskModule)); }
    if cfg.modules.uptime     { modules.push(Box::new(uptime::UptimeModule)); }
    if cfg.modules.shell      { modules.push(Box::new(shell::ShellModule)); }
    if cfg.modules.terminal   { modules.push(Box::new(terminal::TerminalModule)); }
    if cfg.modules.desktop    { modules.push(Box::new(desktop::DesktopModule)); }
    if cfg.modules.resolution { modules.push(Box::new(resolution::ResolutionModule)); }
    if cfg.modules.theme      { modules.push(Box::new(theme::ThemeModule)); }
    if cfg.modules.local_ip   { modules.push(Box::new(local_ip::LocalIpModule)); }
    if cfg.modules.bluetooth  { modules.push(Box::new(bluetooth::BluetoothModule)); }
    if cfg.modules.battery    { modules.push(Box::new(battery::BatteryModule)); }
    if cfg.modules.network    { modules.push(Box::new(network::NetworkModule)); }
    if cfg.modules.media      { modules.push(Box::new(media::MediaModule)); }

    // Color blocks always at the bottom
    if cfg.modules.color_blocks_small { modules.push(Box::new(color_blocks::ColorBlocksSmall)); }
    if cfg.modules.color_blocks_big   { modules.push(Box::new(color_blocks::ColorBlocksBig)); }

    modules
}

pub fn build_mini_modules() -> Vec<Box<dyn Module>> {
    vec![
        Box::new(os_info::OsModule),
        Box::new(cpu::CpuModule),
        Box::new(ram::RamModule),
        Box::new(uptime::UptimeModule),
    ]
}
