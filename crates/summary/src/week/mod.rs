mod errors;

use std::{
    fs,
    path::PathBuf,
};

use chrono::{
    Duration,
    NaiveDate,
};
use config::{
    overwrite::Overwrite,
    setup::{
        default_files::is_default_work_file_content,
        swelog_paths::SwelogPaths,
    },
    swelog_config::SwelogConfig,
    swelog_file_existence::{
        ensure_swelog_directory_exists,
        ensure_swelog_file_exists,
    },
};
use daily_log::file::get_daily_log_file_path;
use dates::formatting::format_date;
use errors::{
    NoDailyLogsFound,
    WeekdayDateOutOfRange,
    WeeklyLogAlreadyExists,
    WorkFileNotDefault,
};
use highlight::stderr::path_link;
use llm::{
    language_model::LanguageModel,
    prompts::get_weekly_log_prompt,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

pub async fn summarize_weekly_work_from_config(
    swelog_config: &SwelogConfig,
    language_model: &dyn LanguageModel,
    monday_date: &NaiveDate,
    context_file_content: Option<&str>,
    overwrite: Overwrite,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_file_exists(&swelog_paths.work_file)?;

    ensure_swelog_directory_exists(&swelog_paths.daily_log_directory)?;

    ensure_swelog_directory_exists(&swelog_paths.weekly_log_directory)?;

    let weekly_log_file = get_weekly_log_file_path(&swelog_paths, monday_date);

    if weekly_log_file.exists() && overwrite == Overwrite::No {
        let weekly_log_already_exists_error = WeeklyLogAlreadyExists { weekly_log_file };

        return Err(weekly_log_already_exists_error.into());
    }

    let work_file_content =
        fs::read_to_string(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", path_link(&swelog_paths.work_file))
        })?;

    if !is_default_work_file_content(&work_file_content) {
        let work_file_not_default_error = WorkFileNotDefault;

        return Err(work_file_not_default_error.into());
    }

    let daily_logs = collect_weekday_daily_logs(&swelog_paths, *monday_date)?;

    if daily_logs.is_empty() {
        let no_daily_logs_found_error = NoDailyLogsFound { monday_date: *monday_date };

        return Err(no_daily_logs_found_error.into());
    }

    let prompt = get_weekly_log_prompt(&daily_logs, context_file_content, monday_date);

    let generated_weekly_log_content = language_model.generate_response(&prompt).await?;

    fs::write(&weekly_log_file, generated_weekly_log_content).into_diagnostic().wrap_err_with(
        || format!("failed to write weekly log file at {}", path_link(&weekly_log_file)),
    )?;

    Ok(())
}

fn get_weekly_log_file_name(monday_date: NaiveDate) -> String {
    let monday_date_string = format_date(&monday_date);

    format!("Week of {monday_date_string}.md")
}

#[must_use]
pub fn get_weekly_log_file_path(swelog_paths: &SwelogPaths, monday_date: &NaiveDate) -> PathBuf {
    let weekly_log_file_name = get_weekly_log_file_name(*monday_date);

    swelog_paths.weekly_log_directory.join(weekly_log_file_name)
}

fn collect_weekday_daily_logs(
    swelog_paths: &SwelogPaths,
    monday_date: NaiveDate,
) -> Result<Vec<String>> {
    let mut daily_logs = Vec::new();

    for day_offset in 0..5 {
        let daily_log_date = monday_date
            .checked_add_signed(Duration::days(day_offset))
            .ok_or(WeekdayDateOutOfRange { monday_date })?;

        let daily_log_file = get_daily_log_file_path(swelog_paths, &daily_log_date);

        if !daily_log_file.exists() {
            continue;
        }

        let daily_log_content =
            fs::read_to_string(&daily_log_file).into_diagnostic().wrap_err_with(|| {
                format!("failed to read daily log file at {}", path_link(&daily_log_file))
            })?;

        daily_logs.push(daily_log_content);
    }

    Ok(daily_logs)
}

#[cfg(test)]
mod tests;
