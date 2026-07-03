use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;

pub struct QuotesModule;

static QUOTES: &[(&str, &str)] = &[
    ("Linus Torvalds",   "Talk is cheap. Show me the code."),
    ("Ken Thompson",     "One of my most productive days was throwing away 1000 lines of code."),
    ("Eric S. Raymond",  "Every good work of software starts by scratching a developer's personal itch."),
    ("Rob Pike",         "Data dominates. If you've chosen the right data structures and organized things well, the algorithms will almost always be self-evident."),
    ("Donald Knuth",     "Premature optimization is the root of all evil."),
    ("Edsger Dijkstra",  "The question of whether computers can think is like the question of whether submarines can swim."),
    ("Brian Kernighan",  "Debugging is twice as hard as writing the code in the first place. Therefore, if you write the code as cleverly as possible, you are, by definition, not smart enough to debug it."),
    ("Larry Wall",       "The three chief virtues of a programmer are: Laziness, Impatience and Hubris."),
    ("Richard Stallman", "Free software is a matter of liberty, not price. To understand the concept, you should think of 'free' as in 'free speech'."),
    ("Alan Kay",         "The best way to predict the future is to invent it."),
    ("Dennis Ritchie",   "Unix is simple. It just takes a genius to understand its simplicity."),
    ("Linus Torvalds",   "Software is like sex: it's better when it's free."),
    ("Alan Turing",      "We can only see a short distance ahead, but we can see plenty there that needs to be done."),
    ("Claude Shannon",   "Information is the resolution of uncertainty."),
    ("Bjarne Stroustrup","C makes it easy to shoot yourself in the foot; C++ makes it harder, but when you do it blows your whole leg off."),
];

// Wrap text at word boundaries for a given column width.
fn word_wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current = word.to_string();
        }
    }
    if !current.is_empty() { lines.push(current); }
    lines
}

impl Module for QuotesModule {
    fn name(&self) -> &'static str { "Quote" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let (author, quote) = QUOTES[random_index(QUOTES.len())];

        // Wrap quote at 58 chars (fits nicely after "Quote         : ")
        let wrapped = word_wrap(quote, 58);
        let mut out = String::new();
        for (i, line) in wrapped.iter().enumerate() {
            if i == 0 {
                out.push('"');
                out.push_str(line);
            } else {
                out.push('\n');
                out.push_str(line);
            }
        }
        out.push('"');
        out.push_str(&format!("\n  -- {}", author));
        Some(out)
    }
}

fn random_index(len: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize;
    seed % len
}
