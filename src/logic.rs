use crate::configuration::Config;
use log::{debug, error, info, warn};
use shlex::{split, try_quote};
use std::{env, fs, path::Path, path::PathBuf, process::Command};

pub fn execute_command(original_command: &str) -> Result<i32, String> {
    let Some(env_file) = find_valid_env_file_path() else {
        return Err("No valid .env file found".into());
    };

    let config = read_configs(env_file.parent().unwrap());

    let mut command = Command::new("op");
    command.arg("run").arg("--env-file").arg(env_file);

    if config.disable_masking.unwrap_or(false) {
        command.arg("--no-masking");
    }

    command.arg("--");

    let safe_args = build_safe_command_arguments(original_command)
        .map_err(|e| format!("Failed to build command arguments: {e}"))?;

    for arg in safe_args {
        command.arg(arg);
    }

    let status = command
        .status()
        .map_err(|e| format!("Failed to execute command: {e}"))?;

    if !status.success() {
        error!("Command failed with status: {status}");
    }

    Ok(status.code().unwrap_or(1))
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
        let is_allowed = config
            .allow_commands
            .iter()
            .any(|p| p.is_match(original_command));
        let is_denied = config
            .deny_commands
            .iter()
            .any(|p| p.is_match(original_command));

        let needs_wrapping = is_allowed && !is_denied;
        debug!(
            "Command '{original_command}' {} wrapping",
            if needs_wrapping {
                "needs"
            } else {
                "does not need"
            }
        );

        return needs_wrapping;
    }

    debug!("No valid .env file found");
    false
}

fn build_safe_command_arguments(original_command: &str) -> Result<Vec<String>, String> {
    match split(original_command) {
        Some(parts) if !parts.is_empty() => {
            let mut safe_args = Vec::new();
            for arg in parts {
                match try_quote(&arg) {
                    Ok(quoted) => safe_args.push(quoted.to_string()),
                    Err(e) => return Err(format!("Failed to quote argument '{arg}': {e}")),
                }
            }
            Ok(safe_args)
        }
        _ => Err(format!("Failed to parse command: {original_command}")),
    }
}

fn read_configs(root_project_folder: &Path) -> Config {
    const CONFIG_FILE: &str = ".openv.toml";

    debug!("Loading global configuration...");
    let global_config = dirs::home_dir()
        .and_then(|home| Config::load(&home.join(CONFIG_FILE)))
        .unwrap_or_else(|| {
            info!("Global config not found. Using default config.");
            Config::load_default_config()
        });

    debug!("Global configuration loaded: {global_config:?}");

    debug!("Loading local configuration...");
    let local_config = Config::load(&root_project_folder.join(CONFIG_FILE));
    debug!("Local configuration loaded: {local_config:?}");

    Config::merge(&global_config, &local_config)
}

fn find_valid_env_file_path() -> Option<PathBuf> {
    let cwd = env::current_dir().ok()?;
    debug!("Current working directory: {cwd:?}");

    let root_project_folder =
        find_root_with_marker(&cwd, ".git").or_else(|| find_root_with_marker(&cwd, ".env"))?;

    let env_file = root_project_folder.join(".env");

    if env_file.exists() && fs::read_to_string(&env_file).ok()?.contains("op://") {
        debug!("Valid .env file found: {env_file:?}");
        Some(env_file)
    } else {
        warn!("No valid .env file found");
        None
    }
}

fn find_root_with_marker(start: &Path, marker: &str) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        if ancestor.join(marker).exists() {
            debug!("Found {marker} in {ancestor:?}");
            return Some(ancestor.to_path_buf());
        }
    }
    warn!("No {marker} root found starting from {start:?}");
    None
}

#[cfg(test)]
mod tests {
    use crate::logic::{execute_command, needs_wrapping};
    use std::env;
    use std::fs::{create_dir_all, write};
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    fn create_test_environment() -> (std::path::PathBuf, std::path::PathBuf) {
        let test_dir = tempdir().unwrap().path().to_path_buf();

        // Create the test directory structure
        create_dir_all(&test_dir).unwrap();
        // Create a fake .env file
        write(test_dir.join(".env"), "SECRET=op://project/item/field").unwrap();

        // Create a matching config file that allows the command
        write(
            test_dir.join(".openv.toml"),
            r#"
                allow_commands = ["npm run dev"]
                deny_commands = []
            "#,
        )
        .unwrap();

        // Temporarily change working directory to our test root
        let original_dir = env::current_dir().unwrap();

        // Return both the original directory and the test directory
        (original_dir, test_dir)
    }

    fn mock_op_binary(test_dir: &std::path::Path) -> (String, String) {
        let bin_dir = test_dir.join("bin");
        std::fs::create_dir_all(&bin_dir).unwrap();
        let fake_op = bin_dir.join("op");
        std::fs::write(&fake_op, "#!/bin/sh\nexit 0").unwrap();
        std::fs::set_permissions(&fake_op, std::fs::Permissions::from_mode(0o755)).unwrap();

        let original_path = env::var("PATH").unwrap();
        let test_path = format!("{}:{}", bin_dir.display(), original_path);

        (original_path, test_path)
    }

    fn setup_test_environment(test_dir: std::path::PathBuf, test_path: String) {
        env::set_current_dir(test_dir).unwrap();
        env::set_var("PATH", test_path);
    }

    fn cleanup_test_environment(original_dir: std::path::PathBuf, original_path: String) {
        env::set_current_dir(original_dir).unwrap();
        env::set_var("PATH", original_path);
    }

    fn run_test_with_mocked_environment<T>(test: T)
    where
        T: FnOnce() + std::panic::UnwindSafe,
    {
        let (original_dir, test_dir) = create_test_environment();
        let (original_path, test_path) = mock_op_binary(&test_dir);

        setup_test_environment(test_dir, test_path);

        let result = std::panic::catch_unwind(test);

        cleanup_test_environment(original_dir, original_path);
        assert!(result.is_ok())
    }

    #[test]
    fn test_needs_wrapping_when_empty() {
        assert!(!needs_wrapping(""));
    }

    #[test]
    fn test_needs_wrapping_when_already_wrapped() {
        assert!(!needs_wrapping("openv execute npm run dev"));
    }

    #[test]
    fn test_needs_wrapping_when_no_env() {
        assert!(!needs_wrapping("echo 'hello'"));
    }

    #[test]
    fn test_needs_wrapping() {
        run_test_with_mocked_environment(|| {
            assert!(needs_wrapping("npm run dev"));
        });
    }

    #[test]
    fn test_execute_when_no_env_file() {
        assert_eq!(execute_command(""), Err("No valid .env file found".into()));
    }

    // TODO: Fix this test
    // #[test]
    // fn test_execute_command_with_env_file() {
    //     run_test_with_mocked_environment(|| {
    //         assert_eq!(execute_command("npm run dev"), Ok(0));
    //     });
    // }
}
