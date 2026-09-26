use std::path::PathBuf;

use chrono::NaiveDate;
use config::{
    overwrite::Overwrite,
    setup::swelog_paths::SwelogPaths,
};
use dates::formatting::format_date;
use miette::Result;

use crate::errors::DailyLogAlreadyExists;

#[must_use]
pub fn get_daily_log_file_path(swelog_paths: &SwelogPaths, log_date: &NaiveDate) -> PathBuf {
    let daily_log_file_name = get_daily_log_file_name(*log_date);

    swelog_paths.daily_log_directory.join(daily_log_file_name)
}

pub fn resolve_daily_log_file(
    swelog_paths: &SwelogPaths,
    log_date: &NaiveDate,
    overwrite: Overwrite,
) -> Result<PathBuf> {
    let daily_log_file = get_daily_log_file_path(swelog_paths, log_date);

    if daily_log_file.exists() && overwrite == Overwrite::No {
        let daily_log_already_exists_error = DailyLogAlreadyExists { daily_log_file };

        return Err(daily_log_already_exists_error.into());
    }

    Ok(daily_log_file)
}

fn get_daily_log_file_name(log_date: NaiveDate) -> String {
    let formatted_date = format_date(&log_date);

    format!("{formatted_date}.md")
}
