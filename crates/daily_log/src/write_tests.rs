use std::path::PathBuf;

use config::{
    errors::SwelogFileNotFound,
    setup::default_files::{
        DEFAULT_WORK_FILE_CONTENT,
        is_default_work_file_content,
    },
};
use tempfile::{
    TempDir,
    tempdir,
};
use undo::snapshot::{
    get_undo_snapshot_file_path,
    read_undo_snapshot,
};

use super::*;
use crate::{
    errors::{
        DailyLogAlreadyExists,
        WorkFileNotUpdated,
    },
    file::get_daily_log_file_path,
};

const WORK_FILE_CONTENT: &str = r"# Today's Work

## Focus
- Debug API timeout

## Log
- Reviewed auth PR
";

const EXPECTED_DAILY_LOG_CONTENT: &str = r"# Daily Log - 05-23-2026

## Focus
- Debug API timeout

## Log
- Reviewed auth PR
";

const EXISTING_DAILY_LOG_CONTENT: &str = "existing daily log";

struct TestContext {
    temporary_directory: TempDir,
    config: SwelogConfig,
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

    fn daily_log_file(&self) -> PathBuf {
        let log_date = test_log_date();

        get_daily_log_file_path(&self.swelog_paths(), &log_date)
    }

    fn cache_directory(&self) -> PathBuf {
        self.temporary_directory.path().join("cache")
    }

    fn undo_snapshot_file(&self) -> PathBuf {
        get_undo_snapshot_file_path(&self.cache_directory())
    }

    fn write_swelog_files(&self) {
        fs::create_dir_all(self.daily_log_directory())
            .expect("daily log directory should be created");

        fs::write(self.work_file(), WORK_FILE_CONTENT).expect("work file should be written");
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

fn test_log_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 5, 23).expect("test date should be valid")
}

#[test]
fn write_daily_log_writes_the_work_file_into_the_daily_log_directory() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let log_date = test_log_date();

    write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::No,
    )
    .expect("daily log should be written");

    let daily_log_content =
        fs::read_to_string(test_context.daily_log_file()).expect("daily log should be readable");

    assert_eq!(daily_log_content, EXPECTED_DAILY_LOG_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_does_not_require_a_context_file() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    assert!(!test_context.swelog_paths().context_file.exists());

    let log_date = test_log_date();

    write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::No,
    )
    .expect("daily log should be written without a context file");

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_fails_when_work_file_is_not_updated() {
    let test_context = get_test_context();

    let log_date = test_log_date();

    test_context.write_swelog_files();

    fs::write(test_context.work_file(), DEFAULT_WORK_FILE_CONTENT)
        .expect("default work file should be written");

    let error = write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::No,
    )
    .expect_err("default work file should fail");

    error.downcast_ref::<WorkFileNotUpdated>().expect("error should be WorkFileNotUpdated");

    assert!(!test_context.daily_log_file().exists());

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_fails_when_work_file_is_missing() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let log_date = test_log_date();

    fs::remove_file(test_context.work_file()).expect("work file should be removed");

    let error = write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::No,
    )
    .expect_err("missing work file should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.work_file());

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_fails_when_daily_log_directory_is_missing() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let log_date = test_log_date();

    fs::remove_dir_all(test_context.daily_log_directory())
        .expect("daily log directory should be removed");

    let error = write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::No,
    )
    .expect_err("missing daily log directory should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.daily_log_directory());

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_fails_when_daily_log_exists_without_force() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let log_date = test_log_date();

    fs::write(test_context.daily_log_file(), EXISTING_DAILY_LOG_CONTENT)
        .expect("existing daily log should be written");

    let error = write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::No,
    )
    .expect_err("existing daily log should fail without force");

    let error = error
        .downcast_ref::<DailyLogAlreadyExists>()
        .expect("error should be DailyLogAlreadyExists");

    assert_eq!(error.daily_log_file, test_context.daily_log_file());

    let daily_log_content =
        fs::read_to_string(test_context.daily_log_file()).expect("daily log should be readable");

    assert_eq!(daily_log_content, EXISTING_DAILY_LOG_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_overwrites_existing_daily_log_with_force() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let log_date = test_log_date();

    fs::write(test_context.daily_log_file(), EXISTING_DAILY_LOG_CONTENT)
        .expect("existing daily log should be written");

    write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::Yes,
        KeepWorkFile::No,
    )
    .expect("existing daily log should be overwritten with force");

    let daily_log_content =
        fs::read_to_string(test_context.daily_log_file()).expect("daily log should be readable");

    assert_eq!(daily_log_content, EXPECTED_DAILY_LOG_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_resets_work_file_by_default() {
    let test_context = get_test_context();

    let log_date = test_log_date();

    test_context.write_swelog_files();

    write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::No,
    )
    .expect("daily log should be written");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert!(is_default_work_file_content(&work_file_content));

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_keeps_work_file_when_keep_is_set() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let log_date = test_log_date();

    write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::Yes,
    )
    .expect("daily log should be written");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_content, WORK_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_saves_an_undo_snapshot_of_the_work_file() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let log_date = test_log_date();

    write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::No,
    )
    .expect("daily log should be written");

    let undo_snapshot = read_undo_snapshot(&test_context.undo_snapshot_file())
        .expect("undo snapshot should be read");

    assert_eq!(undo_snapshot.created_file, Some(test_context.daily_log_file()));

    assert_eq!(undo_snapshot.work_file_content, WORK_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_saves_an_undo_snapshot_when_the_work_file_is_kept() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let log_date = test_log_date();

    write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::Yes,
    )
    .expect("daily log should be written");

    let undo_snapshot = read_undo_snapshot(&test_context.undo_snapshot_file())
        .expect("undo snapshot should be read");

    assert_eq!(undo_snapshot.created_file, Some(test_context.daily_log_file()));

    drop(test_context.temporary_directory);
}

#[test]
fn write_daily_log_does_not_save_an_undo_snapshot_when_the_work_file_is_not_updated() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::write(test_context.work_file(), DEFAULT_WORK_FILE_CONTENT)
        .expect("default work file should be written");

    let log_date = test_log_date();

    write_daily_log_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        &log_date,
        Overwrite::No,
        KeepWorkFile::No,
    )
    .expect_err("default work file should fail");

    assert!(!test_context.undo_snapshot_file().exists());

    drop(test_context.temporary_directory);
}
