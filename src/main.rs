mod config;
mod doc;
mod hooks;
mod logic;

use log::{debug, error, info};
use std::env;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("wrap") => {
            info!("Command: wrap");
            debug!("Arguments: {:?}", &args[2..]);
            let input = args
                .iter()
                .skip(2)
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            let wrapped = logic::wrap_command(&input);
            println!("{wrapped}");
        }
        Some("hook") if args.len() > 2 => {
            info!("Command: hook");
            hooks::print_hook(&args[2]);
        }
        _ => {
            error!("Invalid command");
            doc::print_usage();
        }
    }
}
