pub mod default_files;
pub mod swelog_paths;

use std::{
    fs,
    path::Path,
};

use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use swelog_paths::SwelogPaths;

use crate::{
    errors::SwelogFilesAlreadyExist,
    overwrite::Overwrite,
    swelog_config::SwelogConfig,
    work_file::create_or_reset_work_file,
};

pub fn setup_swelog_files_from_config(
    swelog_config: &SwelogConfig,
    cache_directory: &Path,
    overwrite: Overwrite,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    if overwrite == Overwrite::No {
        fail_if_swelog_paths_exist(&swelog_paths)?;
    }

    fs::create_dir_all(&swelog_paths.swelog_directory).into_diagnostic().wrap_err_with(|| {
        format!(
            "failed to create swelog directory at {}",
            path_link(&swelog_paths.swelog_directory)
        )
    })?;

    create_or_reset_work_file(swelog_config, cache_directory)?;

    fs::create_dir_all(&swelog_paths.daily_log_directory).into_diagnostic().wrap_err_with(
        || {
            format!(
                "failed to create daily log directory at {}",
                path_link(&swelog_paths.daily_log_directory)
            )
        },
    )?;

    fs::create_dir_all(&swelog_paths.weekly_log_directory).into_diagnostic().wrap_err_with(
        || {
            format!(
                "failed to create weekly log directory at {}",
                path_link(&swelog_paths.weekly_log_directory)
            )
        },
    )?;

    Ok(())
}

fn fail_if_swelog_paths_exist(swelog_paths: &SwelogPaths) -> Result<()> {
    for swelog_path in swelog_paths.all_paths() {
        if swelog_path.exists() {
            let swelog_files_already_exist_error =
                SwelogFilesAlreadyExist { swelog_path: swelog_path.clone() };

            return Err(swelog_files_already_exist_error.into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
