use crate::config::Config;
use log::{debug, error, warn};
use shlex::{split, try_quote};
use std::{
    env, fs,
    path::Path,
    path::PathBuf,
    process::{exit, Command},
};

pub fn execute_command(original_command: &str) {
    if !needs_wrapping(original_command) {
        return;
    }

    if let Some(env_file) = find_valid_env_file_path() {
        let config = read_configs(env_file.parent().unwrap());

        let mut command = Command::new("op");
        command.arg("run").arg("--env-file").arg(env_file);

        if config.disable_masking.unwrap_or(false) {
            command.arg("--no-masking");
        }

        command.arg("--");

        for arg in build_safe_command_arguments(original_command) {
            command.arg(arg);
        }

        let status = command.status().expect("Failed to execute command");

        if !status.success() {
            error!("Command failed with status: {status}");
        }

        exit(status.code().unwrap_or(1));
    }
}

pub fn needs_wrapping(original_command: &str) -> bool {
    let trimmed_original = original_command.trim();

    if trimmed_original.is_empty() {
        error!("No command provided");
        return false;
    }

    if trimmed_original.contains("openv") {
        debug!("Command '{original_command}' is already wrapped");
        return false;
    }

    if let Some(env_file) = find_valid_env_file_path() {
        let config = read_configs(env_file.parent().unwrap());
        let is_allowed = config.allow.iter().any(|p| p.is_match(original_command));
        let is_denied = config.deny.iter().any(|p| p.is_match(original_command));

        if is_allowed && !is_denied {
            debug!("Command '{original_command}' needs wrapping");
        } else {
            debug!("Command '{original_command}' does not need wrapping");
        }

        is_allowed && !is_denied
    } else {
        false
    }
}

fn build_safe_command_arguments(original_command: &str) -> Vec<String> {
    match split(original_command) {
        Some(parts) if !parts.is_empty() => {
            let mut safe_args = Vec::new();
            for arg in parts {
                match try_quote(&arg) {
                    Ok(quoted) => safe_args.push(quoted.to_string()),
                    Err(e) => {
                        error!("Failed to quote argument '{arg}': {e}");
                        exit(1);
                    }
                }
            }
            safe_args
        }
        _ => {
            error!("Failed to parse command: {original_command}");
            exit(1);
        }
    }
}

fn read_configs(root_project_folder: &Path) -> Config {
    let global_config = Config::load(&dirs::home_dir().unwrap().join(".openv.toml"));
    let local_config = Config::load(&root_project_folder.join(".openv.toml"));
    Config::merge(&global_config, &local_config)
}

fn find_valid_env_file_path() -> Option<PathBuf> {
    let cwd = env::current_dir().ok()?;
    debug!("Current working directory: {cwd:?}");

    let root_project_folder =
        find_git_root_path(cwd.clone()).or_else(|| find_env_root_path(cwd.clone()))?;

    let env_file = root_project_folder.join(".env");

    if env_file.exists() && fs::read_to_string(&env_file).ok()?.contains("op://") {
        debug!("Valid .env file found: {env_file:?}");
        Some(env_file)
    } else {
        warn!("No valid .env file found");
        None
    }
}

fn find_git_root_path(mut dir: PathBuf) -> Option<PathBuf> {
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

fn find_env_root_path(mut dir: PathBuf) -> Option<PathBuf> {
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
