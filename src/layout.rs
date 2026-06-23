use crossterm::terminal;

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutMode {
    Split,
    Stacked,
}

pub struct TerminalSize {
    pub cols: u16,
    pub rows: u16,
}

pub fn detect_terminal_size() -> TerminalSize {
    match terminal::size() {
        Ok((cols, rows)) => TerminalSize { cols, rows },
        Err(_) => TerminalSize { cols: 80, rows: 24 },
    }
}

pub fn decide_layout(term: &TerminalSize, ascii_width: usize) -> LayoutMode {
    let min_split_width = ascii_width as u16 + 50;
    if term.cols >= min_split_width && term.rows >= 10 {
        LayoutMode::Split
    } else {
        LayoutMode::Stacked
    }
}
