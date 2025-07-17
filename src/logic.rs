use crate::config::Config;
use log::{debug, info, warn};
use std::{env, fs, path::PathBuf};

pub fn wrap_command(original_command: &str) -> String {
    let trimmed_original = original_command.trim();
    if trimmed_original.is_empty() {
        warn!("No command provided");
        return String::new();
    }

    let optional_env_file = find_valid_env_file();

    if optional_env_file.is_none() {
        warn!("No valid .env file found");
        return original_command.to_string();
    }

    let env_file = optional_env_file.unwrap();
    let global_config = Config::load(&dirs::home_dir().unwrap().join(".openv.toml"));
    let local_config = Config::load(&env_file.parent().unwrap().join(".openv.toml"));
    let config = Config::merge(&global_config, &local_config);

    if !config.needs_wrapping(original_command) {
        warn!("Command '{original_command}' is not allowed");
        return original_command.to_string();
    }
    let mut full_cmd = format!(
        "op run --env-file=\"{}\" -- {original_command}",
        env_file.display()
    );

    if config.disable_masking.unwrap_or(false) {
        full_cmd.push_str(" --no-masking");
    }

    info!("Wrapped command: {full_cmd}");
    format!("{full_cmd}\n")
}

fn find_valid_env_file() -> Option<PathBuf> {
    let cwd = env::current_dir().ok()?;
    debug!("Current working directory: {cwd:?}");

    let root_project_folder = find_git_root(cwd.clone()).or_else(|| find_env_root(cwd.clone()))?;

    let env_file = root_project_folder.join(".env");

    if env_file.exists() && fs::read_to_string(&env_file).ok()?.contains("op://") {
        debug!("Valid .env file found: {env_file:?}");
        Some(env_file)
    } else {
        warn!("No valid .env file found");
        None
    }
}

fn find_git_root(mut dir: PathBuf) -> Option<PathBuf> {
    loop {
        if dir.join(".git").exists() {
            debug!("Git root found: {dir:?}");
            return Some(dir);
        }

        if !dir.pop() {
            warn!("No git root found");
            return None;
        }
    }
}

fn find_env_root(mut dir: PathBuf) -> Option<PathBuf> {
    loop {
        if dir.join(".env").exists() {
            debug!(".env root found: {dir:?}");
            return Some(dir);
        }

        if !dir.pop() {
            warn!("No .env root found");
            return None;
        }
    }
}
