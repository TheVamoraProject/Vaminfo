use crate::config::{art_dir, VaminfoConfig};
use crate::renderer::parse_color;
use colored::Colorize;
use std::fs;
use std::io::{self, Write};

const COLORS: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright_black", "bright_red", "bright_green", "bright_yellow",
    "bright_blue", "bright_magenta", "bright_cyan", "bright_white",
];

pub fn run_wizard(mut cfg: VaminfoConfig) {
    loop {
        let c = parse_color(&cfg.ascii_color);
        println!("{}", "\n+==========================================+".color(c).bold());
        println!("{}", "|         Vaminfo  --  Configuration       |".color(c).bold());
        println!("{}", "+==========================================+\n".color(c).bold());

        println!("{}", "-- Main Menu --".color(parse_color(&cfg.title_color)).bold());
        println!("  1) ASCII art selection");
        println!("  2) Manage ASCII art files  (add / delete)");
        println!("  3) Colors");
        println!("  4) Toggle modules");
        println!("  5) Display options  (title / separator)");
        println!("  6) Mini mode: {}  |  ASCII in mini: {}",
            if cfg.mini_mode { "ON".green().bold() } else { "OFF".red() },
            if cfg.mini_mode_ascii { "ON".green().bold() } else { "OFF".red() },
        );
        println!("  7) Separator  [{}]", cfg.separator.bright_yellow());
        println!("  8) Preview");
        println!("  9) Save & exit");
        println!("  0) Exit without saving");
        println!();

        match prompt("Select option [0-9]: ").as_str() {
            "1" => wizard_ascii_select(&mut cfg),
            "2" => wizard_ascii_manage(&cfg),
            "3" => wizard_colors(&mut cfg),
            "4" => wizard_modules(&mut cfg),
            "5" => wizard_display(&mut cfg),
            "6" => {
                print_submenu_title(&cfg, "Mini Mode");
                println!("  a) Toggle mini mode  b) Toggle ASCII in mini");
                match prompt("  Choice [a/b]: ").to_lowercase().as_str() {
                    "a" => {
                        cfg.mini_mode = !cfg.mini_mode;
                        println!("Mini mode: {}", if cfg.mini_mode { "ON".green().bold() } else { "OFF".red() });
                    }
                    "b" => {
                        cfg.mini_mode_ascii = !cfg.mini_mode_ascii;
                        println!("ASCII in mini: {}", if cfg.mini_mode_ascii { "ON".green().bold() } else { "OFF".red() });
                    }
                    _ => println!("{}", "Invalid choice.".red()),
                }
            }
            "7" => {
                let sep = prompt("Enter separator character(s): ");
                if !sep.is_empty() {
                    cfg.separator = sep;
                }
            }
            "8" => {
                crate::renderer::render(&cfg);
            }
            "9" => {
                cfg.save();
                println!("{}", "\n Config saved!".green().bold());
                break;
            }
            "0" => {
                println!("Exiting without saving.");
                break;
            }
            _ => println!("{}", "Invalid option.".red()),
        }
        println!();
    }
}

// ── Display options ─────────────────────────────────────────────────────────

fn wizard_display(cfg: &mut VaminfoConfig) {
    loop {
        print_submenu_title(cfg, "Display Options");
        println!("  1) Show title (user@host)  [{}]", tl(cfg.show_title));
        println!("  2) Show separator line      [{}]", tl(cfg.show_separator));
        println!("  3) Back");

        match prompt("Select [1-3]: ").as_str() {
            "1" => {
                cfg.show_title = !cfg.show_title;
                println!("Title: {}", if cfg.show_title { "ON".green().bold() } else { "OFF".red() });
            }
            "2" => {
                cfg.show_separator = !cfg.show_separator;
                println!("Separator: {}", if cfg.show_separator { "ON".green().bold() } else { "OFF".red() });
            }
            "3" => break,
            _   => println!("{}", "Invalid option.".red()),
        }
    }
}

// ── ASCII art selection ─────────────────────────────────────────────────────

fn wizard_ascii_select(cfg: &mut VaminfoConfig) {
    print_submenu_title(cfg, "ASCII Art Selection");

    println!("  0) Built-in Vamora default");
    let files = list_art_files();
    for (i, f) in files.iter().enumerate() {
        let marker = if *f == cfg.ascii_file { " *" } else { "" };
        println!("  {}) {}{}", i + 1, f, marker);
    }

    if files.is_empty() {
        println!("  (no .vtxt files in art directory -- use option 2 to add some)");
    }

    let choice = prompt(&format!("Select [0-{}] (Enter to keep current): ", files.len()));
    if choice.is_empty() { return; }
    match choice.parse::<usize>() {
        Ok(0) => {
            cfg.ascii_file = "ascii1.vtxt".to_string();
            println!("Using built-in Vamora default (ascii1.vtxt).");
        }
        Ok(n) if n <= files.len() => {
            cfg.ascii_file = files[n - 1].clone();
            println!("Selected: {}", cfg.ascii_file);
        }
        _ => println!("{}", "Invalid selection.".red()),
    }
}

// ── ASCII art management ────────────────────────────────────────────────────

fn wizard_ascii_manage(cfg: &VaminfoConfig) {
    loop {
        print_submenu_title(cfg, "Manage ASCII Art Files");
        println!("  1) Add new ASCII art  (paste it in terminal)");
        println!("  2) Delete an ASCII art file");
        println!("  3) Back");

        match prompt("Select [1-3]: ").as_str() {
            "1" => wizard_ascii_add(),
            "2" => wizard_ascii_delete(cfg),
            "3" => break,
            _   => println!("{}", "Invalid option.".red()),
        }
    }
}

fn wizard_ascii_add() {
    let art_dir = art_dir();
    let _ = fs::create_dir_all(&art_dir);

    let mut idx = 1u32;
    let filename = loop {
        let name = format!("custom{}.vtxt", idx);
        if !art_dir.join(&name).exists() { break name; }
        idx += 1;
    };

    println!("{}", "\nPaste your ASCII art below.".bright_yellow());
    println!("{}", "Type END on its own line when done.\n".bright_yellow());

    let mut lines: Vec<String> = Vec::new();
    loop {
        let line = prompt_raw();
        if line.trim() == "END" { break; }
        lines.push(line);
    }

    if lines.is_empty() || lines.iter().all(|l| l.trim().is_empty()) {
        println!("{}", "Nothing entered -- cancelled.".red());
        return;
    }

    let content = lines.join("\n");
    let path = art_dir.join(&filename);
    match fs::write(&path, &content) {
        Ok(_) => println!("{} Saved as {}", "Saved!".green().bold(), filename),
        Err(e) => println!("{} {}", "Failed to save:".red(), e),
    }
}

fn wizard_ascii_delete(cfg: &VaminfoConfig) {
    let files = list_art_files();
    if files.is_empty() {
        println!("No .vtxt files to delete.");
        return;
    }

    println!();
    for (i, f) in files.iter().enumerate() {
        let marker = if *f == cfg.ascii_file { " * (active)" } else { "" };
        println!("  {}) {}{}", i + 1, f, marker);
    }
    println!("  0) Cancel");

    let choice = prompt(&format!("Delete which [0-{}]: ", files.len()));
    match choice.parse::<usize>() {
        Ok(0) => {}
        Ok(n) if n <= files.len() => {
            let name = &files[n - 1];
            if name == "ascii1.vtxt" {
                println!("{}", "Cannot delete the default Vamora art (ascii1.vtxt).".red());
                return;
            }
            let confirm = prompt(&format!("Delete '{}'? [y/N]: ", name));
            if confirm.to_lowercase() == "y" {
                let path = art_dir().join(name);
                match fs::remove_file(&path) {
                    Ok(_) => println!("{} Deleted {}", "Done!".green().bold(), name),
                    Err(e) => println!("{} {}", "Failed:".red(), e),
                }
            } else {
                println!("Cancelled.");
            }
        }
        _ => println!("{}", "Invalid selection.".red()),
    }
}

fn list_art_files() -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(art_dir()) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".vtxt") { files.push(name); }
        }
    }
    files.sort();
    files
}

// ── Colors ──────────────────────────────────────────────────────────────────

fn wizard_colors(cfg: &mut VaminfoConfig) {
    loop {
        print_submenu_title(cfg, "Colors");
        println!("  1) ASCII color     [{}]", cfg.ascii_color.bright_yellow());
        println!("  2) Title color     [{}]", cfg.title_color.bright_yellow());
        println!("  3) Key color       [{}]", cfg.key_color.bright_yellow());
        println!("  4) Value color     [{}]", cfg.value_color.bright_yellow());
        println!("  5) Back");

        match prompt("Select [1-5]: ").as_str() {
            "1" => cfg.ascii_color  = pick_color("ASCII color",  &cfg.ascii_color.clone()),
            "2" => cfg.title_color  = pick_color("Title color",  &cfg.title_color.clone()),
            "3" => cfg.key_color    = pick_color("Key color",    &cfg.key_color.clone()),
            "4" => cfg.value_color  = pick_color("Value color",  &cfg.value_color.clone()),
            "5" => break,
            _   => println!("{}", "Invalid option.".red()),
        }
    }
}

fn pick_color(label: &str, current: &str) -> String {
    println!("\nAvailable colors:");
    for (i, c) in COLORS.iter().enumerate() {
        println!("  {:2}) {}", i + 1, c);
    }
    let choice = prompt(&format!(
        "Select color for {} [1-{}] (Enter = keep '{}'): ",
        label, COLORS.len(), current
    ));
    if choice.is_empty() { return current.to_string(); }
    match choice.parse::<usize>() {
        Ok(n) if n >= 1 && n <= COLORS.len() => {
            let c = COLORS[n - 1].to_string();
            println!("Set to: {}", c);
            c
        }
        _ => {
            println!("{}", "Invalid selection, keeping current.".red());
            current.to_string()
        }
    }
}

// ── Modules ─────────────────────────────────────────────────────────────────

fn wizard_modules(cfg: &mut VaminfoConfig) {
    loop {
        print_submenu_title(cfg, "Module Toggle");
        let m = &cfg.modules;
        println!("  1)  Hostname            [{}]", tl(m.hostname));
        println!("  2)  OS                  [{}]", tl(m.os));
        println!("  3)  Kernel              [{}]", tl(m.kernel));
        println!("  4)  BIOS                [{}]", tl(m.bios));
        println!("  5)  CPU                 [{}]", tl(m.cpu));
        println!("  6)  GPU                 [{}]", tl(m.gpu));
        println!("  7)  RAM                 [{}]", tl(m.ram));
        println!("  8)  Disk                [{}]", tl(m.disk));
        println!("  9)  Uptime              [{}]", tl(m.uptime));
        println!("  10) Shell               [{}]", tl(m.shell));
        println!("  11) Terminal            [{}]", tl(m.terminal));
        println!("  12) Desktop / WM        [{}]", tl(m.desktop));
        println!("  13) Resolution          [{}]", tl(m.resolution));
        println!("  14) Theme               [{}]", tl(m.theme));
        println!("  15) Local IP            [{}]", tl(m.local_ip));
        println!("  16) Bluetooth           [{}]", tl(m.bluetooth));
        println!("  17) Battery             [{}]", tl(m.battery));
        println!("  18) Network             [{}]", tl(m.network));
        println!("  19) Media / Now Playing [{}]", tl(m.media));
        println!("  20) Color blocks big    [{}]", tl(m.color_blocks_big));
        println!("  21) Color blocks small  [{}]", tl(m.color_blocks_small));
        println!("  0)  Back");

        match prompt("Toggle module [0-21]: ").as_str() {
            "0"  => break,
            "1"  => cfg.modules.hostname          = !cfg.modules.hostname,
            "2"  => cfg.modules.os                = !cfg.modules.os,
            "3"  => cfg.modules.kernel            = !cfg.modules.kernel,
            "4"  => cfg.modules.bios              = !cfg.modules.bios,
            "5"  => cfg.modules.cpu               = !cfg.modules.cpu,
            "6"  => cfg.modules.gpu               = !cfg.modules.gpu,
            "7"  => cfg.modules.ram               = !cfg.modules.ram,
            "8"  => cfg.modules.disk              = !cfg.modules.disk,
            "9"  => cfg.modules.uptime            = !cfg.modules.uptime,
            "10" => cfg.modules.shell             = !cfg.modules.shell,
            "11" => cfg.modules.terminal          = !cfg.modules.terminal,
            "12" => cfg.modules.desktop           = !cfg.modules.desktop,
            "13" => cfg.modules.resolution        = !cfg.modules.resolution,
            "14" => cfg.modules.theme             = !cfg.modules.theme,
            "15" => cfg.modules.local_ip          = !cfg.modules.local_ip,
            "16" => cfg.modules.bluetooth         = !cfg.modules.bluetooth,
            "17" => cfg.modules.battery           = !cfg.modules.battery,
            "18" => cfg.modules.network           = !cfg.modules.network,
            "19" => cfg.modules.media             = !cfg.modules.media,
            "20" => cfg.modules.color_blocks_big  = !cfg.modules.color_blocks_big,
            "21" => cfg.modules.color_blocks_small= !cfg.modules.color_blocks_small,
            _    => println!("{}", "Invalid option.".red()),
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn print_submenu_title(cfg: &VaminfoConfig, label: &str) {
    let c = parse_color(&cfg.ascii_color);
    println!("\n{}", format!("-- {} --", label).color(c).bold());
}

fn tl(on: bool) -> colored::ColoredString {
    if on { "ON ".green().bold() } else { "OFF".red() }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    let _ = io::stdout().flush();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn prompt_raw() -> String {
    let _ = io::stdout().flush();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    if buf.ends_with('\n') { buf.pop(); }
    if buf.ends_with('\r') { buf.pop(); }
    buf
}
