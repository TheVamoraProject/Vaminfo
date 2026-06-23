use crate::config::{art_dir, VaminfoConfig};
use std::fs;

const DEFAULT_ASCII: &str = r#"
          ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒       
       ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    
     ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  
    ▒▒▒▒▒▒▒▒▒▒   ▒▒▒▒▒▒▒▒▒▒   ▒▒▒▒   ▒▒▒▒▒▒▒▒▒▒ 
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒     ▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒▒▒▒      ▒      ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒▒▒      ▒       ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒▒      ▒        ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒      ▒         ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒      ▒          ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒      ▒           ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒      ▒░           ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ░      ▒      ▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒           ▒      ▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒          ▒      ▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒         ▒      ▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒        ▒      ▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒       ▒      ▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒      ▒      ░▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒      ▒▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
    ▒▒▒▒▒▒▒▒▒▒   ▒▒▒▒   ▒▒▒▒▒▒▒▒▒▒   ▒▒▒▒▒▒▒▒▒▒ 
     ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  
       ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    
         ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒       
"#;

pub fn load_ascii(config: &VaminfoConfig) -> Vec<String> {
    let art_path = art_dir().join(&config.ascii_file);

    let content = if art_path.exists() {
        match fs::read_to_string(&art_path) {
            Ok(c) if !c.trim().is_empty() => c,
            _ => DEFAULT_ASCII.to_string(),
        }
    } else {
        DEFAULT_ASCII.to_string()
    };

    content.lines().map(|l| l.to_string()).collect()
}

pub fn ascii_width(lines: &[String]) -> usize {
    lines.iter().map(|l| visible_len(l)).max().unwrap_or(0)
}

pub fn visible_len(s: &str) -> usize {
    let mut len = 0usize;
    let mut in_escape = false;
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape && ch == 'm' {
            in_escape = false;
        } else if !in_escape {
            len += 1;
        }
    }
    len
}

/// Truncate `s` to at most `max` visible characters, preserving ANSI codes.
/// Appends a reset sequence if truncation happened so colors don't bleed.
pub fn truncate_to_visible(s: &str, max: usize) -> String {
    if visible_len(s) <= max {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut vis = 0usize;
    let mut in_esc = false;
    let mut esc_buf = String::new();

    for ch in s.chars() {
        if ch == '\x1b' {
            in_esc = true;
            esc_buf.clear();
            esc_buf.push(ch);
        } else if in_esc {
            esc_buf.push(ch);
            if ch == 'm' {
                in_esc = false;
                // Only emit escape sequences while we still have room
                if vis < max {
                    out.push_str(&esc_buf);
                }
            }
        } else {
            if vis >= max {
                break;
            }
            out.push(ch);
            vis += 1;
        }
    }
    // Reset so colors don't bleed into the next column
    out.push_str("\x1b[0m");
    out
}
