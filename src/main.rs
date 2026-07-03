mod ascii;
mod config;
mod greetings;
mod layout;
mod modules;
mod renderer;
mod tips;
mod wizard;

use colored::Colorize;
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
        Some("--mini") => {
            let mut cfg = config;
            cfg.mini_mode = true;
            renderer::render(&cfg);
        }
        Some("--json") => {
            renderer::render_json(&config);
        }
        Some("--tip") => {
            print_tip(&config);
        }
        Some("--debug") => {
            let ac = renderer::parse_color(&config.ascii_color);
            let tag = "[DEBUG]".color(ac).bold().to_string();
            println!("{} Config path : {}", tag, config::config_path().display());
            println!("{} Art dir     : {}", tag, config::art_dir().display());
            println!("{} Config      :\n{:#?}", tag, config);
            println!();
            renderer::render(&config);
        }
        Some("--help") | Some("-h") => {
            print_help(&config);
        }
        Some("--version") | Some("-v") => {
            renderer::print_version_page(&config);
        }
        None => {
            if greetings::is_birthday_today(&config) {
                greetings::show_birthday_animation(&config);
            } else {
                let events = greetings::todays_events(&config);
                for event in events {
                    greetings::show_greeting(&config, event);
                }
            }
            renderer::render(&config);
        }
        Some(unknown) => {
            eprintln!("Unknown command: '{}'. Run 'vaminfo --help' for usage.", unknown);
            std::process::exit(1);
        }
    }
}

fn print_help(cfg: &VaminfoConfig) {
    renderer::print_page_title(cfg, "vaminfo -- help");
    let tc = renderer::parse_color(&cfg.title_color);
    let ac = renderer::parse_color(&cfg.ascii_color);

    println!("{}", "USAGE:".color(tc).bold());
    println!("    vaminfo [COMMAND]");
    println!();

    println!("{}", "COMMANDS:".color(tc).bold());
    println!("    {}        Display system information", "(none)".color(ac));
    println!("    {}       Launch interactive configuration wizard", "config".color(ac));
    println!("    {}        Show mini mode (OS, Host, RAM, Uptime)", "--mini".color(ac));
    println!("    {}        Export all hardware stats as JSON", "--json".color(ac));
    println!("    {}         Display a random Linux tip", "--tip".color(ac));
    println!("    {}       Show debug info + system information", "--debug".color(ac));
    println!("    {}     Print version information", "--version".color(ac));
    println!("    {}        Show this help message", "--help".color(ac));
    println!();

    println!("{}", "CONFIG:".color(tc).bold());
    println!("    {}", config::config_path().display());
    println!();

    println!("{}", "ART:".color(tc).bold());
    println!("    {}/*.vtxt", config::art_dir().display());
    println!();
}

fn print_tip(cfg: &VaminfoConfig) {
    renderer::print_page_title(cfg, "Linux Tip");
    println!("  {}", tips::random_tip());
    println!();
}
