use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SysAgeModule;

impl Module for SysAgeModule {
    fn name(&self) -> &'static str { "System Age" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let birth_ts = get_root_birth_time()?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs();

        if birth_ts == 0 || birth_ts > now { return None; }

        let diff = now - birth_ts;
        let days  = diff / 86400;
        let years = days / 365;
        let rem   = days % 365;

        let age_str = if years > 0 {
            format!("{}y {}d", years, rem)
        } else {
            format!("{}d", days)
        };

        // Also format the creation date
        let birth_sys = UNIX_EPOCH + std::time::Duration::from_secs(birth_ts);
        if let Ok(dt) = birth_sys.duration_since(UNIX_EPOCH) {
            let ts = dt.as_secs();
            let (y, mo, d) = ts_to_ymd(ts);
            return Some(format!("{} (since {:04}-{:02}-{:02})", age_str, y, mo, d));
        }
        Some(age_str)
    }
}

fn get_root_birth_time() -> Option<u64> {
    // Linux: stat -c %W / gives inode birth time (0 if unsupported)
    if let Ok(out) = Command::new("stat").args(["-c", "%W", "/"]).output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if let Ok(n) = s.parse::<u64>() {
                if n > 0 { return Some(n); }
            }
        }
    }

    // macOS: stat -f "%SB" /
    if let Ok(out) = Command::new("stat").args(["-f", "%SB", "/"]).output() {
        // Output is like "Jan  1 00:00:00 2020" - too complex to parse portably
        // Skip for now
        let _ = out;
    }

    // Fallback: read /lost+found mtime as rough estimate
    if let Ok(meta) = std::fs::metadata("/lost+found") {
        if let Ok(modified) = meta.modified() {
            if let Ok(dur) = modified.duration_since(UNIX_EPOCH) {
                return Some(dur.as_secs());
            }
        }
    }

    None
}

/// Simple timestamp → (year, month, day) without chrono
fn ts_to_ymd(ts: u64) -> (u32, u32, u32) {
    let days_since_epoch = (ts / 86400) as u32;
    let mut y = 1970u32;
    let mut remaining = days_since_epoch;
    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in_year { break; }
        remaining -= days_in_year;
        y += 1;
    }
    let months = [31u32, if is_leap(y) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m = 0u32;
    for &days in &months {
        if remaining < days { break; }
        remaining -= days;
        m += 1;
    }
    (y, m + 1, remaining + 1)
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
