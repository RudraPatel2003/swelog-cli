use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};

use crate::{
    errors::{
        ConfigNotFound,
        EmptyObsidianVaultPath,
    },
    swelog_config::SwelogConfig,
};

const APP_NAME: &str = "swelog";

const CONFIG_FILE_NAME: &str = "swelog.json";

pub fn get_default_config_file_path() -> Result<PathBuf> {
    let config_directory =
        dirs::config_dir().ok_or_else(|| miette!("unable to determine config directory"))?;

    let config_file_path = config_directory.join(APP_NAME).join(CONFIG_FILE_NAME);

    Ok(config_file_path)
}

pub fn read_config_file(config_file_path: &Path) -> Result<SwelogConfig> {
    if !config_file_path.exists() {
        let config_not_found_error =
            ConfigNotFound { config_file_path: config_file_path.to_path_buf() };

        return Err(config_not_found_error.into());
    }

    let config_file_contents =
        fs::read_to_string(config_file_path).into_diagnostic().wrap_err_with(|| {
            format!("failed to read config file at {}", path_link(config_file_path))
        })?;

    let config: SwelogConfig = serde_json::from_str(&config_file_contents)
        .into_diagnostic()
        .wrap_err("failed to parse config")?;

    if config.obsidian_vault_path.as_os_str().is_empty() {
        let empty_obsidian_vault_path_error =
            EmptyObsidianVaultPath { config_file_path: config_file_path.to_path_buf() };

        return Err(empty_obsidian_vault_path_error.into());
    }

    Ok(config)
}
