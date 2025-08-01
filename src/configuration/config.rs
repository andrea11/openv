use log::{debug, error};
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default, deserialize_with = "deserialize_vec_regex")]
    pub allow_commands: Vec<Regex>,
    #[serde(default, deserialize_with = "deserialize_vec_regex")]
    pub deny_commands: Vec<Regex>,
    #[serde(default)]
    pub disable_masking: Option<bool>,
}

fn deserialize_vec_regex<'de, D>(deserializer: D) -> Result<Vec<Regex>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec = Vec::<String>::deserialize(deserializer)?;
    vec.into_iter()
        .map(|s| Regex::new(&s).map_err(serde::de::Error::custom))
        .collect()
}

impl Config {
    pub fn load(path: &Path) -> Option<Config> {
        let content = fs::read_to_string(path).ok()?;
        match toml::from_str::<Config>(&content) {
            Ok(cfg) => {
                debug!("Config loaded from {path:?}: {cfg:?}");
                Some(cfg)
            }
            Err(e) => {
                error!("Failed to parse TOML config {path:?}: {e}");
                None
            }
        }
    }

    pub fn load_default_config() -> Self {
        let toml_str = include_str!("default_config.toml");
        let config = toml::from_str(toml_str).expect("Invalid default_config.toml");
        debug!("Default configuration loaded: {config:?}");
        config
    }

    pub fn merge(global_config: &Config, local_config: &Option<Config>) -> Config {
        let local = match local_config {
            Some(c) => c,
            _ => return global_config.clone(),
        };

        let pick = |local: &Vec<Regex>, global: &Vec<Regex>| {
            if local.is_empty() {
                global.clone()
            } else {
                local.clone()
            }
        };

        let allow_commands = pick(&local.allow_commands, &global_config.allow_commands);
        let deny_commands = pick(&local.deny_commands, &global_config.deny_commands);
        let disable_masking = local.disable_masking.or(global_config.disable_masking);

        let cfg = Config {
            allow_commands,
            deny_commands,
            disable_masking,
        };

        debug!("Merged configuration: {cfg:?}");
        cfg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn validate_default_config() {
        let config_str = include_str!("default_config.toml");

        toml::from_str::<Config>(config_str).expect("❌ default_config.toml is invalid TOML");
    }
}
