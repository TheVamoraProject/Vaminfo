use crate::ascii::load_ascii;
use crate::config::{GreetingEvent, VaminfoConfig};
use crate::renderer::parse_color;
use colored::Colorize;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// ── Public API ────────────────────────────────────────────────────────────────

/// Returns true if today matches the configured birthday (MM-DD).
pub fn is_birthday_today(cfg: &VaminfoConfig) -> bool {
    if !cfg.greetings.enabled { return false; }
    if cfg.greetings.birthday.trim().is_empty() { return false; }
    let (_, m, d) = today_ymd();
    let today = format!("{:02}-{:02}", m, d);
    cfg.greetings.birthday.trim() == today
}

/// Returns any non-birthday events matching today's date.
pub fn todays_events(cfg: &VaminfoConfig) -> Vec<&GreetingEvent> {
    if !cfg.greetings.enabled { return vec![]; }
    let (_, m, d) = today_ymd();
    let today = format!("{:02}-{:02}", m, d);
    cfg.greetings.events.iter().filter(|e| e.date.trim() == today).collect()
}

/// Full-screen animated birthday screen. Press Space or Enter to skip.
pub fn show_birthday_animation(cfg: &VaminfoConfig) {
    use crossterm::{
        cursor,
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    };
    use std::io::stdout;

    let _ = enable_raw_mode();
    let _ = execute!(stdout(), cursor::Hide, Clear(ClearType::All), cursor::MoveTo(0, 0));

    let colors = [
        colored::Color::BrightRed,
        colored::Color::BrightYellow,
        colored::Color::BrightGreen,
        colored::Color::BrightCyan,
        colored::Color::BrightBlue,
        colored::Color::BrightMagenta,
    ];

    // Extract a display name from "user@host" or the birthday field label
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_else(|_| "You".to_string());

    let start = std::time::Instant::now();
    let frame_ms = 220u128;

    'anim: loop {
        let elapsed = start.elapsed().as_millis();
        let ci = (elapsed / frame_ms) as usize;

        let _ = execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0));
        draw_birthday_frame(cfg, &user, &colors, ci);

        // Poll for keypress without blocking
        loop {
            match event::poll(std::time::Duration::from_millis(0)) {
                Ok(true) => match event::read() {
                    Ok(Event::Key(k)) => match k.code {
                        KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Char('q') => break 'anim,
                        _ => {}
                    },
                    _ => {}
                },
                _ => break,
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(frame_ms as u64));
    }

    let _ = disable_raw_mode();
    let _ = execute!(stdout(), cursor::Show, Clear(ClearType::All), cursor::MoveTo(0, 0));
}

/// Simple (non-animated) full-screen greeting for other events.
pub fn show_greeting(cfg: &VaminfoConfig, event: &GreetingEvent) {
    let c  = parse_color(&cfg.ascii_color);
    let tc = parse_color(&cfg.title_color);

    print!("\x1b[2J\x1b[H");

    let art = load_ascii(cfg);
    for line in &art {
        println!("{}", line.color(c));
    }
    println!();

    let border = "=".repeat(52);
    println!("{}", border.color(c).bold());
    println!();

    let msg = event.message.to_uppercase();
    let pad = 52usize.saturating_sub(msg.len()) / 2;
    println!("{}{}", " ".repeat(pad), msg.color(tc).bold());
    println!();

    let name_line = format!("  {}", event.name);
    println!("{}", name_line.color(c));
    println!();
    println!("{}", border.color(c).bold());
    println!();

    print!("  Press Enter to continue... ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}

// ── Birthday frame renderer ───────────────────────────────────────────────────

static CAKE: &[&str] = &[
    r"              .   .   .              ",
    r"           .     |     .            ",
    r"         .   .   |   .   .          ",
    r"      .  I  . I  |  I .  I  .      ",
    r"   ___I__I___I___|___I___I__I___    ",
    r"  |  ,-.  ,-.  ,-.  ,-.  ,-.    |  ",
    r"  | ( o )( o )( o )( o )( o )   |  ",
    r"  |  `-'  `-'  `-'  `-'  `-'    |  ",
    r"  |______________________________|  ",
    r"  |                              |  ",
    r"  |   H A P P Y  B I R T H D A Y|  ",
    r"  |                              |  ",
    r"  |______________________________|  ",
    r"  |~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|  ",
    r"  |______________________________|  ",
];

static SPARKLES: &[&str] = &[
    "  * . * . * . * . * . * . * . *  ",
    "  . * . * . * . * . * . * . * .  ",
];

fn draw_birthday_frame(
    cfg: &VaminfoConfig,
    user: &str,
    colors: &[colored::Color],
    frame: usize,
) {
    let n = colors.len();

    // Sparkle row (top)
    let top_spark = SPARKLES[frame % SPARKLES.len()];
    println!("{}", top_spark.color(colors[(frame) % n]).bold());
    println!();

    // Cake — each line cycles color offset
    for (i, line) in CAKE.iter().enumerate() {
        let c = colors[(frame + i) % n];
        println!("{}", line.color(c).bold());
    }
    println!();

    // Name banner
    let banner = format!("  Happy Birthday, {}!", user);
    let banner_c = colors[(frame + 2) % n];
    println!("{}", banner.color(banner_c).bold());
    println!();

    // Sparkle row (bottom)
    let bot_spark = SPARKLES[(frame + 1) % SPARKLES.len()];
    println!("{}", bot_spark.color(colors[(frame + 3) % n]).bold());
    println!();

    // ASCII art if available
    let art = load_ascii(cfg);
    if !art.is_empty() {
        let art_c = colors[(frame + 4) % n];
        for line in art.iter().take(6) {
            println!("  {}", line.color(art_c));
        }
        println!();
    }

    // Skip hint — blink by alternating bright/dim
    let hint = if (frame % 4) < 2 {
        "  [ Press SPACE or ENTER to continue ]  "
            .bright_white()
            .bold()
            .to_string()
    } else {
        "  [ Press SPACE or ENTER to continue ]  "
            .white()
            .to_string()
    };
    println!("{}", hint);
}

// ── Date helpers ──────────────────────────────────────────────────────────────

fn today_ymd() -> (u32, u32, u32) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    ts_to_ymd(secs)
}

fn ts_to_ymd(ts: u64) -> (u32, u32, u32) {
    let mut remaining = (ts / 86400) as u32;
    let mut y = 1970u32;
    loop {
        let days_in = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in { break; }
        remaining -= days_in;
        y += 1;
    }
    let months = [
        31u32,
        if is_leap(y) { 29 } else { 28 },
        31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];
    let mut m = 0u32;
    for &dm in &months {
        if remaining < dm { break; }
        remaining -= dm;
        m += 1;
    }
    (y, m + 1, remaining + 1)
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
