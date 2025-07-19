use crate::config::Config;
use log::{debug, error, info, warn};
use shlex::try_quote;
use std::{env, fs, path::Path, path::PathBuf};

pub fn wrap_command(original_command: &str) -> String {
    let trimmed_original = original_command.trim();

    if trimmed_original.is_empty() {
        error!("No command provided");
        return String::new();
    }

    let env_file = match find_valid_env_file_path() {
        Some(path) => path,
        _ => {
            warn!("No valid .env file found");
            return original_command.to_string();
        }
    };

    let config = read_configs(env_file.parent().unwrap());

    if !config.needs_wrapping(trimmed_original) {
        return original_command.to_string();
    }

    match build_safe_shell_string(trimmed_original, &config, &env_file) {
        Ok(cmd) => {
            info!("Wrapped shell command: {cmd}");
            cmd
        }
        Err(e) => {
            error!("Failed to quote wrapped command: {e}");
            original_command.to_string()
        }
    }
}

fn build_safe_shell_string(
    original_command: &str,
    config: &Config,
    env_file: &Path,
) -> Result<String, shlex::QuoteError> {
    let mut parts = vec![
        try_quote("op")?.to_string(),
        try_quote("run")?.to_string(),
        try_quote("--env-file")?.to_string(),
        try_quote(env_file.to_str().unwrap_or_default())?.to_string(),
    ];

    if config.disable_masking.unwrap_or(false) {
        parts.push(try_quote("--no-masking")?.to_string());
    }

    parts.push(try_quote("--")?.to_string());
    parts.push(try_quote(original_command)?.to_string());

    Ok(parts.join(" "))
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
