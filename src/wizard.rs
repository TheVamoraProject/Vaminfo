use crate::config::{art_dir, GreetingEvent, VaminfoConfig};
use crate::modules::{effective_order, is_enabled, set_enabled, MODULE_KEYS};
use crate::renderer::parse_color;
use colored::Colorize;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::fs;
use std::io::{self, stdout, Write};

// ── Module display labels (key → human label) ─────────────────────────────────

const MODULE_LABELS: &[(&str, &str)] = &[
    ("os",                      "OS"),
    ("vamorasys_version",       "VamoraSys Version"),
    ("vmf_version",             "VMF Version"),
    ("vamora_version_codename", "Vamora Version + Codename"),
    ("hostname",                "Hostname"),
    ("kernel",                  "Kernel"),
    ("bios",                    "BIOS"),
    ("android_version",         "Android Version"),
    ("android_device",          "Android Device"),
    ("cpu",                     "CPU"),
    ("gpu",                     "GPU"),
    ("ram",                     "RAM"),
    ("disk",                    "Disk"),
    ("battery",                 "Battery"),
    ("bluetooth",               "Bluetooth"),
    ("uptime",                  "Uptime"),
    ("sys_age",                 "System Age"),
    ("shell",                   "Shell"),
    ("terminal",                "Terminal"),
    ("tty_type",                "TTY Type"),
    ("desktop",                 "Desktop / WM"),
    ("display_server",          "Display Server"),
    ("resolution",              "Resolution"),
    ("theme",                   "Theme"),
    ("fs_type",                 "Filesystem Type"),
    ("sudo_status",             "Sudo Privileges"),
    ("local_ip",                "Local IP"),
    ("public_ip",               "Public IP"),
    ("network",                 "Network I/O"),
    ("birthday_countdown",      "Birthday Countdown"),
    ("quotes",                  "Quotes"),
    ("jokes",                   "Linux Jokes"),
    ("color_blocks_big",        "Color Blocks (big)"),
    ("color_blocks_small",      "Color Blocks (small)"),
];

fn module_label(key: &str) -> String {
    MODULE_LABELS.iter()
        .find(|(k, _)| *k == key)
        .map(|(_, l)| l.to_string())
        .unwrap_or_else(|| key.to_string())
}

// ── Available colors ──────────────────────────────────────────────────────────

const COLORS: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright_black", "bright_red", "bright_green", "bright_yellow",
    "bright_blue", "bright_magenta", "bright_cyan", "bright_white",
];

// ── Screen drawing ────────────────────────────────────────────────────────────

fn draw(lines: &[String]) {
    let mut out = stdout();
    let _ = execute!(out, cursor::MoveTo(0, 0), Clear(ClearType::All));
    for line in lines {
        let _ = out.write_all(line.as_bytes());
        let _ = out.write_all(b"\r\n");
    }
    let _ = out.flush();
}

fn header(title: &str, hl: colored::Color) -> Vec<String> {
    let w = title.len() + 4;
    vec![
        String::new(),
        format!("  ╔{}╗", "═".repeat(w)).color(hl).bold().to_string(),
        format!("  ║  {}  ║", title).color(hl).bold().to_string(),
        format!("  ╚{}╝", "═".repeat(w)).color(hl).bold().to_string(),
        String::new(),
    ]
}

fn term_rows() -> usize {
    terminal::size().unwrap_or((80, 24)).1 as usize
}

// ── Key input ─────────────────────────────────────────────────────────────────

#[allow(dead_code)]
enum Key { Up, Down, Enter, Space, Back, MoveUp, MoveDown, Char(char) }

fn read_key() -> Key {
    loop {
        if let Ok(Event::Key(k)) = event::read() {
            return match (k.code, k.modifiers) {
                // Shift+Up / Shift+Down → reorder
                (KeyCode::Up,   m) if m.contains(KeyModifiers::SHIFT) => Key::MoveUp,
                (KeyCode::Down, m) if m.contains(KeyModifiers::SHIFT) => Key::MoveDown,
                (KeyCode::Up,   _) | (KeyCode::Char('k'), _)          => Key::Up,
                (KeyCode::Down, _) | (KeyCode::Char('j'), _)          => Key::Down,
                (KeyCode::Enter, _)     => Key::Enter,
                (KeyCode::Char(' '), _) => Key::Space,
                (KeyCode::Esc,  _)
                | (KeyCode::Char('q'), _)
                | (KeyCode::Char('Q'), _) => Key::Back,
                (KeyCode::Char('u'), _) | (KeyCode::Char('U'), _) => Key::MoveUp,
                (KeyCode::Char('d'), _) | (KeyCode::Char('D'), _) => Key::MoveDown,
                (KeyCode::Char(c),  _)  => Key::Char(c),
                _                       => continue,
            };
        }
    }
}

// ── Text prompt (exits raw mode temporarily) ──────────────────────────────────

fn prompt(msg: &str) -> String {
    let _ = terminal::disable_raw_mode();
    let _ = execute!(stdout(), cursor::Show);
    print!("\r\n{}", msg);
    let _ = io::stdout().flush();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    let _ = terminal::enable_raw_mode();
    let _ = execute!(stdout(), cursor::Hide);
    buf.trim().to_string()
}

// ── Generic arrow-select ──────────────────────────────────────────────────────
// Returns Some(index) on Enter, None on Back/q/Esc.

fn arrow_select(
    title: &str,
    items: &[String],
    mut sel: usize,
    hl: colored::Color,
    hint: &str,
) -> Option<usize> {
    sel = sel.min(items.len().saturating_sub(1));
    let mut top = 0usize;

    loop {
        let visible = term_rows().saturating_sub(9).max(3);
        if sel < top               { top = sel; }
        if sel >= top + visible    { top = sel + 1 - visible; }

        let mut lines = header(title, hl);
        lines.push(format!("  {}", hint.dimmed()));
        lines.push(String::new());

        for (i, item) in items.iter().enumerate().skip(top).take(visible) {
            if i == sel {
                lines.push(
                    format!("  ▶  {}", item).color(hl).bold().to_string()
                );
            } else {
                lines.push(format!("     {}", item));
            }
        }

        if items.len() > visible {
            lines.push(String::new());
            lines.push(
                format!("  {}/{}", sel + 1, items.len()).dimmed().to_string()
            );
        }

        draw(&lines);

        match read_key() {
            Key::Up   => { if sel > 0 { sel -= 1; } }
            Key::Down => { if sel + 1 < items.len() { sel += 1; } }
            Key::Enter | Key::Space => return Some(sel),
            Key::Back => return None,
            _ => {}
        }
    }
}

// ── Main entry point ──────────────────────────────────────────────────────────

pub fn run_wizard(mut cfg: VaminfoConfig) {
    let _ = terminal::enable_raw_mode();
    let _ = execute!(stdout(), cursor::Hide);

    main_menu(&mut cfg);

    let _ = terminal::disable_raw_mode();
    let _ = execute!(
        stdout(),
        cursor::Show,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    );
}

fn main_menu(cfg: &mut VaminfoConfig) {
    let items: Vec<String> = vec![
        "ASCII Art".into(),
        "Colors".into(),
        "Modules & Order".into(),
        "Display Options".into(),
        "Greetings & Events".into(),
        "Preview".into(),
        "Save & Exit".into(),
        "Exit without Saving".into(),
    ];
    let mut sel = 0;

    loop {
        let hl = parse_color(&cfg.ascii_color);
        let choice = arrow_select(
            "Vaminfo Config Wizard",
            &items,
            sel,
            hl,
            "↑↓  Navigate     Enter  Select     Q  Exit without saving",
        );
        match choice {
            None    => return,  // q/Esc = exit without saving
            Some(i) => {
                sel = i;
                match i {
                    0 => menu_ascii(cfg),
                    1 => menu_colors(cfg),
                    2 => menu_modules(cfg),
                    3 => menu_display(cfg),
                    4 => menu_greetings(cfg),
                    5 => run_preview(cfg),
                    6 => { cfg.save(); return; }
                    _ => return,
                }
            }
        }
    }
}

// ── Preview ───────────────────────────────────────────────────────────────────

fn run_preview(cfg: &VaminfoConfig) {
    let _ = terminal::disable_raw_mode();
    let _ = execute!(stdout(), cursor::Show, Clear(ClearType::All), cursor::MoveTo(0, 0));
    crate::renderer::render(cfg);
    println!("\n  Press any key to return to wizard...");
    let _ = io::stdout().flush();
    // re-enter raw mode then wait for one key
    let _ = terminal::enable_raw_mode();
    let _ = execute!(stdout(), cursor::Hide);
    let _ = read_key();
}

// ── ASCII art selection ───────────────────────────────────────────────────────

fn menu_ascii(cfg: &mut VaminfoConfig) {
    let hl = parse_color(&cfg.ascii_color);
    loop {
        let files = list_art_files();
        // Prepend built-in sentinel
        let mut items: Vec<String> = vec!["[built-in] ascii1.vtxt  (Vamora default)".into()];
        for f in &files {
            let marker = if *f == cfg.ascii_file { " *" } else { "" };
            items.push(format!("{}{}", f, marker));
        }

        let current_idx = files.iter().position(|f| *f == cfg.ascii_file).map(|i| i + 1).unwrap_or(0);

        let choice = arrow_select(
            "ASCII Art",
            &items,
            current_idx,
            hl,
            "↑↓  Navigate     Enter  Select     Q  Back",
        );

        match choice {
            None    => return,
            Some(0) => { cfg.ascii_file = "ascii1.vtxt".into(); }
            Some(n) if n <= files.len() => {
                cfg.ascii_file = files[n - 1].clone();
            }
            _ => {}
        }

        // After selection ask if they want to add/delete or just go back
        let mgmt: Vec<String> = vec![
            "Keep this selection".into(),
            "Add new ASCII art (paste)".into(),
            "Delete an ASCII art file".into(),
            "Back".into(),
        ];
        match arrow_select("ASCII Art Management", &mgmt, 0, hl, "↑↓  Navigate     Enter  Select") {
            Some(0) | None => return,
            Some(1) => ascii_add(),
            Some(2) => ascii_delete(cfg, hl),
            _       => return,
        }
        // loop to let user pick again after management
    }
}

fn list_art_files() -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(art_dir()) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".vtxt") { files.push(name); }
        }
    }
    files.sort();
    files
}

fn ascii_add() {
    let art_dir = art_dir();
    let _ = fs::create_dir_all(&art_dir);
    let mut idx = 1u32;
    let filename = loop {
        let name = format!("custom{}.vtxt", idx);
        if !art_dir.join(&name).exists() { break name; }
        idx += 1;
    };

    // Need cooked mode for multi-line paste
    let _ = terminal::disable_raw_mode();
    let _ = execute!(stdout(), cursor::Show, Clear(ClearType::All), cursor::MoveTo(0, 0));
    println!("Paste your ASCII art below.");
    println!("Type END on its own line when done.\n");
    let mut lines: Vec<String> = Vec::new();
    loop {
        let _ = io::stdout().flush();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        let trimmed = buf.trim_end_matches('\n').trim_end_matches('\r').to_string();
        if trimmed == "END" { break; }
        lines.push(trimmed);
    }
    if lines.is_empty() || lines.iter().all(|l| l.trim().is_empty()) {
        println!("Nothing entered — cancelled.");
    } else {
        match fs::write(art_dir.join(&filename), lines.join("\n")) {
            Ok(_)  => println!("Saved as {}", filename),
            Err(e) => println!("Failed: {}", e),
        }
    }
    println!("\nPress Enter to continue...");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut String::new()).ok();
    let _ = terminal::enable_raw_mode();
    let _ = execute!(stdout(), cursor::Hide);
}

fn ascii_delete(cfg: &VaminfoConfig, hl: colored::Color) {
    let files = list_art_files();
    if files.is_empty() { return; }
    let items: Vec<String> = files.iter().map(|f| {
        if *f == cfg.ascii_file { format!("{} (active)", f) } else { f.clone() }
    }).collect();
    if let Some(n) = arrow_select("Delete ASCII Art", &items, 0, hl, "↑↓  Navigate     Enter  Delete     Q  Cancel") {
        let name = &files[n];
        if name == "ascii1.vtxt" {
            // can't delete default — just return
            return;
        }
        match fs::remove_file(art_dir().join(name)) {
            Ok(_)  => {}
            Err(_) => {}
        }
    }
}

// ── Colors ────────────────────────────────────────────────────────────────────

fn menu_colors(cfg: &mut VaminfoConfig) {
    loop {
        let hl = parse_color(&cfg.ascii_color);
        let items = vec![
            format!("ASCII color    [ {} ]", cfg.ascii_color.bright_yellow()),
            format!("Title color    [ {} ]", cfg.title_color.bright_yellow()),
            format!("Key color      [ {} ]", cfg.key_color.bright_yellow()),
            format!("Value color    [ {} ]", cfg.value_color.bright_yellow()),
            "Back".into(),
        ];
        match arrow_select("Colors", &items, 0, hl, "↑↓  Navigate     Enter  Select     Q  Back") {
            None | Some(4) => return,
            Some(0) => cfg.ascii_color  = pick_color("ASCII color",  &cfg.ascii_color.clone(),  parse_color(&cfg.ascii_color)),
            Some(1) => cfg.title_color  = pick_color("Title color",  &cfg.title_color.clone(),  hl),
            Some(2) => cfg.key_color    = pick_color("Key color",    &cfg.key_color.clone(),    hl),
            Some(3) => cfg.value_color  = pick_color("Value color",  &cfg.value_color.clone(),  hl),
            _ => {}
        }
    }
}

fn pick_color(title: &str, current: &str, hl: colored::Color) -> String {
    // Colored block on the left, plain text label (no blinding full-row color)
    let items: Vec<String> = COLORS.iter().map(|&c| {
        let block  = "██".color(parse_color(c)).to_string();
        let marker = if c == current { "  ◄ current" } else { "" };
        format!("{} {}{}", block, c, marker)
    }).collect();

    let current_idx = COLORS.iter().position(|&c| c == current).unwrap_or(0);

    match arrow_select(title, &items, current_idx, hl, "↑↓  Navigate     Enter  Select     Q  Keep current") {
        Some(n) => COLORS[n].to_string(),
        None    => current.to_string(),
    }
}

// ── Display options ───────────────────────────────────────────────────────────

fn menu_display(cfg: &mut VaminfoConfig) {
    loop {
        let hl = parse_color(&cfg.ascii_color);
        let on  = |b: bool| if b { "ON".green().bold().to_string() } else { "OFF".red().to_string() };
        let items = vec![
            format!("Show title (user@host)   [ {} ]", on(cfg.show_title)),
            format!("Show separator line      [ {} ]", on(cfg.show_separator)),
            format!("Mini mode                [ {} ]", on(cfg.mini_mode)),
            format!("Separator char           [ {} ]", cfg.separator.bright_yellow()),
            "Back".into(),
        ];
        match arrow_select("Display Options", &items, 0, hl, "↑↓  Navigate     Enter  Toggle/Set     Q  Back") {
            None | Some(4) => return,
            Some(0) => cfg.show_title    = !cfg.show_title,
            Some(1) => cfg.show_separator = !cfg.show_separator,
            Some(2) => cfg.mini_mode     = !cfg.mini_mode,
            Some(3) => {
                let s = prompt("Separator character(s): ");
                if !s.is_empty() { cfg.separator = s; }
            }
            _ => {}
        }
    }
}

// ── Greetings ─────────────────────────────────────────────────────────────────

fn menu_greetings(cfg: &mut VaminfoConfig) {
    loop {
        let hl  = parse_color(&cfg.ascii_color);
        let on  = |b: bool| if b { "ON".green().bold().to_string() } else { "OFF".red().to_string() };
        let bday = if cfg.greetings.birthday.is_empty() {
            "not set".dimmed().to_string()
        } else {
            cfg.greetings.birthday.bright_yellow().to_string()
        };
        let items = vec![
            format!("Greetings enabled     [ {} ]", on(cfg.greetings.enabled)),
            format!("Birthday (MM-DD)      [ {} ]", bday),
            format!("Events                [ {} ]", cfg.greetings.events.len().to_string().bright_yellow()),
            "Add event".into(),
            "List / delete events".into(),
            "Back".into(),
        ];
        match arrow_select("Greetings & Events", &items, 0, hl, "↑↓  Navigate     Enter  Select/Toggle     Q  Back") {
            None | Some(5) => return,
            Some(0) => cfg.greetings.enabled = !cfg.greetings.enabled,
            Some(1) => {
                let v = prompt("Birthday MM-DD (e.g. 07-15), or Enter to clear: ");
                cfg.greetings.birthday = v;
            }
            Some(2) | Some(4) => greetings_list(cfg, hl),
            Some(3) => greetings_add(cfg),
            _ => {}
        }
    }
}

fn greetings_add(cfg: &mut VaminfoConfig) {
    let date    = prompt("Date (MM-DD): ");
    if date.trim().is_empty() { return; }
    let name    = prompt("Event name: ");
    let message = prompt("Greeting message: ");
    cfg.greetings.events.push(GreetingEvent {
        date:    date.trim().to_string(),
        name:    name.trim().to_string(),
        message: message.trim().to_string(),
    });
}

fn greetings_list(cfg: &mut VaminfoConfig, hl: colored::Color) {
    loop {
        if cfg.greetings.events.is_empty() { return; }
        let items: Vec<String> = cfg.greetings.events.iter().map(|ev| {
            format!("{}  {}  \"{}\"", ev.date.bright_yellow(), ev.name, ev.message)
        }).chain(std::iter::once("Back".to_string())).collect();

        match arrow_select("Events (Enter to delete)", &items, 0, hl, "↑↓  Navigate     Enter  Delete     Q  Back") {
            None => return,
            Some(n) if n < cfg.greetings.events.len() => {
                cfg.greetings.events.remove(n);
            }
            _ => return,
        }
    }
}

// ── Modules & reordering ──────────────────────────────────────────────────────

struct ModRow {
    key:     String,
    label:   String,
    enabled: bool,
}

fn menu_modules(cfg: &mut VaminfoConfig) {
    let hl = parse_color(&cfg.ascii_color);

    // Build ordered list from config
    let mut rows: Vec<ModRow> = effective_order(cfg)
        .into_iter()
        .filter(|k| MODULE_KEYS.contains(&k.as_str()))
        .map(|k| ModRow {
            label:   module_label(&k),
            enabled: is_enabled(cfg, &k),
            key:     k,
        })
        .collect();

    let mut sel   = 0usize;
    let mut top   = 0usize;

    loop {
        let visible = term_rows().saturating_sub(10).max(3);
        if sel < top             { top = sel; }
        if sel >= top + visible  { top = sel + 1 - visible; }

        let mut lines = header("Modules & Order", hl);
        lines.push(
            "  ↑↓/jk Navigate   Space Toggle   U/D or Shift+↑↓ Reorder   Q Save & Back"
                .dimmed().to_string()
        );
        lines.push(String::new());

        for (i, row) in rows.iter().enumerate().skip(top).take(visible) {
            if i == sel {
                // Selected: entire row in hl color
                let state = if row.enabled { "[ON ]" } else { "[OFF]" };
                lines.push(
                    format!("  ▶  {} {}", state, row.label)
                        .color(hl).bold().to_string()
                );
            } else {
                let state = if row.enabled {
                    "[ON ]".green().bold().to_string()
                } else {
                    "[OFF]".red().dimmed().to_string()
                };
                lines.push(format!("     {} {}", state, row.label));
            }
        }

        if rows.len() > visible {
            lines.push(String::new());
            lines.push(format!("  {}/{}", sel + 1, rows.len()).dimmed().to_string());
        }

        draw(&lines);

        match read_key() {
            Key::Up   => { if sel > 0 { sel -= 1; } }
            Key::Down => { if sel + 1 < rows.len() { sel += 1; } }
            Key::Space | Key::Enter => { rows[sel].enabled = !rows[sel].enabled; }
            Key::MoveUp => {
                if sel > 0 {
                    rows.swap(sel, sel - 1);
                    sel -= 1;
                }
            }
            Key::MoveDown => {
                if sel + 1 < rows.len() {
                    rows.swap(sel, sel + 1);
                    sel += 1;
                }
            }
            Key::Back => break,
            _ => {}
        }
    }

    // Write back toggle states and order
    for row in &rows {
        set_enabled(cfg, &row.key, row.enabled);
    }
    cfg.module_order = rows.into_iter().map(|r| r.key).collect();
}
