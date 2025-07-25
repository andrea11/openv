pub mod bash;
pub mod fish;
pub mod zsh;

use crate::hooks::bash::BASH_HOOK;
use crate::hooks::fish::FISH_HOOK;
use crate::hooks::zsh::ZSH_HOOK;

use std::env;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use clap::ValueEnum;
use strum_macros::Display;

#[derive(Copy, Clone, Display, Debug, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

pub fn print_hook(shell: &Shell) {
    match shell {
        Shell::Bash => {
            println!("{BASH_HOOK}");
        }
        Shell::Zsh => {
            println!("{ZSH_HOOK}");
        }
        Shell::Fish => {
            println!("{FISH_HOOK}");
        }
    }
}

pub fn setup_hook(shell: Option<&Shell>) -> Result<(), String> {
    let shell = match shell {
        Some(s) => s.to_string(),
        _ => env::var("SHELL")
            .ok()
            .and_then(|s| s.split('/').next_back().map(String::from))
            .ok_or("Could not detect shell. Please specify one using `--shell <shell>`.")?,
    };

    let home = env::var("HOME").map_err(|_| "Could not determine $HOME directory.".to_string())?;

    let config_path = match shell.as_str() {
        "bash" => Some(PathBuf::from(&home).join(".bashrc")),
        "zsh" => {
            let zdotdir = env::var("ZDOTDIR").unwrap_or_else(|_| home.clone());
            Some(PathBuf::from(zdotdir).join(".zshrc"))
        }
        "fish" => {
            let config_home =
                env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{home}/.config"));
            Some(PathBuf::from(config_home).join("fish/config.fish"))
        }
        _ => None,
    }
    .ok_or_else(|| format!("Unsupported shell: {shell}"))?;

    let init_command = match shell.as_str() {
        "fish" => "openv init fish | source".to_string(),
        _ => format!("eval \"$(openv init {shell})\""),
    };

    let already_present = match read_to_string(&config_path) {
        Ok(content) => content.contains(&init_command),
        Err(_) => false,
    };

    if already_present {
        println!(
            "Init hook already present in {} for shell '{}'. Skipping.",
            config_path.display(),
            shell
        );
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&config_path)
        .map_err(|e| format!("Unable to open shell config file: {e}"))?;

    writeln!(file, "{init_command}")
        .map_err(|e| format!("Unable to write to shell config file: {e}"))?;

    println!(
        "Added init hook to {} for shell '{}'. You may need to restart your terminal.",
        config_path.display(),
        shell
    );

    Ok(())
}
