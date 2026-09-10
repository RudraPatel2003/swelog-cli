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
};

use crate::{
    errors::ConfigAlreadyExists,
    overwrite::Overwrite,
    swelog_config::SwelogConfig,
};

pub fn write_default_config(
    config_file_path: &PathBuf,
    config: &SwelogConfig,
    overwrite: Overwrite,
) -> Result<()> {
    if config_file_path.exists() && overwrite == Overwrite::No {
        let config_already_exists_error =
            ConfigAlreadyExists { config_file_path: config_file_path.clone() };

        return Err(config_already_exists_error.into());
    }

    create_config_directory(config_file_path)?;

    let serialized_config = serde_json::to_string_pretty(config)
        .into_diagnostic()
        .wrap_err("failed to serialize config")?;

    fs::write(config_file_path, format!("{serialized_config}\n")).into_diagnostic().wrap_err_with(
        || format!("failed to write config file at {}", path_link(config_file_path)),
    )?;

    Ok(())
}

fn create_config_directory(config_file_path: &Path) -> Result<()> {
    let Some(config_directory) = config_file_path.parent() else {
        return Ok(());
    };

    fs::create_dir_all(config_directory).into_diagnostic().wrap_err_with(|| {
        format!("failed to create config directory at {}", path_link(config_directory))
    })?;

    Ok(())
}

#[cfg(test)]
mod tests;
