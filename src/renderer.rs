use crate::ascii::{ascii_width, load_ascii, truncate_to_visible, visible_len};
use crate::config::VaminfoConfig;
use crate::layout::{decide_layout, detect_terminal_size, LayoutMode};
use crate::modules::{build_mini_modules, build_modules};
use colored::{Color, Colorize};
use sysinfo::System;

// ── Public entry points ───────────────────────────────────────────────────────

pub fn render(cfg: &VaminfoConfig) {
    if cfg.mini_mode {
        render_mini(cfg);
    } else {
        render_full(cfg);
    }
}

pub fn render_full(cfg: &VaminfoConfig) {
    let sys = System::new_all();
    let show_ascii = !cfg.mini_mode;

    let ascii_lines = if show_ascii { load_ascii(cfg) } else { vec![] };
    let aw   = if show_ascii { ascii_width(&ascii_lines) } else { 0 };
    let term = detect_terminal_size();
    let layout = if show_ascii { decide_layout(&term, aw) } else { LayoutMode::Stacked };

    let modules = build_modules(cfg);
    let info_lines = build_info_lines(cfg, &sys, &modules, false);

    match layout {
        LayoutMode::Split => render_split(&ascii_lines, &info_lines, cfg, aw, term.cols as usize),
        LayoutMode::Stacked => {
            if show_ascii {
                render_stacked(&ascii_lines, &info_lines, cfg);
            } else {
                render_info_only(&info_lines);
            }
        }
    }
}

// ── Gorgeous mini mode ────────────────────────────────────────────────────────

pub fn render_mini(cfg: &VaminfoConfig) {
    let sys = System::new_all();
    let modules = build_mini_modules();
    let kc = parse_color(&cfg.key_color);
    let vc = parse_color(&cfg.value_color);
    let tc = parse_color(&cfg.title_color);
    let ac = parse_color(&cfg.ascii_color);

    // Collect rows
    let mut rows: Vec<(String, String)> = Vec::new();
    for module in &modules {
        if let Some(val) = module.collect(&sys, cfg) {
            rows.push((module.name().to_string(), val));
        }
    }

    // Determine box width
    let title = user_host();
    let inner_width = {
        let max_row = rows.iter()
            .map(|(k, v)| k.len() + 3 + v.len()) // "Key : Value"
            .max()
            .unwrap_or(0);
        max_row.max(title.len() + 2).max(40).min(70)
    };
    let _box_w = inner_width + 2; // borders

    // Top border: ╭─ user@host ─...─╮
    let title_section = format!(" {} ", title);
    let dash_total = inner_width.saturating_sub(title_section.len());
    let dash_r = dash_total;
    let top = format!(
        "{}{}{}{}",
        "╭".color(ac),
        title_section.color(tc).bold(),
        "─".repeat(dash_r).color(ac),
        "╮".color(ac),
    );
    println!("{}", top);

    // Rows
    const KEY_W: usize = 8;
    const COLON_W: usize = 3; // " : "
    const INDENT: usize = 2;  // leading "  "
    let max_val_w = inner_width.saturating_sub(INDENT + KEY_W + COLON_W + 1);

    for (key, val) in &rows {
        // RAM gets a progress bar
        let base_val = if key == "RAM" {
            format!("{}  {}", val, ram_bar(val, 8))
        } else {
            val.clone()
        };

        // Truncate to box width (plain text, before coloring)
        let display_val = if base_val.len() > max_val_w {
            format!("{}~", &base_val[..max_val_w.saturating_sub(1)])
        } else {
            base_val
        };

        let row_vis = INDENT + KEY_W + COLON_W + display_val.len();
        let pad     = inner_width.saturating_sub(row_vis);

        let key_c   = format!("{:KEY_W$}", key).color(kc).bold().to_string();
        let colon_c = " : ".color(kc).to_string();
        let val_c   = display_val.color(vc).to_string();

        println!(
            "{}  {}{}{}{}{}",
            "│".color(ac),
            key_c,
            colon_c,
            val_c,
            " ".repeat(pad),
            "│".color(ac),
        );
    }

    // Bottom border
    let bottom = format!("{}{}{}", "╰".color(ac), "─".repeat(inner_width).color(ac), "╯".color(ac));
    println!("{}", bottom);
    println!();
}

/// Build a compact progress bar for RAM usage, e.g. "▓▓▓▓░░░░"
fn ram_bar(val: &str, width: usize) -> String {
    // Try to parse "X.X GiB / Y.Y GiB (ZZ.Z%)" format
    if let Some(pct_start) = val.rfind('(') {
        if let Some(pct_end) = val.rfind('%') {
            let pct_str = &val[pct_start + 1..pct_end];
            if let Ok(pct) = pct_str.trim().parse::<f64>() {
                let filled = ((pct / 100.0) * width as f64).round() as usize;
                let empty  = width.saturating_sub(filled);
                return format!("[{}{}]", "▓".repeat(filled), "░".repeat(empty));
            }
        }
    }
    String::new()
}

// ── JSON export ───────────────────────────────────────────────────────────────

pub fn render_json(cfg: &VaminfoConfig) {
    let pairs = crate::modules::collect_all(cfg);
    println!("{{");
    let last = pairs.len().saturating_sub(1);
    for (i, (k, v)) in pairs.iter().enumerate() {
        let comma = if i < last { "," } else { "" };
        println!("  \"{}\": \"{}\"{}",
            json_escape(k),
            json_escape(v),
            comma,
        );
    }
    println!("}}");
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"',  "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}

// ── Version page ──────────────────────────────────────────────────────────────

pub fn print_version_page(cfg: &VaminfoConfig) {
    let ac = parse_color(&cfg.ascii_color);
    let tc = parse_color(&cfg.title_color);
    let vc = parse_color(&cfg.value_color);
    let version = env!("CARGO_PKG_VERSION");

    // Outer box
    let w = 52usize;
    let top    = format!("╔{}╗", "═".repeat(w));
    let sep    = format!("╠{}╣", "═".repeat(w));
    let bottom = format!("╚{}╝", "═".repeat(w));

    println!();
    println!("{}", top.color(ac).bold());

    // Title row
    let title = "  V A M I N F O";
    let title_pad = w.saturating_sub(title.len());
    println!("{}{}{}{}",
        "║".color(ac).bold(),
        title.color(tc).bold(),
        " ".repeat(title_pad),
        "║".color(ac).bold(),
    );

    println!("{}", sep.color(ac).bold());

    // Version row
    let v_label = format!("  Version   :  v{}", version);
    let v_pad = w.saturating_sub(v_label.len());
    println!("{}{}{}{}",
        "║".color(ac).bold(),
        v_label.color(vc),
        " ".repeat(v_pad),
        "║".color(ac).bold(),
    );

    // Description row
    let desc = "  Platform  :  Vamora OS";
    let d_pad = w.saturating_sub(desc.len());
    println!("{}{}{}{}",
        "║".color(ac).bold(),
        desc.color(vc),
        " ".repeat(d_pad),
        "║".color(ac).bold(),
    );

    // Author row
    let author = "  Tool      :  System Information";
    let a_pad = w.saturating_sub(author.len());
    println!("{}{}{}{}",
        "║".color(ac).bold(),
        author.color(vc),
        " ".repeat(a_pad),
        "║".color(ac).bold(),
    );

    // Config row
    let cfg_path = crate::config::config_path();
    let cfg_str  = format!("  Config    :  {}", cfg_path.display());
    let c_trunc  = if cfg_str.len() > w { format!("{}~", &cfg_str[..w - 1]) } else { cfg_str };
    let c_pad    = w.saturating_sub(c_trunc.len());
    println!("{}{}{}{}",
        "║".color(ac).bold(),
        c_trunc.color(vc).dimmed(),
        " ".repeat(c_pad),
        "║".color(ac).bold(),
    );

    println!("{}", sep.color(ac).bold());

    // Decorative color band row
    let band_colors = [
        colored::Color::Red,
        colored::Color::Yellow,
        colored::Color::Green,
        colored::Color::Cyan,
        colored::Color::Blue,
        colored::Color::Magenta,
    ];
    print!("{}", "║".color(ac).bold());
    let seg = w / band_colors.len();
    for bc in &band_colors {
        print!("{}", "█".repeat(seg).color(*bc));
    }
    // fill remainder
    let filled = seg * band_colors.len();
    if w > filled { print!("{}", " ".repeat(w - filled)); }
    println!("{}", "║".color(ac).bold());

    println!("{}", bottom.color(ac).bold());
    println!();
}

// ── Info line builder ─────────────────────────────────────────────────────────

fn build_info_lines(
    cfg: &VaminfoConfig,
    sys: &System,
    modules: &[Box<dyn crate::modules::Module>],
    _compact: bool,
) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    if cfg.show_title {
        let title_str = user_host();
        lines.push(title_str.color(parse_color(&cfg.title_color)).bold().to_string());
    }
    if cfg.show_separator {
        let title_str = user_host();
        let sep_w = title_str.len().max(30);
        lines.push(cfg.separator.repeat(sep_w).color(parse_color(&cfg.key_color)).to_string());
    }

    const KEY_W: usize = 13;
    let indent = " ".repeat(KEY_W + 3); // 13 key + " : "

    let mut hit_raw = false;
    for module in modules {
        if let Some(value) = module.collect(sys, cfg) {
            if module.name().is_empty() {
                // Unnamed modules (color blocks) — raw output block
                if !hit_raw {
                    lines.push(String::new());
                    hit_raw = true;
                }
                for sub in value.split('\n') {
                    lines.push(sub.to_string());
                }
            } else {
                let key   = format!("{:KEY_W$}", module.name())
                    .color(parse_color(&cfg.key_color)).bold().to_string();
                let colon = " : ".color(parse_color(&cfg.key_color)).to_string();
                let vc    = parse_color(&cfg.value_color);

                if value.contains('\n') {
                    // Multi-line value: first line gets key+colon, rest get indent
                    let mut parts = value.splitn(64, '\n');
                    let first = parts.next().unwrap_or("");
                    lines.push(format!("{}{}{}", key, colon, first.color(vc)));
                    for cont in parts {
                        lines.push(format!("{}{}", indent, cont.color(vc)));
                    }
                } else {
                    lines.push(format!("{}{}{}", key, colon, value.color(vc)));
                }
            }
        }
    }
    lines
}

// ── Layout renderers ──────────────────────────────────────────────────────────

fn render_split(
    ascii: &[String],
    info: &[String],
    cfg: &VaminfoConfig,
    aw: usize,
    term_cols: usize,
) {
    let padding = 4usize;
    let info_col_width = term_cols.saturating_sub(aw + padding + 1);
    let max_rows = ascii.len().max(info.len());

    for i in 0..max_rows {
        let ascii_col = ascii.get(i).map(|s| s.as_str()).unwrap_or("");
        let info_col  = info.get(i).map(|s| s.as_str()).unwrap_or("");
        let ascii_vis = visible_len(ascii_col);
        let pad = aw.saturating_sub(ascii_vis) + padding;
        let colored_ascii = ascii_col.color(parse_color(&cfg.ascii_color));
        let info_display = if info_col_width > 0 && visible_len(info_col) > info_col_width {
            truncate_to_visible(info_col, info_col_width)
        } else {
            info_col.to_string()
        };
        println!("{}{}{}", colored_ascii, " ".repeat(pad), info_display);
    }
    println!();
}

fn render_stacked(ascii: &[String], info: &[String], cfg: &VaminfoConfig) {
    for line in ascii {
        println!("{}", line.color(parse_color(&cfg.ascii_color)));
    }
    println!();
    render_info_only(info);
}

fn render_info_only(info: &[String]) {
    for line in info {
        println!("{}", line);
    }
    println!();
}

// ── Page title (for help/version/wizard) ──────────────────────────────────────

pub fn print_page_title(cfg: &VaminfoConfig, label: &str) {
    let c = parse_color(&cfg.ascii_color);
    let inner  = format!("  {}  ", label);
    let width  = inner.len().max(44);
    let pad    = width.saturating_sub(inner.len());
    let pad_l  = pad / 2;
    let pad_r  = pad - pad_l;
    let top    = format!("+{}+", "=".repeat(width));
    let middle = format!("|{}{}{}|", " ".repeat(pad_l), inner, " ".repeat(pad_r));
    let bottom = format!("+{}+", "=".repeat(width));
    println!("\n{}", top.color(c).bold());
    println!("{}", middle.color(c).bold());
    println!("{}\n", bottom.color(c).bold());
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn user_host() -> String {
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "user".to_string());
    let host = System::host_name().unwrap_or_else(|| "host".to_string());
    format!("{}@{}", user, host)
}

pub fn parse_color(name: &str) -> Color {
    match name.to_lowercase().as_str() {
        "black"          => Color::Black,
        "red"            => Color::Red,
        "green"          => Color::Green,
        "yellow"         => Color::Yellow,
        "blue"           => Color::Blue,
        "magenta"        => Color::Magenta,
        "cyan"           => Color::Cyan,
        "white"          => Color::White,
        "bright_black"   => Color::BrightBlack,
        "bright_red"     => Color::BrightRed,
        "bright_green"   => Color::BrightGreen,
        "bright_yellow"  => Color::BrightYellow,
        "bright_blue"    => Color::BrightBlue,
        "bright_magenta" => Color::BrightMagenta,
        "bright_cyan"    => Color::BrightCyan,
        "bright_white"   => Color::BrightWhite,
        _                => Color::White,
    }
}
