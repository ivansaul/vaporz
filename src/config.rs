use crate::models::TargetInfo;
use color_eyre::Result;
use serde::Deserialize;
use std::{fs, path::PathBuf};

static DEFAULT_CONFIG: &str = include_str!("../config.toml");

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub targets: Vec<TargetInfo>,
}

pub fn load_config() -> Result<Config> {
    let mut config: Config = toml::from_str(DEFAULT_CONFIG)?;

    let app_name = env!("CARGO_PKG_NAME");

    let config_path: Option<PathBuf> = if cfg!(target_os = "macos") {
        dirs::home_dir().map(|home| home.join(format!(".config/{}/config.toml", app_name)))
    } else {
        dirs::config_dir().map(|dir| dir.join(format!("{}/config.toml", app_name)))
    };

    if let Some(config_path) = config_path
        && config_path.exists()
    {
        let content = fs::read_to_string(config_path)?;
        let user_config: Config = toml::from_str(&content)?;
        config = user_config;
    }

    Ok(config)
}
