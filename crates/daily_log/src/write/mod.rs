use std::{
    fs,
    path::Path,
};

use chrono::NaiveDate;
use config::{
    overwrite::Overwrite,
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
    swelog_file_existence::ensure_swelog_directory_exists,
};
use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use undo::snapshot::{
    UndoSnapshot,
    get_undo_snapshot_file_path,
    write_undo_snapshot,
};

use crate::{
    content::build_daily_log_content,
    file::resolve_daily_log_file,
    work_file::{
        KeepWorkFile,
        finish_work_file,
        read_work_file_notes,
    },
};

pub fn write_daily_log_from_config(
    swelog_config: &SwelogConfig,
    cache_directory: &Path,
    log_date: &NaiveDate,
    overwrite: Overwrite,
    keep_work_file: KeepWorkFile,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_directory_exists(&swelog_paths.daily_log_directory)?;

    let daily_log_file = resolve_daily_log_file(&swelog_paths, log_date, overwrite)?;

    let work_file_content = read_work_file_notes(&swelog_paths)?;

    let daily_log_content = build_daily_log_content(&work_file_content, log_date);

    fs::write(&daily_log_file, daily_log_content).into_diagnostic().wrap_err_with(|| {
        format!("failed to write daily log file at {}", path_link(&daily_log_file))
    })?;

    let undo_snapshot = UndoSnapshot { created_file: Some(daily_log_file), work_file_content };

    write_undo_snapshot(&get_undo_snapshot_file_path(cache_directory), &undo_snapshot)?;

    finish_work_file(swelog_config, cache_directory, keep_work_file)
}

#[cfg(test)]
mod tests;
