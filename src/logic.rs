use crate::config::Config;
use log::{debug, info, warn};
use std::{env, fs, path::PathBuf};

pub fn wrap_command(original: &str) -> String {
    let args: Vec<&str> = original.split_whitespace().collect();
    if args.is_empty() {
        warn!("No command provided");
        return String::new();
    }

    let global_config = Config::load(&dirs::home_dir().unwrap().join(".openv/.openv.toml"));
    let local_config = Config::load(&env::current_dir().unwrap().join(".openv.toml"));
    let config = Config::merge(&global_config, &local_config);

    let command = args.join(" ");

    if !config.needs_wrapping(&command) {
        warn!("Command '{command}' is not allowed");
        return command;
    }

    if let Some(env_file) = find_valid_env_file() {
        let mut full_cmd = format!(
            "op run --env-file=\"{}\" -- {}",
            env_file.display(),
            original
        );

        if config.disable_masking.unwrap_or(false) {
            full_cmd.push_str(" --no-masking");
        }

        info!("Wrapped command: {full_cmd}");
        return format!("{full_cmd}\n");
    }

    warn!("No valid .env file found");
    String::new()
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
