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

const HIDE_COMMENTS_FLAG_FILE_NAME: &str = "hide-comments";

pub fn set_hide_comments_flag(cache_directory: &Path) -> Result<()> {
    let flag_file_path = get_hide_comments_flag_file_path(cache_directory);

    if has_hide_comments_flag(cache_directory) {
        return Ok(());
    }

    if let Some(parent) = flag_file_path.parent() {
        fs::create_dir_all(parent).into_diagnostic().wrap_err_with(|| {
            format!("failed to create the swelog cache directory at {}", path_link(parent))
        })?;
    }

    fs::write(&flag_file_path, "").into_diagnostic().wrap_err_with(|| {
        format!("failed to write the hide comments flag at {}", path_link(&flag_file_path))
    })
}

#[must_use]
pub fn has_hide_comments_flag(cache_directory: &Path) -> bool {
    get_hide_comments_flag_file_path(cache_directory).is_file()
}

fn get_hide_comments_flag_file_path(cache_directory: &Path) -> PathBuf {
    cache_directory.join(HIDE_COMMENTS_FLAG_FILE_NAME)
}

#[cfg(test)]
mod tests;
