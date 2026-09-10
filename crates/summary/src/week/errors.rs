use std::path::PathBuf;

use chrono::NaiveDate;
use highlight::stderr::path_link;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("weekly log already exists at {}", path_link(.weekly_log_file))]
#[diagnostic(
    code(swelog::summary::weekly_log_already_exists),
    help("use `swelog summarize week --force` to overwrite the existing weekly log file")
)]
pub struct WeeklyLogAlreadyExists {
    pub weekly_log_file: PathBuf,
}

#[derive(Debug, Diagnostic, Error)]
#[error("work file contains unsummarized work")]
#[diagnostic(
    code(swelog::summary::work_file_not_default),
    help(
        "run `swelog log` to file the current work file into a daily log, or `swelog reset` to \
         discard it"
    )
)]
pub struct WorkFileNotDefault;

#[derive(Debug, Diagnostic, Error)]
#[error("no daily logs found for week of {monday_date}")]
#[diagnostic(
    code(swelog::summary::no_daily_logs_found),
    help(
        "run `swelog log` or `swelog summarize day` for at least one weekday of that week, or \
         use `swelog summarize week --last-week` to summarize the previous week"
    )
)]
pub struct NoDailyLogsFound {
    pub monday_date: NaiveDate,
}

#[derive(Debug, Diagnostic, Error)]
#[error("week of {monday_date} extends past the supported date range")]
#[diagnostic(
    code(swelog::summary::weekday_date_out_of_range),
    help("use a Monday date within the supported calendar range")
)]
pub struct WeekdayDateOutOfRange {
    pub monday_date: NaiveDate,
}
