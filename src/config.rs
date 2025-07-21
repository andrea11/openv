use log::{debug, error, info};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;
use toml::Value;

#[derive(Debug)]
pub struct Config {
    pub allow: Vec<Regex>,
    pub deny: Vec<Regex>,
    pub disable_masking: Option<bool>,
}

static DEFAULT_ALLOW_LIST: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        // JavaScript / Node.js
        Regex::new(r"^npm (run )?(start|dev|build)").unwrap(),
        Regex::new(r"^yarn (run )?(start|dev|build)").unwrap(),
        Regex::new(r"^pnpm (run )?(start|dev|build)").unwrap(),
        Regex::new(r"^bun (run )?(start|dev|build)").unwrap(),
    ]
});

impl Config {
    pub fn load(path: &PathBuf) -> Self {
        let mut allow = vec![];
        let mut deny = vec![];
        let mut disable_masking = None;

        info!("Loading configuration from: {path:?}");

        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(value) = content.parse::<Value>() {
                if let Some(arr) = value.get("allowList").and_then(|v| v.as_array()) {
                    allow = arr
                        .iter()
                        .filter_map(|v| v.as_str())
                        .filter_map(|s| match Regex::new(s) {
                            Ok(r) => Some(r),
                            Err(e) => {
                                error!("Invalid regex in allowList: {s}, error: {e}");
                                None
                            }
                        })
                        .collect();
                    debug!("Loaded allow list: {allow:?}");
                }
                if let Some(arr) = value.get("denyList").and_then(|v| v.as_array()) {
                    deny = arr
                        .iter()
                        .filter_map(|v| v.as_str())
                        .filter_map(|s| match Regex::new(s) {
                            Ok(r) => Some(r),
                            Err(e) => {
                                error!("Invalid regex in denyList: {s}, error: {e}");
                                None
                            }
                        })
                        .collect();
                    debug!("Loaded deny list: {deny:?}");
                }
                if let Some(disable) = value.get("disableMasking").and_then(|v| v.as_bool()) {
                    disable_masking = Some(disable);
                    debug!("Loaded disable masking: {disable:?}");
                }
            } else {
                error!("Failed to parse TOML content from: {path:?}");
            }
        } else {
            debug!("No configuration file: {path:?}");
        }

        if allow.is_empty() {
            allow = DEFAULT_ALLOW_LIST.clone();
            info!("Using default allow list: {allow:?}");
        }

        Config {
            allow,
            deny,
            disable_masking,
        }
    }

    pub fn merge(config1: &Config, config2: &Config) -> Config {
        let allow = if config2.allow.is_empty() {
            config1.allow.clone()
        } else {
            config2.allow.clone()
        };

        let deny = if config2.deny.is_empty() {
            config1.deny.clone()
        } else {
            config2.deny.clone()
        };

        let disable_masking = config2.disable_masking.or(config1.disable_masking);

        debug!(
            "Merged configuration: allow: {allow:?}, deny: {deny:?}, disable_masking: {disable_masking:?}"
        );

        Config {
            allow,
            deny,
            disable_masking,
        }
    }
}
