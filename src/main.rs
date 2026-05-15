// ---------------------------------------------------------------
// Vaminfo (vamifetch) - A simple system fetch tool for VamoraOS
// https://github.com/TheVamoraProject/Vaminfo
// ---------------------------------------------------------------
//
// MIT License
// Copyright (c) 2025 Vamora

#![allow(dead_code)]

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, exit};

// ========== ANSI helpers ===========================================

const RESET: &str = "\x1b[0m";
const BOLD_YELLOW: &str = "\x1b[93m\x1b[1m";
const CYAN: &str = "\x1b[96m";
const MAGENTA: &str = "\x1b[95m";

fn resolve_color(name: &str) -> &'static str {
    match name {
        "LightBlue"   => "\x1b[1;94m",
        "DeepBlue"    => "\x1b[1;34m",
        "Blue"        => "\x1b[34m",
        "LightGreen"  => "\x1b[1;92m",
        "Green"       => "\x1b[1;32m",
        "DarkGreen"   => "\x1b[32m",
        "LightRed"    => "\x1b[1;91m",
        "Red"         => "\x1b[1;31m",
        "Yellow"      => "\x1b[1;93m",
        "Gold"        => "\x1b[33m",
        "LightPurple" => "\x1b[1;95m",
        "Purple"      => "\x1b[1;35m",
        "Magenta"     => "\x1b[35m",
        "LightCyan"   => "\x1b[1;96m",
        "Cyan"        => "\x1b[1;36m",
        "White"       => "\x1b[1;97m",
        "Gray"        => "\x1b[1;90m",
        "Pink"        => "\x1b[1;95m",
        "Orange"      => "\x1b[38;5;214m",
        _ => "",
    }
}

fn color_names() -> &'static [&'static str] {
    &[
        "LightBlue", "DeepBlue", "Blue",
        "LightGreen", "Green", "DarkGreen",
        "LightRed", "Red",
        "Yellow", "Gold",
        "LightPurple", "Purple", "Magenta",
        "LightCyan", "Cyan",
        "White", "Gray", "Pink", "Orange",
    ]
}

fn is_valid_color(name: &str) -> bool {
    color_names().contains(&name)
}

// ========== Config =================================================

fn config_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| "/root".into());
    PathBuf::from(home).join(".vaminfo")
}

fn config_file() -> PathBuf {
    config_dir().join("config.vmf")
}

fn ensure_config() {
    let dir = config_dir();
    let file = config_file();
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    if !file.exists() {
        let _ = fs::write(&file, "COLOR=\"LightBlue\"\n");
    }
}

fn load_color() -> String {
    let file = config_file();
    if let Ok(content) = fs::read_to_string(&file) {
        for line in content.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("COLOR=") {
                let val = rest.trim_matches('"').trim();
                if !val.is_empty() {
                    return val.to_string();
                }
            }
        }
    }
    "LightBlue".to_string()
}

fn save_color(color: &str) -> bool {
    ensure_config();
    let file = config_file();
    fs::write(&file, format!("COLOR=\"{}\"\n", color)).is_ok()
}

// ========== Custom ASCII ===========================================

fn custom_ascii_dir() -> PathBuf {
    config_dir().join("ascii")
}

fn list_custom_ascii() -> Vec<String> {
    let dir = custom_ascii_dir();
    if !dir.exists() {
        return vec![];
    }
    let mut names = vec![];
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "txt").unwrap_or(false) {
                if let Some(stem) = path.file_stem() {
                    names.push(stem.to_string_lossy().to_string());
                }
            }
        }
    }
    names.sort();
    names
}

fn load_custom_ascii(name: &str) -> Option<Vec<String>> {
    let path = custom_ascii_dir().join(format!("{}.txt", name));
    let content = fs::read_to_string(&path).ok()?;
    Some(content.lines().map(|l| l.to_string()).collect())
}

fn save_custom_ascii(name: &str, lines: &[String]) -> bool {
    let dir = custom_ascii_dir();
    let _ = fs::create_dir_all(&dir);
    let path = dir.join(format!("{}.txt", name));
    fs::write(&path, lines.join("\n")).is_ok()
}

fn delete_custom_ascii(name: &str) -> bool {
    let path = custom_ascii_dir().join(format!("{}.txt", name));
    fs::remove_file(&path).is_ok()
}

fn load_ascii_config() -> Option<String> {
    let file = config_file();
    let content = fs::read_to_string(&file).ok()?;
    for line in content.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("ASCII=") {
            let val = rest.trim_matches('"').trim();
            if !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}

fn save_ascii_config(name: Option<&str>) -> bool {
    ensure_config();
    let file = config_file();
    let content = fs::read_to_string(&file).unwrap_or_default();

    // Remove existing ASCII= lines
    let mut new_lines: Vec<String> = content
        .lines()
        .filter(|l| !l.trim().starts_with("ASCII="))
        .map(|l| l.to_string())
        .collect();

    if let Some(n) = name {
        new_lines.push(format!("ASCII=\"{}\"", n));
    }

    fs::write(&file, new_lines.join("\n") + "\n").is_ok()
}

// ========== System Info ============================================

fn cmd(prog: &str, args: &[&str]) -> String {
    Command::new(prog)
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn get_distro() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("PRETTY_NAME=") {
                return rest.trim_matches('"').to_string();
            }
        }
    }
    "Unknown OS".to_string()
}

fn get_kernel() -> String {
    cmd("uname", &["-r"])
}

fn get_uptime() -> String {
    let raw = cmd("uptime", &["-p"]);
    raw.strip_prefix("up ").unwrap_or(&raw).to_string()
}

fn get_shell() -> String {
    env::var("SHELL").unwrap_or_else(|_| "Unknown".into())
}

fn get_wm() -> String {
    env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "None".into())
}

fn get_cpu() -> String {
    let out = cmd("lscpu", &[]);
    for line in out.lines() {
        if line.starts_with("Model name") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if let Some(val) = parts.get(1) {
                return val.trim().to_string();
            }
        }
    }
    "N/A".to_string()
}

fn get_gpu() -> String {
    let out = cmd("lspci", &[]);
    for line in out.lines() {
        let lower = line.to_lowercase();
        if lower.contains("vga") || lower.contains("3d") {
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if let Some(val) = parts.get(2) {
                return val.trim().to_string();
            }
        }
    }
    "N/A".to_string()
}

fn get_ram() -> String {
    let out = cmd("free", &["-h"]);
    for line in out.lines() {
        if line.starts_with("Mem:") {
            let cols: Vec<&str> = line.split_whitespace().collect();
            return cols.get(1).copied().unwrap_or("N/A").to_string();
        }
    }
    "N/A".to_string()
}

fn get_disk() -> String {
    let out = cmd("df", &["-h", "/"]);
    let lines: Vec<&str> = out.lines().collect();
    if let Some(line) = lines.get(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        let used = cols.get(2).copied().unwrap_or("?");
        let total = cols.get(1).copied().unwrap_or("?");
        return format!("{} / {} used", used, total);
    }
    "N/A".to_string()
}

fn get_battery() -> String {
    // Try /sys/class/power_supply first (no upower needed)
    if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("BAT") {
                let cap_path = entry.path().join("capacity");
                let status_path = entry.path().join("status");
                if let Ok(cap) = fs::read_to_string(&cap_path) {
                    let pct = cap.trim();
                    let status = fs::read_to_string(&status_path)
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    return format!("{}% ({})", pct, status);
                }
            }
        }
    }
    "N/A".to_string()
}

fn get_resolution() -> String {
    let out = cmd("xrandr", &["--current"]);
    for line in out.lines() {
        if line.contains('*') {
            if let Some(res) = line.split_whitespace().next() {
                return res.to_string();
            }
        }
    }
    "N/A".to_string()
}

fn get_vapps() -> String {
    if PathBuf::from("/opt/VamoraApps").exists() {
        "Supported (global)".into()
    } else if PathBuf::from("/VamoraSys/vapps").exists() {
        "Supported (Please update your Vapp installer)".into()
    } else {
        let home = env::var("HOME").unwrap_or_default();
        if PathBuf::from(format!("{}/.vapps", home)).exists() {
            "Supported (multi user)".into()
        } else {
            "Not supported".into()
        }
    }
}

fn get_network() -> (String, String) {
    let connected = Command::new("ping")
        .args(["-c", "1", "-W", "2", "1.1.1.1"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let status = if connected {
        let iface_out = cmd("ip", &["link", "show"]);
        let iface = iface_out
            .lines()
            .find(|l| l.contains("state UP"))
            .and_then(|l| l.split_whitespace().nth(1))
            .map(|s| s.trim_end_matches(':').to_string())
            .unwrap_or_else(|| "unknown".into());
        format!("Connected (interface: {})", iface)
    } else {
        "Not connected".into()
    };

    let ip = if connected {
        cmd("curl", &["-s", "--max-time", "3", "ifconfig.me"])
    } else {
        "N/A".into()
    };

    (status, ip)
}

fn get_terminal() -> String {
    let pid = std::process::id();
    let mut current = pid;
    loop {
        let comm = fs::read_to_string(format!("/proc/{}/comm", current))
            .unwrap_or_default()
            .trim()
            .to_string();
        let term = match comm.as_str() {
            "gnome-terminal" | "gnome-terminal-" | "gnome-terminal-s" => Some("GNOME Terminal"),
            "konsole"        => Some("Konsole"),
            "xterm"          => Some("XTerm"),
            "alacritty"      => Some("Alacritty"),
            "mate-terminal"  => Some("MATE Terminal"),
            "tilix"          => Some("Tilix"),
            "kitty"          => Some("Kitty"),
            "urxvt" | "rxvt" => Some("URxvt"),
            "st"             => Some("ST"),
            "lxterminal"     => Some("LXTerminal"),
            "xfce4-terminal" => Some("XFCE Terminal"),
            "tmux" | "screen"=> None, // skip multiplexers
            _                => None,
        };
        if let Some(t) = term {
            return t.to_string();
        }
        // Get parent pid
        let stat = fs::read_to_string(format!("/proc/{}/stat", current))
            .unwrap_or_default();
        let ppid: u32 = stat
            .split_whitespace()
            .nth(3)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        if ppid == 0 || ppid == current {
            break;
        }
        current = ppid;
    }
    "VamoTerminal".to_string()
}

fn get_vamora_version() -> String {
    let release = fs::read_to_string("/etc/VamoraSys/vamora-release").unwrap_or_default();
    for line in release.lines() {
        if let Some(rest) = line.strip_prefix("VERSION=") {
            return rest.trim_matches('"').to_string();
        }
    }
    "N/A".to_string()
}

// ========== Truncate ===============================================

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() > max {
        format!("{}...", &s.chars().take(max).collect::<String>())
    } else {
        s.to_string()
    }
}

// ========== Default ASCII art ======================================

fn default_ascii(color: &str) -> Vec<String> {
    let c = resolve_color(color);
    let r = RESET;
    vec![
        format!("{}     %%%%%%%%%%%%%%%%%%%%%%%%%%%     {}", c, r),
        format!("{}   %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%   {}", c, r),
        format!("{} %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%% {}", c, r),
        format!("{}%%%%%%%%....*%%%%%%....%%....%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%.....%%%%%.....%.....%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%.....%%%%.....%......%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%.....%%%.....%.......%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%.....%%.....%........%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%.....%.....%.........%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%.....#....+..........%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%..........%....-.....%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%.........%.....%.....%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%........%.....%%.....%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%.......%.....%%%.....%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%......%.....%%%%.....%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%.....%.....%%%%%.....%%%%%%%%{}", c, r),
        format!("{}%%%%%%%%....+%....%%%%%%%....%%%%%%%%{}", c, r),
        format!("{} %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%% {}", c, r),
        format!("{}   %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%   {}", c, r),
        format!("{}     %%%%%%%%%%%%%%%%%%%%%%%%%%%     {}", c, r),
    ]
}

// ========== Visible length (strip ANSI) ============================

fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if ch == 'm' {
                in_escape = false;
            }
        } else {
            len += 1;
        }
    }
    len
}

// ========== Print side-by-side =====================================

fn print_fetch(ascii_lines: &[String], info_lines: &[String]) {
    let ascii_col_width = ascii_lines
        .iter()
        .map(|l| visible_len(l))
        .max()
        .unwrap_or(36);

    let max = ascii_lines.len().max(info_lines.len());
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for i in 0..max {
        let left = ascii_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        let right = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");

        // Pad left column to ascii_col_width visible chars
        let pad = ascii_col_width.saturating_sub(visible_len(left));
        let padding = " ".repeat(pad);
        writeln!(out, "{}{}  {}", left, padding, right).unwrap();
    }
}

// ========== Color swatch ===========================================

fn color_swatches() -> Vec<String> {
    vec![
        format!(
            " \x1b[40m    \x1b[41m    \x1b[42m    \x1b[43m    \x1b[44m    \x1b[45m    \x1b[46m    \x1b[47m    {}",
            RESET
        ),
        format!(
            " \x1b[100m    \x1b[101m    \x1b[102m    \x1b[103m    \x1b[104m    \x1b[105m    \x1b[106m    \x1b[107m    {}",
            RESET
        ),
    ]
}

// ========== Mini mode ==============================================

fn print_mini(info: &SysInfo) {
    println!(
        r#"__     __
\ \   / /_ _ _ __ ___   ___  _ __ __ _
 \ \ / / _` | '_ ` _ \ / _ \| '__/ _` |
  \ V / (_| | | | | | | (_) | | | (_| |
   \_/ \__,_|_| |_| |_|\___/|_|  \__,_|
----------------------------------------"#
    );
    println!("OS:      {}", info.distro);
    println!("Kernel:  {}", info.kernel);
    println!("CPU:     {}", info.cpu);
    println!("GPU:     {}", info.gpu);
    println!("RAM:     {}", info.ram);
    println!("Disk:    {}", info.disk);
    println!("Network: {}", info.network_status);
}

// ========== SysInfo struct =========================================

struct SysInfo {
    user: String,
    host: String,
    distro: String,
    kernel: String,
    uptime: String,
    shell: String,
    wm: String,
    vamora_version: String,
    vapps: String,
    network_status: String,
    public_ip: String,
    cpu: String,
    gpu: String,
    ram: String,
    disk: String,
    battery: String,
    resolution: String,
    terminal: String,
}

impl SysInfo {
    fn collect() -> Self {
        let (network_status, public_ip) = get_network();
        let cpu = truncate(&get_cpu(), 42);
        let gpu = truncate(&get_gpu(), 30);
        SysInfo {
            user: cmd("whoami", &[]),
            host: cmd("hostname", &[]),
            distro: get_distro(),
            kernel: get_kernel(),
            uptime: get_uptime(),
            shell: get_shell(),
            wm: get_wm(),
            vamora_version: get_vamora_version(),
            vapps: get_vapps(),
            network_status,
            public_ip,
            cpu,
            gpu,
            ram: get_ram(),
            disk: get_disk(),
            battery: get_battery(),
            resolution: get_resolution(),
            terminal: get_terminal(),
        }
    }

    fn info_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("{}{}@{}{}", MAGENTA, self.user, self.host, RESET),
            format!("{}---------------------------{}", CYAN, RESET),
            format!("{}OS:{} {}", BOLD_YELLOW, RESET, self.distro),
            format!("{}Kernel:{} {}", BOLD_YELLOW, RESET, self.kernel),
            format!("{}Uptime:{} {}", BOLD_YELLOW, RESET, self.uptime),
            format!("{}Shell:{} {}", BOLD_YELLOW, RESET, self.shell),
            format!("{}WM/Desktop:{} {}", BOLD_YELLOW, RESET, self.wm),
            format!("{}VamoraSys version:{} {}", BOLD_YELLOW, RESET, self.vamora_version),
            format!("{}VamoraApps support:{} {}", BOLD_YELLOW, RESET, self.vapps),
            format!("{}---------------------------{}", CYAN, RESET),
            format!("{}Network:{} {}", BOLD_YELLOW, RESET, self.network_status),
            format!("{}Public IP:{} {}", BOLD_YELLOW, RESET, self.public_ip),
            format!("{}CPU:{} {}", BOLD_YELLOW, RESET, self.cpu),
            format!("{}GPU:{} {}", BOLD_YELLOW, RESET, self.gpu),
            format!("{}RAM:{} {}", BOLD_YELLOW, RESET, self.ram),
            format!("{}Disk:{} {}", BOLD_YELLOW, RESET, self.disk),
            format!("{}Battery:{} {}", BOLD_YELLOW, RESET, self.battery),
            format!("{}Resolution:{} {}", BOLD_YELLOW, RESET, self.resolution),
            format!("{}Terminal:{} {}", BOLD_YELLOW, RESET, self.terminal),
            format!("{}---------------------------{}", CYAN, RESET),
        ];
        for swatch in color_swatches() {
            lines.push(swatch);
        }
        lines
    }
}

// ========== Entry point ============================================

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = args.get(1).map(|s| s.as_str()).unwrap_or("");
    let arg2 = args.get(2).map(|s| s.as_str()).unwrap_or("");

    match flag {
        "--help" | "-h" => {
            println!("Vaminfo - A simple system fetch tool made just for VamoraOS");
            println!();
            println!("Usage: vaminfo [options]");
            println!();
            println!("Options:");
            println!("  --help,         -h               Show this help message");
            println!("  --version,      -v               Show version info");
            println!("  --update,       -u               Update vaminfo to latest version");
            println!("  --mini,         -m               Show a smaller version of vaminfo");
            println!("  --color,        -c <color>       Set the ASCII art color");
            println!("  --colors                         List all available colors");
            println!();
            println!("  --ascii,        -a <name>        Use a saved custom ASCII art");
            println!("  --ascii-list                     List saved custom ASCII arts");
            println!("  --ascii-add     <name>           Add custom ASCII from stdin");
            println!("  --ascii-delete  <name>           Delete a saved custom ASCII");
            println!("  --ascii-reset                    Reset to default Vamora logo");
            exit(0);
        }

        "--colors" => {
            println!("Available colors for vaminfo:");
            println!();
            println!("  Blues:    LightBlue  DeepBlue  Blue");
            println!("  Greens:   LightGreen Green     DarkGreen");
            println!("  Reds:     LightRed   Red");
            println!("  Yellows:  Yellow     Gold");
            println!("  Purples:  LightPurple Purple   Magenta");
            println!("  Cyans:    LightCyan  Cyan");
            println!("  Others:   White      Gray      Pink     Orange");
            println!();
            println!("Usage: vaminfo --color <ColorName>");
            println!("Example: vaminfo --color LightCyan");
            exit(0);
        }

        "--color" | "-c" => {
            if arg2.is_empty() {
                eprintln!("Please provide a color name. Run 'vaminfo --colors' to see all options.");
                exit(1);
            }
            if !is_valid_color(arg2) {
                eprintln!("Unknown color '{}'. Run 'vaminfo --colors' to see all options.", arg2);
                exit(1);
            }
            if save_color(arg2) {
                println!("Color set to {} and saved.", arg2);
            } else {
                eprintln!("Could not write config file.");
                exit(1);
            }
            exit(0);
        }

        "--mini" | "-m" => {
            let info = SysInfo::collect();
            print_mini(&info);
            exit(0);
        }

        "--version" | "-v" => {
            println!("Vaminfo version: 5.0.0 (Rust edition)");
            exit(0);
        }

        "--update" | "-u" => {
            println!("Checking for updates...");

            // Local version from info.vmf
            let local_ver: u64 = fs::read_to_string("/etc/VamoraSys/default/vaminfo/info.vmf")
                .unwrap_or_default()
                .lines()
                .find(|l| l.starts_with("VAMINFO_VERSION="))
                .and_then(|l| l.split('"').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);

            // ---- Option A: fetch version from GitHub raw ----
            let remote_ver_github: u64 = cmd(
                "curl",
                &[
                    "-s", "--max-time", "5",
                    "https://raw.githubusercontent.com/TheVamoraProject/Vaminfo/main/info.vmf",
                ],
            )
            .lines()
            .find(|l| l.starts_with("VAMINFO_VERSION="))
            .and_then(|l| l.split('"').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

            // ---- Option B: fetch version from website ----
            // let remote_ver_website: u64 = cmd(
            //     "curl",
            //     &[
            //         "-s", "--max-time", "5",
            //         "https://vamora.vercel.app/install/vaminfo-version",
            //     ],
            // )
            // .trim()
            // .parse()
            // .unwrap_or(0);

            let remote_ver = remote_ver_github;

            if remote_ver == 0 {
                eprintln!("Failed to check latest version. Check your internet connection.");
                exit(1);
            }

            if remote_ver > local_ver {
                println!("New version available ({} -> {}). Installing...", local_ver, remote_ver);

                let status = Command::new("sh")
                    .args(["-c", "curl -fsSL https://vamora.vercel.app/install/vaminfo.sh | sudo bash"])
                    .status();

                match status {
                    Ok(s) if s.success() => println!("Update complete!"),
                    _ => { eprintln!("Update failed."); exit(1); }
                }
            } else if remote_ver < local_ver {
                println!("💙 Ooooh beta tester detected!");
                println!("You are on version {} while the latest release is {}.", local_ver, remote_ver);
            } else {
                println!("You are already on the latest version ({}).", local_ver);
            }
            exit(0);
        }

        // ---- Custom ASCII management ----

        "--ascii-list" => {
            let names = list_custom_ascii();
            if names.is_empty() {
                println!("No custom ASCII arts saved yet.");
                println!("Add one with: vaminfo --ascii-add <name>");
            } else {
                println!("Saved custom ASCII arts:");
                for name in &names {
                    let active = load_ascii_config()
                        .map(|a| a == *name)
                        .unwrap_or(false);
                    if active {
                        println!("  {} (active)", name);
                    } else {
                        println!("  {}", name);
                    }
                }
            }
            exit(0);
        }

        "--ascii-add" => {
            if arg2.is_empty() {
                eprintln!("Usage: vaminfo --ascii-add <name>");
                eprintln!("Then pipe or type your ASCII art, end with Ctrl+D.");
                exit(1);
            }
            println!("Paste your ASCII art below, then press Ctrl+D when done:");
            let mut input = String::new();
            loop {
                let mut line = String::new();
                match io::stdin().read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => input.push_str(&line),
                    Err(_) => break,
                }
            }
            let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
            if lines.is_empty() {
                eprintln!("No input received.");
                exit(1);
            }
            if save_custom_ascii(arg2, &lines) {
                println!("Saved ASCII art '{}' ({} lines).", arg2, lines.len());
                println!("Use it with: vaminfo --ascii {}", arg2);
            } else {
                eprintln!("Failed to save ASCII art.");
                exit(1);
            }
            exit(0);
        }

        "--ascii-delete" => {
            if arg2.is_empty() {
                eprintln!("Usage: vaminfo --ascii-delete <name>");
                exit(1);
            }
            if delete_custom_ascii(arg2) {
                // Also clear active config if it was this one
                if load_ascii_config().map(|a| a == arg2).unwrap_or(false) {
                    let _ = save_ascii_config(None);
                }
                println!("Deleted ASCII art '{}'.", arg2);
            } else {
                eprintln!("Could not delete '{}'. Does it exist? Run 'vaminfo --ascii-list'.", arg2);
                exit(1);
            }
            exit(0);
        }

        "--ascii-reset" => {
            let _ = save_ascii_config(None);
            println!("Reset to default Vamora logo.");
            exit(0);
        }

        "--ascii" | "-a" => {
            if arg2.is_empty() {
                eprintln!("Usage: vaminfo --ascii <name>");
                eprintln!("Run 'vaminfo --ascii-list' to see saved arts.");
                exit(1);
            }
            if load_custom_ascii(arg2).is_none() {
                eprintln!("ASCII art '{}' not found. Run 'vaminfo --ascii-list'.", arg2);
                exit(1);
            }
            if save_ascii_config(Some(arg2)) {
                println!("Now using ASCII art '{}'. Run 'vaminfo' to see it.", arg2);
            } else {
                eprintln!("Failed to save config.");
                exit(1);
            }
            exit(0);
        }

        unknown if !unknown.is_empty() => {
            eprintln!("Unknown command '{}'\n", unknown);
            eprintln!("Vaminfo - A simple system fetch tool made just for VamoraOS");
            eprintln!();
            eprintln!("Usage: vaminfo [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  --help,         -h               Show this help message");
            eprintln!("  --version,      -v               Show version info");
            eprintln!("  --update,       -u               Update vaminfo to latest version");
            eprintln!("  --mini,         -m               Show a smaller version of vaminfo");
            eprintln!("  --color,        -c <color>       Set the ASCII art color");
            eprintln!("  --colors                         List all available colors");
            eprintln!();
            eprintln!("  --ascii,        -a <name>        Use a saved custom ASCII art");
            eprintln!("  --ascii-list                     List saved custom ASCII arts");
            eprintln!("  --ascii-add     <name>           Add custom ASCII from stdin");
            eprintln!("  --ascii-delete  <name>           Delete a saved custom ASCII");
            eprintln!("  --ascii-reset                    Reset to default Vamora logo");
            exit(1);
        }

        _ => {}
    }

    // ---- Normal fetch display ----
    ensure_config();
    let color = load_color();
    let active_ascii = load_ascii_config();

    let ascii_lines = if let Some(ref name) = active_ascii {
        load_custom_ascii(name).unwrap_or_else(|| default_ascii(&color))
    } else {
        default_ascii(&color)
    };

    // Apply color to custom ascii lines (wrap each line)
    let ascii_lines = if active_ascii.is_some() {
        let c = resolve_color(&color);
        ascii_lines
            .iter()
            .map(|l| format!("{}{}{}", c, l, RESET))
            .collect()
    } else {
        ascii_lines
    };

    let info = SysInfo::collect();
    let info_lines = info.info_lines();
    print_fetch(&ascii_lines, &info_lines);
}
