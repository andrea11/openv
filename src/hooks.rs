pub mod bash;
pub mod fish;
pub mod zsh;

use crate::hooks::bash::BASH_HOOK;
use crate::hooks::fish::FISH_HOOK;
use crate::hooks::zsh::ZSH_HOOK;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn print_hook(shell: &str) {
    match shell {
        "bash" => {
            println!("{BASH_HOOK}");
        }
        "zsh" => {
            println!("{ZSH_HOOK}");
        }
        "fish" => {
            println!("{FISH_HOOK}");
        }
        _ => {
            eprintln!("Unsupported shell: {shell}");
        }
    }
}

pub fn setup_hook(shell: Option<&str>) -> Result<(), String> {
    let shell = match shell {
        Some(s) => s.to_string(),
        _ => env::var("SHELL")
            .ok()
            .and_then(|s| s.split('/').last().map(String::from))
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
                env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{}/.config", home));
            Some(PathBuf::from(config_home).join("fish/config.fish"))
        }
        _ => None,
    }
    .ok_or_else(|| format!("Unsupported shell: {}", shell))?;

    let init_command = match shell.as_str() {
        "fish" => format!("eval (openv init {})", shell),
        _ => format!("eval \"$(openv init {})\"", shell),
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&config_path)
        .map_err(|e| format!("Unable to open shell config file: {}", e))?;

    writeln!(file, "{}", init_command)
        .map_err(|e| format!("Unable to write to shell config file: {}", e))?;

    println!(
        "Added init hook to {} for shell '{}'. You may need to restart your terminal.",
        config_path.display(),
        shell
    );

    Ok(())
}
