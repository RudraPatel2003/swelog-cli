pub mod hide_comments;

use std::{
    fs,
    path::Path,
};

use hide_comments::has_hide_comments_flag;
use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::{
    setup::{
        default_files::{
            DEFAULT_WORK_FILE_CONTENT,
            DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS,
        },
        swelog_paths::SwelogPaths,
    },
    swelog_config::SwelogConfig,
    swelog_file_existence::ensure_swelog_file_exists,
};

pub fn read_work_file(swelog_paths: &SwelogPaths) -> Result<String> {
    ensure_swelog_file_exists(&swelog_paths.work_file)?;

    fs::read_to_string(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
        format!("failed to read work file at {}", path_link(&swelog_paths.work_file))
    })
}

pub fn create_or_reset_work_file(
    swelog_config: &SwelogConfig,
    cache_directory: &Path,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    let default_work_file_content = get_default_work_file_content(cache_directory);

    fs::write(&swelog_paths.work_file, default_work_file_content).into_diagnostic().wrap_err_with(
        || format!("failed to write work file at {}", path_link(&swelog_paths.work_file)),
    )
}

fn get_default_work_file_content(cache_directory: &Path) -> &'static str {
    if has_hide_comments_flag(cache_directory) {
        DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS
    } else {
        DEFAULT_WORK_FILE_CONTENT
    }
}

#[cfg(test)]
mod tests;
