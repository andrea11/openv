mod config;
mod hooks;
mod logic;

use clap::{Parser, Subcommand};
use log::{info, trace};

#[derive(Parser)]
#[command(about, author, version)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..4))]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a command wrapped with op CLI
    Execute {
        /// The command to execute
        #[arg(required = true, num_args = 1..)]
        cmd: Vec<String>,
    },
    /// Check if a command needs to be wrapped with op CLI
    Check {
        /// The command to check
        #[arg(required = true, num_args = 1..)]
        cmd: Vec<String>,
    },
    /// Print the hook for a given shell
    Hook {
        /// The shell to print the hook for
        #[arg(required = true, value_enum)]
        shell: hooks::Shell,
    },
    /// Setup the hook for a given shell
    Init {
        /// The shell to setup the hook for
        #[arg(required = true, value_enum)]
        shell: hooks::Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut log_builder: env_logger::Builder =
        env_logger::Builder::from_env(env_logger::Env::default());

    if cli.verbose > 0 {
        let level = match cli.verbose {
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        };
        log_builder.filter_level(level);
    }

    log_builder.init();

    match &cli.command {
        Commands::Execute { cmd: input } => {
            trace!("Command: execute, Arguments: {input:?}");
            match logic::execute_command(&input.join(" ")) {
                Ok(code) => std::process::exit(code),
                Err(err) => {
                    eprintln!("Error: {err}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Check { cmd: input } => {
            trace!("Command: check, Arguments: {input:?}");
            let needs_wrapping = logic::needs_wrapping(&input.join(" "));
            println!("{needs_wrapping}");
            std::process::exit(!needs_wrapping as i32); // Exit with 0 if wrapping is not needed, 1 otherwise
        }
        Commands::Hook { shell } => {
            trace!("Command: hook, Arguments: {shell:?}");
            hooks::print_hook(shell);
        }
        Commands::Init { shell } => {
            trace!("Command: init, Arguments: {shell:?}");
            match hooks::setup_hook(Some(shell)) {
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
    }
}
