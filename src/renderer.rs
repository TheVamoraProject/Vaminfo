use crate::ascii::{ascii_width, load_ascii, truncate_to_visible, visible_len};
use crate::config::VaminfoConfig;
use crate::layout::{decide_layout, detect_terminal_size, LayoutMode};
use crate::modules::{build_mini_modules, build_modules};
use colored::{Color, Colorize};
use sysinfo::System;

pub fn render(cfg: &VaminfoConfig) {
    let sys = System::new_all();

    let show_ascii = !cfg.mini_mode || cfg.mini_mode_ascii;

    let ascii_lines = if show_ascii { load_ascii(cfg) } else { vec![] };
    let aw = if show_ascii { ascii_width(&ascii_lines) } else { 0 };
    let term = detect_terminal_size();
    let layout = if show_ascii {
        decide_layout(&term, aw)
    } else {
        LayoutMode::Stacked
    };

    let modules = if cfg.mini_mode {
        build_mini_modules()
    } else {
        build_modules(cfg)
    };

    let mut info_lines: Vec<String> = Vec::new();

    if cfg.show_title {
        let title_str = user_host();
        info_lines.push(
            title_str
                .color(parse_color(&cfg.title_color))
                .bold()
                .to_string(),
        );
    }

    if cfg.show_separator {
        let title_str = user_host();
        let sep_width = title_str.len().max(30);
        info_lines.push(
            cfg.separator
                .repeat(sep_width)
                .color(parse_color(&cfg.key_color))
                .to_string(),
        );
    }

    let mut hit_raw = false;

    for module in &modules {
        if let Some(value) = module.collect(&sys, cfg) {
            if module.name().is_empty() {
                // Raw / color block — add blank line before first one
                if !hit_raw {
                    info_lines.push(String::new());
                    hit_raw = true;
                }
                // Color blocks can be multi-line (2 rows)
                for sub_line in value.split('\n') {
                    info_lines.push(sub_line.to_string());
                }
            } else {
                let key = format!("{:13}", module.name())
                    .color(parse_color(&cfg.key_color))
                    .bold()
                    .to_string();
                let colon = " : ".color(parse_color(&cfg.key_color)).to_string();
                let val = value.color(parse_color(&cfg.value_color)).to_string();
                info_lines.push(format!("{}{}{}", key, colon, val));
            }
        }
    }

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

fn user_host() -> String {
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "user".to_string());
    let host = System::host_name().unwrap_or_else(|| "host".to_string());
    format!("{}@{}", user, host)
}

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

        // Truncate the info column if it would overflow the terminal
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

/// Print the styled vaminfo page title (follows ascii_color)
pub fn print_page_title(cfg: &VaminfoConfig, label: &str) {
    let c = parse_color(&cfg.ascii_color);
    let width = 42usize;
    let inner = format!(" {} ", label);
    let pad_total = width.saturating_sub(inner.len() + 2); // -2 for border chars
    let pad_l = pad_total / 2;
    let pad_r = pad_total - pad_l;
    let top    = format!("+{}+", "=".repeat(width));
    let middle = format!("|{}{}{}|", " ".repeat(pad_l), inner, " ".repeat(pad_r));
    let bottom = format!("+{}+", "=".repeat(width));
    println!("{}", top.color(c).bold());
    println!("{}", middle.color(c).bold());
    println!("{}", bottom.color(c).bold());
    println!();
}
