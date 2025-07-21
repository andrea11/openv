mod config;
mod doc;
mod hooks;
mod logic;

use log::{debug, error, info};
use std::{env, process};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("execute") => {
            debug!("Command: execute");
            debug!("Arguments: {:?}", &args[2..]);
            let input = args[2..].join(" ");
            logic::execute_command(&input);
        }
        Some("check") => {
            debug!("Command: check");
            debug!("Arguments: {:?}", &args[2..]);
            let input = args[2..].join(" ");
            if logic::needs_wrapping(&input) {
                println!("true");
                process::exit(0);
            } else {
                println!("false");
                process::exit(1);
            }
        }
        Some("hook") if args.len() > 2 => {
            debug!("Command: hook");
            debug!("Arguments: {:?}", &args[2..]);
            hooks::print_hook(&args[2]);
        }
        Some("init") if args.len() > 1 => {
            debug!("Command: init");
            debug!("Arguments: {:?}", &args[2..]);
            match hooks::setup_hook(Some(&args[2])) {
                Ok(_) => {
                    info!("Hook setup successfully");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            error!("Invalid command");
            doc::print_usage();
        }
    }
}
