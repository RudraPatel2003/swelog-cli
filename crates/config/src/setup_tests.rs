use std::{
    fs,
    path::PathBuf,
};

use tempfile::{
    TempDir,
    tempdir,
};

use super::*;
use crate::{
    overwrite::Overwrite,
    setup::default_files::is_default_work_file_content,
};

const EXISTING_CONTEXT_FILE_CONTENT: &str = "existing context";

const EXISTING_WORK_FILE_CONTENT: &str = "existing work";

struct TestContext {
    temporary_directory: TempDir,
    config: SwelogConfig,
}

impl TestContext {
    fn swelog_paths(&self) -> SwelogPaths {
        SwelogPaths::new(&self.config)
    }

    fn cache_directory(&self) -> PathBuf {
        self.temporary_directory.path().join("cache")
    }

    fn swelog_directory(&self) -> PathBuf {
        self.swelog_paths().swelog_directory
    }

    fn context_file(&self) -> PathBuf {
        self.swelog_paths().context_file
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
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let config = SwelogConfig {
        obsidian_vault_path: temporary_directory.path().to_path_buf(),
        ..SwelogConfig::get_default_config()
    };

    TestContext { temporary_directory, config }
}

#[test]
fn setup_creates_swelog_files_and_directories() {
    let test_context = get_test_context();

    let overwrite_existing_files = Overwrite::No;

    setup_swelog_files_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        overwrite_existing_files,
    )
    .expect("swelog files should be created");

    let work_file_contents =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert!(is_default_work_file_content(&work_file_contents));

    assert!(!test_context.context_file().exists());

    assert!(test_context.daily_log_directory().is_dir());

    assert!(test_context.weekly_log_directory().is_dir());

    drop(test_context.temporary_directory);
}

#[test]
fn setup_uses_configured_path_names() {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let config = SwelogConfig {
        obsidian_vault_path: temporary_directory.path().to_path_buf(),
        swelog_folder_name: String::from("accomplishments"),
        work_file_name: String::from("daily-work.md"),
        daily_log_folder_name: String::from("days"),
        weekly_log_folder_name: String::from("weeks"),
        ..SwelogConfig::get_default_config()
    };

    let swelog_paths = SwelogPaths::new(&config);

    let cache_directory = temporary_directory.path().join("cache");

    let overwrite_existing_files = Overwrite::No;

    setup_swelog_files_from_config(&config, &cache_directory, overwrite_existing_files)
        .expect("swelog files should be created");

    assert!(swelog_paths.work_file.is_file());

    assert!(swelog_paths.daily_log_directory.is_dir());

    assert!(swelog_paths.weekly_log_directory.is_dir());

    drop(temporary_directory);
}

#[test]
fn setup_fails_when_work_file_exists_without_force() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.swelog_directory())
        .expect("swelog directory should be created");

    fs::write(test_context.work_file(), EXISTING_WORK_FILE_CONTENT)
        .expect("existing work file should be written");

    let overwrite_existing_files = Overwrite::No;

    let result = setup_swelog_files_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        overwrite_existing_files,
    );

    let error = result.expect_err("existing work file should not be overwritten without force");

    let error = error
        .downcast_ref::<SwelogFilesAlreadyExist>()
        .expect("error should be SetupFilesAlreadyExist");

    assert_eq!(error.swelog_path, test_context.work_file());

    let work_file_contents =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_contents, EXISTING_WORK_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn setup_fails_when_daily_log_directory_exists_without_force() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.daily_log_directory())
        .expect("daily log directory should be created");

    let overwrite_existing_files = Overwrite::No;

    let result = setup_swelog_files_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        overwrite_existing_files,
    );

    let error =
        result.expect_err("existing daily log directory should not be overwritten without force");

    let error = error
        .downcast_ref::<SwelogFilesAlreadyExist>()
        .expect("error should be SetupFilesAlreadyExist");

    assert_eq!(error.swelog_path, test_context.daily_log_directory());

    drop(test_context.temporary_directory);
}

#[test]
fn setup_fails_when_weekly_log_directory_exists_without_force() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.weekly_log_directory())
        .expect("weekly log directory should be created");

    let overwrite_existing_files = Overwrite::No;

    let result = setup_swelog_files_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        overwrite_existing_files,
    );

    let error =
        result.expect_err("existing weekly log directory should not be overwritten without force");

    let error = error
        .downcast_ref::<SwelogFilesAlreadyExist>()
        .expect("error should be SetupFilesAlreadyExist");

    assert_eq!(error.swelog_path, test_context.weekly_log_directory());

    drop(test_context.temporary_directory);
}

#[test]
fn setup_overwrites_existing_files_when_force_is_set() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.swelog_directory())
        .expect("swelog directory should be created");

    fs::write(test_context.work_file(), EXISTING_WORK_FILE_CONTENT)
        .expect("existing work file should be written");

    let overwrite_existing_files = Overwrite::Yes;

    setup_swelog_files_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        overwrite_existing_files,
    )
    .expect("swelog files should be overwritten");

    let work_file_contents =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert!(is_default_work_file_content(&work_file_contents));

    assert!(test_context.daily_log_directory().is_dir());

    assert!(test_context.weekly_log_directory().is_dir());

    drop(test_context.temporary_directory);
}

#[test]
fn setup_accepts_existing_log_directories_when_force_is_set() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.daily_log_directory())
        .expect("daily log directory should be created");

    fs::create_dir_all(test_context.weekly_log_directory())
        .expect("weekly log directory should be created");

    let overwrite_existing_files = Overwrite::Yes;

    setup_swelog_files_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        overwrite_existing_files,
    )
    .expect("existing log directories should be accepted");

    assert!(test_context.work_file().is_file());

    assert!(test_context.daily_log_directory().is_dir());

    assert!(test_context.weekly_log_directory().is_dir());

    drop(test_context.temporary_directory);
}

#[test]
fn setup_leaves_an_existing_context_file_alone() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.swelog_directory())
        .expect("swelog directory should be created");

    fs::write(test_context.context_file(), EXISTING_CONTEXT_FILE_CONTENT)
        .expect("existing context file should be written");

    let overwrite_existing_files = Overwrite::No;

    setup_swelog_files_from_config(
        &test_context.config,
        &test_context.cache_directory(),
        overwrite_existing_files,
    )
    .expect("an existing context file should not block setup");

    let context_file_contents =
        fs::read_to_string(test_context.context_file()).expect("context file should be readable");

    assert_eq!(context_file_contents, EXISTING_CONTEXT_FILE_CONTENT);

    drop(test_context.temporary_directory);
}
