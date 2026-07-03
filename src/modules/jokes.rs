use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct JokesModule;

/// Each entry is (setup, punchline)
static JOKES: &[(&str, &str)] = &[
    ("Why do programmers prefer dark mode?",       "Because light attracts bugs."),
    ("Why did the developer go broke?",            "Because he used up all his cache."),
    ("How do you comfort a JavaScript bug?",       "You console it."),
    ("Why do Java developers wear glasses?",       "Because they don't C#."),
    ("A SQL query walks into a bar, walks up to two tables and asks...", "\"Can I join you?\""),
    ("Why did the Linux admin cross the road?",    "To mount the other side."),
    ("What's a computer's favorite snack?",        "Micro-chips."),
    ("Why do programmers always mix up Christmas and Halloween?", "Because Oct 31 == Dec 25."),
    ("How many programmers does it take to change a light bulb?", "None, that's a hardware problem."),
    ("Why was the shell script nervous?",          "It had too many forks."),
    ("What's the object-oriented way to become wealthy?", "Inheritance."),
    ("Why did the programmer quit his job?",       "Because he didn't get arrays."),
];

impl Module for JokesModule {
    fn name(&self) -> &'static str { "Joke" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let (setup, punchline) = JOKES[random_index(JOKES.len())];
        // Two-line format: setup on first line, punchline indented on second
        Some(format!("{}\n  -> {}", setup, punchline))
    }
}

fn random_index(len: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize;
    (seed / 7) % len
}
