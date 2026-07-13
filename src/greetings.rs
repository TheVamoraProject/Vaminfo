use crate::config::{GreetingEvent, VaminfoConfig};
use crate::renderer::parse_color;
use colored::Colorize;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// ── Public API ────────────────────────────────────────────────────────────────

pub fn is_birthday_today(cfg: &VaminfoConfig) -> bool {
    if !cfg.greetings.enabled { return false; }
    if cfg.greetings.birthday.trim().is_empty() { return false; }
    let (_, m, d) = today_ymd();
    let today = format!("{:02}-{:02}", m, d);
    cfg.greetings.birthday.trim() == today
}

pub fn todays_events(cfg: &VaminfoConfig) -> Vec<&GreetingEvent> {
    if !cfg.greetings.enabled { return vec![]; }
    let (_, m, d) = today_ymd();
    let today = format!("{:02}-{:02}", m, d);
    cfg.greetings.events.iter().filter(|e| e.date.trim() == today).collect()
}

// ── Birthday animation ────────────────────────────────────────────────────────

pub fn show_birthday_animation(_cfg: &VaminfoConfig) {
    use crossterm::{
        cursor,
        event::{self, Event, KeyCode},
        execute,
        terminal::{self, Clear, ClearType},
    };

    let _ = terminal::enable_raw_mode();
    let mut out = io::stdout();
    let _ = execute!(out, cursor::Hide, Clear(ClearType::All), cursor::MoveTo(0, 0));

    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_else(|_| "You".to_string());

    let palette = [
        colored::Color::BrightRed,
        colored::Color::BrightYellow,
        colored::Color::BrightGreen,
        colored::Color::BrightCyan,
        colored::Color::BrightBlue,
        colored::Color::BrightMagenta,
    ];

    let frame_ms = 300u128;
    let start    = std::time::Instant::now();

    'anim: loop {
        let frame = (start.elapsed().as_millis() / frame_ms) as usize;
        let lines = build_birthday_frame(&user, &palette, frame);

        let _ = execute!(out, cursor::MoveTo(0, 0), Clear(ClearType::All));
        for line in &lines {
            // raw mode requires explicit \r before \n
            let _ = out.write_all(line.as_bytes());
            let _ = out.write_all(b"\r\n");
        }
        let _ = out.flush();

        loop {
            match event::poll(std::time::Duration::from_millis(0)) {
                Ok(true) => match event::read() {
                    Ok(Event::Key(k)) => match k.code {
                        KeyCode::Char(' ')
                        | KeyCode::Enter
                        | KeyCode::Char('q')
                        | KeyCode::Char('Q') => break 'anim,
                        _ => {}
                    },
                    _ => {}
                },
                _ => break,
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(frame_ms as u64));
    }

    let _ = terminal::disable_raw_mode();
    let _ = execute!(out, cursor::Show, Clear(ClearType::All), cursor::MoveTo(0, 0));
}

// ── Cake frame builder ────────────────────────────────────────────────────────
//
//  The cake is split into CANDLE lines (animated) + BODY lines (static).
//  Every line is exactly CAKE_W visible characters so the box aligns.

const CAKE_W: usize = 31;

// Candle flames alternate each frame
static FLAMES_A: &str = "    i     i     i     i     i  ";   // thin flames
static FLAMES_B: &str = "   ,*,   ,*,   ,*,   ,*,   ,*, ";  // wide flames
static STEMS:    &str = "    |     |     |     |     |  ";

// Static cake body (CAKE_W chars each, padded with trailing spaces as needed)
static BODY: &[&str] = &[
    " ___|_____|_____|_____|_____|_",
    " |  ~  ~  ~  ~  ~  ~  ~  ~ |",
    " |  ,-.   ,-.   ,-.   ,-.  |",
    " | ( o ) ( o ) ( o ) ( o ) |",
    " |  `-'   `-'   `-'   `-'  |",
    " |___________________________| ",
    " |                           |",
    " |   H A P P Y               |",
    " |   B I R T H D A Y  ! ! !  |",
    " |___________________________|",
    " |~~~~~~~~~~~~~~~~~~~~~~~~~~~|",
    " |___________________________|",
];

fn pad(s: &str) -> String {
    if s.len() >= CAKE_W {
        s[..CAKE_W].to_string()
    } else {
        format!("{:width$}", s, width = CAKE_W)
    }
}

fn build_birthday_frame(
    user: &str,
    palette: &[colored::Color],
    frame: usize,
) -> Vec<String> {
    let n = palette.len();
    let mut lines: Vec<String> = Vec::new();

    // Sparkle header
    let sparkle = if frame % 2 == 0 {
        "  * . * . * . * . * . * . * . * . *"
    } else {
        "  . * . * . * . * . * . * . * . * ."
    };
    lines.push(sparkle.color(palette[frame % n]).bold().to_string());
    lines.push(String::new());

    // Candle flames (flicker every frame)
    let flame_line = if frame % 2 == 0 { FLAMES_A } else { FLAMES_B };
    lines.push(pad(flame_line).color(palette[(frame + 1) % n]).bold().to_string());
    lines.push(pad(STEMS).color(colored::Color::BrightYellow).to_string());

    // Cake body — each row cycles through the palette
    for (i, row) in BODY.iter().enumerate() {
        lines.push(pad(row).color(palette[(frame + i) % n]).bold().to_string());
    }
    lines.push(String::new());

    // Name banner
    let banner = format!("   Happy Birthday, {}!   ", user);
    lines.push(banner.color(palette[(frame + 3) % n]).bold().to_string());
    lines.push(String::new());

    // Sparkle footer
    let sparkle2 = if frame % 2 == 0 {
        "  . * . * . * . * . * . * . * . * ."
    } else {
        "  * . * . * . * . * . * . * . * . *"
    };
    lines.push(sparkle2.color(palette[(frame + 2) % n]).bold().to_string());
    lines.push(String::new());

    // Blinking hint
    let hint = if (frame / 2) % 2 == 0 {
        "  [ Press SPACE or ENTER to continue ]"
            .bright_white().bold().to_string()
    } else {
        "  [ Press SPACE or ENTER to continue ]"
            .white().dimmed().to_string()
    };
    lines.push(hint);

    lines
}

// ── Simple event greeting (not animated, no raw mode) ────────────────────────

pub fn show_greeting(cfg: &VaminfoConfig, event: &GreetingEvent) {
    use crate::ascii::load_ascii;

    let c  = parse_color(&cfg.ascii_color);
    let tc = parse_color(&cfg.title_color);

    print!("\x1b[2J\x1b[H");

    let art = load_ascii(cfg);
    for line in &art { println!("{}", line.color(c)); }
    println!();

    let border = "=".repeat(52);
    println!("{}", border.color(c).bold());
    println!();

    let msg = event.message.to_uppercase();
    let pad_len = 52usize.saturating_sub(msg.len()) / 2;
    println!("{}{}", " ".repeat(pad_len), msg.color(tc).bold());
    println!();
    println!("{}", format!("  {}", event.name).color(c));
    println!();
    println!("{}", border.color(c).bold());
    println!();

    print!("  Press Enter to continue... ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}

// ── Date helpers ──────────────────────────────────────────────────────────────

fn today_ymd() -> (u32, u32, u32) {
    ts_to_ymd(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    )
}

fn ts_to_ymd(ts: u64) -> (u32, u32, u32) {
    let mut rem = (ts / 86400) as u32;
    let mut y   = 1970u32;
    loop {
        let dy = if is_leap(y) { 366 } else { 365 };
        if rem < dy { break; }
        rem -= dy;
        y += 1;
    }
    let months = [
        31u32, if is_leap(y) { 29 } else { 28 },
        31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];
    let mut m = 0u32;
    for &dm in &months { if rem < dm { break; } rem -= dm; m += 1; }
    (y, m + 1, rem + 1)
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
