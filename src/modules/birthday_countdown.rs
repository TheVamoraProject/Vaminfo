use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct BirthdayCountdownModule;

impl Module for BirthdayCountdownModule {
    fn name(&self) -> &'static str { "Birthday" }

    fn collect(&self, _sys: &System, cfg: &VaminfoConfig) -> Option<String> {
        let bday = cfg.greetings.birthday.trim();
        if bday.is_empty() { return None; }

        let (bm, bd) = parse_md(bday)?;
        let (today_y, today_m, today_d) = today_ymd();

        let days = days_until(today_y, today_m, today_d, bm, bd);
        if days == 0 {
            Some("Today! Happy Birthday!".to_string())
        } else if days == 1 {
            Some("Tomorrow!".to_string())
        } else {
            Some(format!("in {} days ({})", days, bday))
        }
    }
}

fn parse_md(s: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 { return None; }
    let m = parts[0].parse::<u32>().ok()?;
    let d = parts[1].parse::<u32>().ok()?;
    if m >= 1 && m <= 12 && d >= 1 && d <= 31 { Some((m, d)) } else { None }
}

fn today_ymd() -> (u32, u32, u32) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    ts_to_ymd(secs)
}

fn days_until(cy: u32, cm: u32, cd: u32, bm: u32, bd: u32) -> u32 {
    // Days until next occurrence of (bm/bd) from (cy/cm/cd)
    let target_this_year = ymd_to_days(cy, bm, bd);
    let today = ymd_to_days(cy, cm, cd);

    if target_this_year >= today {
        target_this_year - today
    } else {
        // Birthday already passed this year — count to next year
        let target_next = ymd_to_days(cy + 1, bm, bd);
        target_next - today
    }
}

fn ymd_to_days(y: u32, m: u32, d: u32) -> u32 {
    let mut days = 0u32;
    for yr in 1970..y {
        days += if is_leap(yr) { 366 } else { 365 };
    }
    let months = [31u32, if is_leap(y) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for mo in 0..(m as usize - 1) {
        days += months[mo];
    }
    days += d - 1;
    days
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
    let months = [31u32, if is_leap(y) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
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
