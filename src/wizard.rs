use crate::config::{art_dir, GreetingEvent, VaminfoConfig};
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
        println!("{}", "|      Vaminfo Configuration Wizard        |".color(c).bold());
        println!("{}", "+==========================================+\n".color(c).bold());

        println!("{}", "-- Main Menu --".color(parse_color(&cfg.title_color)).bold());
        println!("  1) ASCII art selection");
        println!("  2) Manage ASCII art files");
        println!("  3) Colors");
        println!("  4) Toggle modules");
        println!("  5) Display options    (title / separator)");
        println!("  6) Mini mode: {}",
            if cfg.mini_mode { "ON".green().bold() } else { "OFF".red() },
        );
        println!("  7) Separator  [{}]", cfg.separator.bright_yellow());
        println!("  8) Greetings & events");
        println!("  9) Preview");
        println!("  s) Save & exit");
        println!("  q) Exit without saving");
        println!();

        match prompt("Select option: ").to_lowercase().as_str() {
            "1" => wizard_ascii_select(&mut cfg),
            "2" => wizard_ascii_manage(&cfg),
            "3" => wizard_colors(&mut cfg),
            "4" => wizard_modules(&mut cfg),
            "5" => wizard_display(&mut cfg),
            "6" => {
                cfg.mini_mode = !cfg.mini_mode;
                println!("Mini mode: {}", if cfg.mini_mode { "ON".green().bold() } else { "OFF".red() });
            }
            "7" => {
                let sep = prompt("Enter separator character(s): ");
                if !sep.is_empty() { cfg.separator = sep; }
            }
            "8" => wizard_greetings(&mut cfg),
            "9" => { crate::renderer::render(&cfg); }
            "s" => {
                cfg.save();
                println!("{}", "\n Config saved!".green().bold());
                break;
            }
            "q" => {
                println!("Exiting without saving.");
                break;
            }
            _ => println!("{}", "Invalid option.".red()),
        }
        println!();
    }
}

// ── Display options ──────────────────────────────────────────────────────────

fn wizard_display(cfg: &mut VaminfoConfig) {
    loop {
        print_submenu_title(cfg, "Display Options");
        println!("  1) Show title (user@host)  [{}]", tl(cfg.show_title));
        println!("  2) Show separator line      [{}]", tl(cfg.show_separator));
        println!("  3) Back");
        match prompt("Select [1-3]: ").as_str() {
            "1" => cfg.show_title     = !cfg.show_title,
            "2" => cfg.show_separator = !cfg.show_separator,
            "3" => break,
            _   => println!("{}", "Invalid option.".red()),
        }
    }
}

// ── Greetings / Events ───────────────────────────────────────────────────────

fn wizard_greetings(cfg: &mut VaminfoConfig) {
    loop {
        print_submenu_title(cfg, "Greetings & Events");
        println!("  Greetings enabled  [{}]", tl(cfg.greetings.enabled));
        println!("  Birthday (MM-DD)   [{}]",
            if cfg.greetings.birthday.is_empty() { "not set".dimmed().to_string() }
            else { cfg.greetings.birthday.bright_yellow().to_string() }
        );
        println!("  Events             [{}]", cfg.greetings.events.len().to_string().bright_yellow());
        println!();
        println!("  1) Toggle greetings on/off");
        println!("  2) Set birthday (MM-DD)");
        println!("  3) Add event");
        println!("  4) List / delete events");
        println!("  5) Back");

        match prompt("Select [1-5]: ").as_str() {
            "1" => {
                cfg.greetings.enabled = !cfg.greetings.enabled;
                println!("Greetings: {}", if cfg.greetings.enabled { "ON".green().bold() } else { "OFF".red() });
            }
            "2" => {
                let v = prompt("Birthday MM-DD (e.g. 07-15), or Enter to clear: ");
                cfg.greetings.birthday = v.trim().to_string();
                if cfg.greetings.birthday.is_empty() {
                    println!("Birthday cleared.");
                } else {
                    println!("Birthday set to {}", cfg.greetings.birthday.bright_yellow());
                }
            }
            "3" => {
                print_submenu_title(cfg, "Add Event");
                let date = prompt("Date (MM-DD): ");
                if date.trim().is_empty() { println!("Cancelled."); continue; }
                let name = prompt("Event name: ");
                let message = prompt("Greeting message: ");
                cfg.greetings.events.push(GreetingEvent {
                    date:    date.trim().to_string(),
                    name:    name.trim().to_string(),
                    message: message.trim().to_string(),
                });
                println!("{}", "Event added!".green().bold());
            }
            "4" => {
                if cfg.greetings.events.is_empty() {
                    println!("No events set.");
                    continue;
                }
                print_submenu_title(cfg, "Events");
                for (i, ev) in cfg.greetings.events.iter().enumerate() {
                    println!("  {}) {}  {}  \"{}\"", i + 1, ev.date.bright_yellow(), ev.name, ev.message);
                }
                let del = prompt("Delete event number (or Enter to cancel): ");
                if let Ok(n) = del.trim().parse::<usize>() {
                    if n >= 1 && n <= cfg.greetings.events.len() {
                        cfg.greetings.events.remove(n - 1);
                        println!("{}", "Event deleted.".green().bold());
                    } else {
                        println!("{}", "Invalid number.".red());
                    }
                }
            }
            "5" => break,
            _   => println!("{}", "Invalid option.".red()),
        }
    }
}

// ── ASCII art selection ──────────────────────────────────────────────────────

fn wizard_ascii_select(cfg: &mut VaminfoConfig) {
    print_submenu_title(cfg, "ASCII Art Selection");
    println!("  0) Built-in Vamora default");
    let files = list_art_files();

    if files.is_empty() {
        println!("  (no .vtxt files found -- use option 2 to add some)");
    } else if files.len() > 10 {
        // 2-column layout
        let mid = (files.len() + 1) / 2;
        for i in 0..mid {
            let ml = if files[i] == cfg.ascii_file { " *" } else { "" };
            let left = format!("{:3}) {}{}", i + 1, &files[i], ml);
            if let Some(rf) = files.get(mid + i) {
                let mr = if *rf == cfg.ascii_file { " *" } else { "" };
                let right = format!("{:3}) {}{}", mid + i + 1, rf, mr);
                println!("  {:<36}  {}", left, right);
            } else {
                println!("  {}", left);
            }
        }
    } else {
        for (i, f) in files.iter().enumerate() {
            let marker = if *f == cfg.ascii_file { " *" } else { "" };
            println!("  {}) {}{}", i + 1, f, marker);
        }
    }

    let choice = prompt(&format!("Select [0-{}] (Enter to keep current): ", files.len()));
    if choice.is_empty() { return; }
    match choice.parse::<usize>() {
        Ok(0) => {
            cfg.ascii_file = "ascii1.vtxt".to_string();
            println!("Using built-in Vamora default.");
        }
        Ok(n) if n <= files.len() => {
            cfg.ascii_file = files[n - 1].clone();
            println!("Selected: {}", cfg.ascii_file);
        }
        _ => println!("{}", "Invalid selection.".red()),
    }
}

// ── ASCII art management ─────────────────────────────────────────────────────

fn wizard_ascii_manage(cfg: &VaminfoConfig) {
    loop {
        print_submenu_title(cfg, "Manage ASCII Art Files");
        println!("  1) Add new ASCII art");
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
    let path = art_dir.join(&filename);
    match fs::write(&path, lines.join("\n")) {
        Ok(_) => println!("{} Saved as {}", "Saved!".green().bold(), filename),
        Err(e) => println!("{} {}", "Failed to save:".red(), e),
    }
}

fn wizard_ascii_delete(cfg: &VaminfoConfig) {
    let files = list_art_files();
    if files.is_empty() { println!("No .vtxt files to delete."); return; }
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
                match fs::remove_file(art_dir().join(name)) {
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

// ── Colors ───────────────────────────────────────────────────────────────────

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
        "Select {} [1-{}] (Enter = keep '{}'): ", label, COLORS.len(), current
    ));
    if choice.is_empty() { return current.to_string(); }
    match choice.parse::<usize>() {
        Ok(n) if n >= 1 && n <= COLORS.len() => {
            let c = COLORS[n - 1].to_string();
            println!("Set to: {}", c);
            c
        }
        _ => { println!("{}", "Invalid, keeping current.".red()); current.to_string() }
    }
}

// ── Module toggles ────────────────────────────────────────────────────────────

fn wizard_modules(cfg: &mut VaminfoConfig) {
    loop {
        print_submenu_title(cfg, "Module Toggle");
        let m = &cfg.modules;
        println!("{}", "  -- System Identity --".dimmed());
        println!("   1) Hostname          [{}]", tl(m.hostname));
        println!("   2) OS               [{}]", tl(m.os));
        println!("   3) Kernel           [{}]", tl(m.kernel));
        println!("   4) BIOS             [{}]", tl(m.bios));
        println!("{}", "  -- Hardware --".dimmed());
        println!("   5) CPU              [{}]", tl(m.cpu));
        println!("   6) GPU              [{}]", tl(m.gpu));
        println!("   7) RAM              [{}]", tl(m.ram));
        println!("   8) Disk             [{}]", tl(m.disk));
        println!("   9) Battery          [{}]", tl(m.battery));
        println!("  10) Bluetooth        [{}]", tl(m.bluetooth));
        println!("{}", "  -- Time --".dimmed());
        println!("  11) Uptime           [{}]", tl(m.uptime));
        println!("  12) System Age       [{}]", tl(m.sys_age));
        println!("{}", "  -- Environment --".dimmed());
        println!("  13) Shell            [{}]", tl(m.shell));
        println!("  14) Terminal         [{}]", tl(m.terminal));
        println!("  15) TTY type         [{}]", tl(m.tty_type));
        println!("  16) Desktop / WM     [{}]", tl(m.desktop));
        println!("  17) Display server   [{}]", tl(m.display_server));
        println!("  18) Resolution       [{}]", tl(m.resolution));
        println!("  19) Theme            [{}]", tl(m.theme));
        println!("  20) Filesystem type  [{}]", tl(m.fs_type));
        println!("  21) Sudo privileges  [{}]", tl(m.sudo_status));
        println!("{}", "  -- Network --".dimmed());
        println!("  22) Local IP         [{}]", tl(m.local_ip));
        println!("  23) Public IP        [{}]  (makes a network request)", tl(m.public_ip));
        println!("  24) Network I/O      [{}]", tl(m.network));
        println!("{}", "  -- Android --".dimmed());
        println!("  25) Android version  [{}]  (Android/Termux only)", tl(m.android_version));
        println!("  26) Android device   [{}]  (Android/Termux only)", tl(m.android_device));
        println!("{}", "  -- VamoraOS --".dimmed());
        println!("  27) VamoraSys version   [{}]  (only if /etc/VamoraSys/ exists)", tl(m.vamorasys_version));
        println!("  28) VMF version         [{}]  (only if /etc/VamoraSys/ exists)", tl(m.vmf_version));
        println!("  29) Vamora ver+codename [{}]  (only if /etc/VamoraSys/ exists)", tl(m.vamora_version_codename));
        println!("{}", "  -- Fun / Optional --".dimmed());
        println!("  30) Birthday cntdwn  [{}]  (requires birthday in greetings)", tl(m.birthday_countdown));
        println!("  31) Quotes           [{}]", tl(m.quotes));
        println!("  32) Linux Jokes      [{}]", tl(m.jokes));
        println!("{}", "  -- Color Blocks --".dimmed());
        println!("  33) Color blocks big [{}]", tl(m.color_blocks_big));
        println!("  34) Color blocks sml [{}]", tl(m.color_blocks_small));
        println!("   0) Back");

        let input = prompt("Toggle [0-34]: ");
        match input.as_str() {
            "0"  => break,
            "1"  => cfg.modules.hostname           = !cfg.modules.hostname,
            "2"  => cfg.modules.os                 = !cfg.modules.os,
            "3"  => cfg.modules.kernel             = !cfg.modules.kernel,
            "4"  => cfg.modules.bios               = !cfg.modules.bios,
            "5"  => cfg.modules.cpu                = !cfg.modules.cpu,
            "6"  => cfg.modules.gpu                = !cfg.modules.gpu,
            "7"  => cfg.modules.ram                = !cfg.modules.ram,
            "8"  => cfg.modules.disk               = !cfg.modules.disk,
            "9"  => cfg.modules.battery            = !cfg.modules.battery,
            "10" => cfg.modules.bluetooth          = !cfg.modules.bluetooth,
            "11" => cfg.modules.uptime             = !cfg.modules.uptime,
            "12" => cfg.modules.sys_age            = !cfg.modules.sys_age,
            "13" => cfg.modules.shell              = !cfg.modules.shell,
            "14" => cfg.modules.terminal           = !cfg.modules.terminal,
            "15" => cfg.modules.tty_type           = !cfg.modules.tty_type,
            "16" => cfg.modules.desktop            = !cfg.modules.desktop,
            "17" => cfg.modules.display_server     = !cfg.modules.display_server,
            "18" => cfg.modules.resolution         = !cfg.modules.resolution,
            "19" => cfg.modules.theme              = !cfg.modules.theme,
            "20" => cfg.modules.fs_type            = !cfg.modules.fs_type,
            "21" => cfg.modules.sudo_status        = !cfg.modules.sudo_status,
            "22" => cfg.modules.local_ip           = !cfg.modules.local_ip,
            "23" => cfg.modules.public_ip          = !cfg.modules.public_ip,
            "24" => cfg.modules.network            = !cfg.modules.network,
            "25" => cfg.modules.android_version    = !cfg.modules.android_version,
            "26" => cfg.modules.android_device     = !cfg.modules.android_device,
            "27" => cfg.modules.vamorasys_version       = !cfg.modules.vamorasys_version,
            "28" => cfg.modules.vmf_version             = !cfg.modules.vmf_version,
            "29" => cfg.modules.vamora_version_codename = !cfg.modules.vamora_version_codename,
            "30" => cfg.modules.birthday_countdown      = !cfg.modules.birthday_countdown,
            "31" => cfg.modules.quotes                  = !cfg.modules.quotes,
            "32" => cfg.modules.jokes                   = !cfg.modules.jokes,
            "33" => cfg.modules.color_blocks_big        = !cfg.modules.color_blocks_big,
            "34" => cfg.modules.color_blocks_small      = !cfg.modules.color_blocks_small,
            _    => println!("{}", "Invalid option.".red()),
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

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
