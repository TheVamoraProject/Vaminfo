use super::Module;
use crate::config::VaminfoConfig;
use colored::Colorize;
use sysinfo::System;

pub struct ColorBlocksBig;
pub struct ColorBlocksSmall;

impl Module for ColorBlocksBig {
    fn name(&self) -> &'static str { "" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let b = "    ";
        // Row 1: normal/dark colors
        let row1 = format!(
            "{}{}{}{}{}{}{}{}",
            b.on_black(),
            b.on_red(),
            b.on_green(),
            b.on_yellow(),
            b.on_blue(),
            b.on_magenta(),
            b.on_cyan(),
            b.on_white(),
        );
        // Row 2: bright colors (lighter row like neofetch)
        let row2 = format!(
            "{}{}{}{}{}{}{}{}",
            b.on_bright_black(),
            b.on_bright_red(),
            b.on_bright_green(),
            b.on_bright_yellow(),
            b.on_bright_blue(),
            b.on_bright_magenta(),
            b.on_bright_cyan(),
            b.on_bright_white(),
        );
        Some(format!("{}\n{}", row1, row2))
    }
}

impl Module for ColorBlocksSmall {
    fn name(&self) -> &'static str { "" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        let b = "  ";
        let row1 = format!(
            "{}{}{}{}{}{}{}{}",
            b.on_black(),
            b.on_red(),
            b.on_green(),
            b.on_yellow(),
            b.on_blue(),
            b.on_magenta(),
            b.on_cyan(),
            b.on_white(),
        );
        let row2 = format!(
            "{}{}{}{}{}{}{}{}",
            b.on_bright_black(),
            b.on_bright_red(),
            b.on_bright_green(),
            b.on_bright_yellow(),
            b.on_bright_blue(),
            b.on_bright_magenta(),
            b.on_bright_cyan(),
            b.on_bright_white(),
        );
        Some(format!("{}\n{}", row1, row2))
    }
}
