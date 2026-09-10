use std::path::PathBuf;

use highlight::stderr::path_link;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("daily log already exists at {}", path_link(.daily_log_file))]
#[diagnostic(
    code(swelog::daily_log::daily_log_already_exists),
    help("re-run with `--force` to overwrite the existing daily log file")
)]
pub struct DailyLogAlreadyExists {
    pub daily_log_file: PathBuf,
}

#[derive(Debug, Diagnostic, Error)]
#[error("work file not updated")]
#[diagnostic(
    code(swelog::daily_log::work_file_not_updated),
    help("add your work notes to the work file before running `swelog log` or `swelog summarize`")
)]
pub struct WorkFileNotUpdated;
