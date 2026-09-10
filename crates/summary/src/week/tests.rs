use std::{
    fs,
    path::PathBuf,
};

use async_trait::async_trait;
use chrono::{
    Duration,
    NaiveDate,
};
use config::{
    errors::SwelogFileNotFound,
    setup::{
        default_files::DEFAULT_WORK_FILE_CONTENT,
        swelog_paths::SwelogPaths,
    },
    swelog_config::SwelogConfig,
};
use llm::language_model::LanguageModel;
use miette::Result;
use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const CONTEXT_FILE_CONTENT: &str = "backend engineer on platform team";

const MONDAY_DAILY_LOG_CONTENT: &str = "# Daily Log - 06-01-2026\n\nDebugged API timeout";

const WEDNESDAY_DAILY_LOG_CONTENT: &str = "# Daily Log - 06-03-2026\n\nReviewed auth PR";

const FRIDAY_DAILY_LOG_CONTENT: &str = "# Daily Log - 06-05-2026\n\nPlanned release";

const EXISTING_WEEKLY_LOG_CONTENT: &str = "existing weekly log";

struct TestContext {
    temporary_directory: TempDir,
    config: SwelogConfig,
}

struct FakeLanguageModel;

#[async_trait]
impl LanguageModel for FakeLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        Ok(format!("generated from prompt:\n{prompt}"))
    }
}

impl TestContext {
    fn swelog_paths(&self) -> SwelogPaths {
        SwelogPaths::new(&self.config)
    }

    fn work_file(&self) -> PathBuf {
        self.swelog_paths().work_file
    }

    fn daily_log_directory(&self) -> PathBuf {
        self.swelog_paths().daily_log_directory
    }

    fn weekly_log_directory(&self) -> PathBuf {
        self.swelog_paths().weekly_log_directory
    }

    fn daily_log_file(&self, log_date: NaiveDate) -> PathBuf {
        get_daily_log_file_path(&self.swelog_paths(), &log_date)
    }

    fn weekly_log_file(&self) -> PathBuf {
        get_weekly_log_file_path(&self.swelog_paths(), &test_monday_date())
    }

    fn write_swelog_files(&self) {
        fs::create_dir_all(self.daily_log_directory())
            .expect("daily log directory should be created");

        fs::create_dir_all(self.weekly_log_directory())
            .expect("weekly log directory should be created");

        fs::write(self.work_file(), DEFAULT_WORK_FILE_CONTENT)
            .expect("work file should be written");
    }

    fn write_daily_log(&self, log_date: NaiveDate, content: &str) {
        fs::write(self.daily_log_file(log_date), content).expect("daily log should be written");
    }

    fn write_work_file(&self, content: &str) {
        fs::write(self.work_file(), content).expect("work file should be written");
    }
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let config = SwelogConfig {
        obsidian_vault_path: temporary_directory.path().to_path_buf(),
        ..SwelogConfig::get_default_config()
    };

    TestContext { temporary_directory, config }
}

fn test_monday_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 6, 1).expect("test date should be valid")
}

fn test_wednesday_date() -> NaiveDate {
    test_monday_date()
        .checked_add_signed(Duration::days(2))
        .expect("test wednesday date should be valid")
}

fn test_friday_date() -> NaiveDate {
    test_monday_date()
        .checked_add_signed(Duration::days(4))
        .expect("test friday date should be valid")
}

#[tokio::test]
async fn summarize_weekly_work_writes_generated_weekly_log() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    let wednesday_date = test_wednesday_date();

    test_context.write_swelog_files();

    test_context.write_daily_log(monday_date, MONDAY_DAILY_LOG_CONTENT);

    test_context.write_daily_log(wednesday_date, WEDNESDAY_DAILY_LOG_CONTENT);

    summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        Some(CONTEXT_FILE_CONTENT),
        Overwrite::No,
    )
    .await
    .expect("weekly log should be written");

    let weekly_log_content =
        fs::read_to_string(test_context.weekly_log_file()).expect("weekly log should be readable");

    assert!(weekly_log_content.starts_with("generated from prompt:\n"));

    assert!(weekly_log_content.contains(MONDAY_DAILY_LOG_CONTENT));

    assert!(weekly_log_content.contains(WEDNESDAY_DAILY_LOG_CONTENT));

    assert!(weekly_log_content.contains(CONTEXT_FILE_CONTENT));

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_weekly_work_skips_missing_weekday_logs() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    let friday_date = test_friday_date();

    test_context.write_swelog_files();

    test_context.write_daily_log(friday_date, FRIDAY_DAILY_LOG_CONTENT);

    summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        Some(CONTEXT_FILE_CONTENT),
        Overwrite::No,
    )
    .await
    .expect("weekly log should be written from available daily logs");

    let weekly_log_content =
        fs::read_to_string(test_context.weekly_log_file()).expect("weekly log should be readable");

    assert!(weekly_log_content.contains(FRIDAY_DAILY_LOG_CONTENT));

    assert!(!weekly_log_content.contains(MONDAY_DAILY_LOG_CONTENT));

    assert!(!weekly_log_content.contains(WEDNESDAY_DAILY_LOG_CONTENT));

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_weekly_work_fails_when_no_daily_logs_exist() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    test_context.write_swelog_files();

    let error = summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        Some(CONTEXT_FILE_CONTENT),
        Overwrite::No,
    )
    .await
    .expect_err("missing daily logs should fail");

    let error = error.downcast_ref::<NoDailyLogsFound>().expect("error should be NoDailyLogsFound");

    assert_eq!(error.monday_date, monday_date);

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_weekly_work_prompts_without_context_when_none_is_given() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    test_context.write_swelog_files();

    test_context.write_daily_log(monday_date, MONDAY_DAILY_LOG_CONTENT);

    summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        None,
        Overwrite::No,
    )
    .await
    .expect("weekly log should be written without context");

    let weekly_log_content =
        fs::read_to_string(test_context.weekly_log_file()).expect("weekly log should be readable");

    assert!(weekly_log_content.contains("no context given"));

    assert!(!weekly_log_content.contains(CONTEXT_FILE_CONTENT));

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_weekly_work_fails_when_work_file_is_missing() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    test_context.write_swelog_files();

    test_context.write_daily_log(monday_date, MONDAY_DAILY_LOG_CONTENT);

    fs::remove_file(test_context.work_file()).expect("work file should be removed");

    let error = summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        Some(CONTEXT_FILE_CONTENT),
        Overwrite::No,
    )
    .await
    .expect_err("missing work file should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.work_file());

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_weekly_work_fails_when_daily_log_directory_is_missing() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    test_context.write_swelog_files();

    fs::remove_dir(test_context.daily_log_directory())
        .expect("daily log directory should be removed");

    let error = summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        Some(CONTEXT_FILE_CONTENT),
        Overwrite::No,
    )
    .await
    .expect_err("missing daily log directory should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.daily_log_directory());

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_weekly_work_fails_when_weekly_log_directory_is_missing() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    test_context.write_swelog_files();

    test_context.write_daily_log(monday_date, MONDAY_DAILY_LOG_CONTENT);

    fs::remove_dir(test_context.weekly_log_directory())
        .expect("weekly log directory should be removed");

    let error = summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        Some(CONTEXT_FILE_CONTENT),
        Overwrite::No,
    )
    .await
    .expect_err("missing weekly log directory should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.weekly_log_directory());

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_weekly_work_fails_when_weekly_log_exists_without_force() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    test_context.write_swelog_files();

    test_context.write_daily_log(monday_date, MONDAY_DAILY_LOG_CONTENT);

    fs::write(test_context.weekly_log_file(), EXISTING_WEEKLY_LOG_CONTENT)
        .expect("existing weekly log should be written");

    let error = summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        Some(CONTEXT_FILE_CONTENT),
        Overwrite::No,
    )
    .await
    .expect_err("existing weekly log should fail without force");

    let error = error
        .downcast_ref::<WeeklyLogAlreadyExists>()
        .expect("error should be WeeklyLogAlreadyExists");

    assert_eq!(error.weekly_log_file, test_context.weekly_log_file());

    let weekly_log_content =
        fs::read_to_string(test_context.weekly_log_file()).expect("weekly log should be readable");

    assert_eq!(weekly_log_content, EXISTING_WEEKLY_LOG_CONTENT);

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_weekly_work_overwrites_existing_weekly_log_with_force() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    test_context.write_swelog_files();

    test_context.write_daily_log(monday_date, MONDAY_DAILY_LOG_CONTENT);

    fs::write(test_context.weekly_log_file(), EXISTING_WEEKLY_LOG_CONTENT)
        .expect("existing weekly log should be written");

    summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        Some(CONTEXT_FILE_CONTENT),
        Overwrite::Yes,
    )
    .await
    .expect("existing weekly log should be overwritten with force");

    let weekly_log_content =
        fs::read_to_string(test_context.weekly_log_file()).expect("weekly log should be readable");

    assert_ne!(weekly_log_content, EXISTING_WEEKLY_LOG_CONTENT);

    assert!(weekly_log_content.contains(MONDAY_DAILY_LOG_CONTENT));

    drop(test_context.temporary_directory);
}

const UNSUMMARIZED_WORK_FILE_CONTENT: &str = r"# Today's Work

## Log
- Still needs a daily summary
";

#[tokio::test]
async fn summarize_weekly_work_fails_when_work_file_is_not_default() {
    let test_context = get_test_context();

    let monday_date = test_monday_date();

    test_context.write_swelog_files();

    test_context.write_daily_log(monday_date, MONDAY_DAILY_LOG_CONTENT);

    test_context.write_work_file(UNSUMMARIZED_WORK_FILE_CONTENT);

    let error = summarize_weekly_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &monday_date,
        Some(CONTEXT_FILE_CONTENT),
        Overwrite::No,
    )
    .await
    .expect_err("unsummarized work file should fail");

    error.downcast_ref::<WorkFileNotDefault>().expect("error should be WorkFileNotDefault");

    assert!(!test_context.weekly_log_file().exists());

    drop(test_context.temporary_directory);
}
