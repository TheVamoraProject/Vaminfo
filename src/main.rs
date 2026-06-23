// --------------------------------------------------------------
// Vaminfo (vamifetch) - A simple system fetch tool for VamoraOS
// https://github.com/TheVamoraProject/Vaminfo
// --------------------------------------------------------------
//
// MIT License
//
// Copyright (c) 2025 Vamora
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod ascii;
mod config;
mod layout;
mod modules;
mod renderer;
mod wizard;

use config::VaminfoConfig;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = VaminfoConfig::load_or_create();
    let subcommand = args.get(1).map(|s| s.as_str());

    match subcommand {
        Some("config") | Some("--config") => {
            wizard::run_wizard(config);
        }
        Some("mini") | Some("--mini") => {
            let mut cfg = config;
            cfg.mini_mode = true;
            renderer::render(&cfg);
        }
        Some("--debug") => {
            println!("[DEBUG] Config path: {}", config::config_path().display());
            println!("[DEBUG] Config: {:#?}", config);
            renderer::render(&config);
        }
        Some("--help") | Some("-h") => {
            print_help(&config);
        }
        Some("--version") | Some("-v") => {
            print_version(&config);
        }
        None => {
            renderer::render(&config);
        }
        Some(unknown) => {
            eprintln!("Unknown command: '{}'. Run 'vaminfo -h' for usage.", unknown);
            std::process::exit(1);
        }
    }
}

fn print_help(cfg: &VaminfoConfig) {
    renderer::print_page_title(cfg, "vaminfo  --  help");
    println!(
        r#"USAGE:
    vaminfo [COMMAND]

COMMANDS:
    (none)        Display system information
    config        Launch interactive configuration wizard
    --mini        Show mini mode (OS, CPU, RAM, Uptime only)
    --debug       Show debug info + system information
    --version     Print version
    --help        Show this help message

CONFIG:
    ~/.VamoraSys/apps/vaminfo/config.vmf

ART:
    ~/.VamoraSys/apps/vaminfo/art/*.vtxt
"#
    );
}

fn print_version(cfg: &VaminfoConfig) {
    renderer::print_page_title(cfg, "vaminfo  --  version");
    println!("  vaminfo  v{}", env!("CARGO_PKG_VERSION"));
    println!("  Vamora OS system information tool");
    println!();
}
