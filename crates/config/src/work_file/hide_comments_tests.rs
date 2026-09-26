use std::path::PathBuf;

use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const EXISTING_FLAG_FILE_CONTENT: &str = "written by an earlier run";

struct TestContext {
    temporary_directory: TempDir,
    cache_directory: PathBuf,
}

impl TestContext {
    fn flag_file_path(&self) -> PathBuf {
        self.cache_directory.join(HIDE_COMMENTS_FLAG_FILE_NAME)
    }
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let cache_directory = temporary_directory.path().join("cache").join("swelog");

    TestContext { temporary_directory, cache_directory }
}

#[test]
fn hide_comments_flag_is_missing_before_it_is_set() {
    let test_context = get_test_context();

    assert!(!has_hide_comments_flag(&test_context.cache_directory));

    drop(test_context.temporary_directory);
}

#[test]
fn set_hide_comments_flag_creates_parent_directories() {
    let test_context = get_test_context();

    set_hide_comments_flag(&test_context.cache_directory)
        .expect("hide comments flag should be written");

    assert!(test_context.flag_file_path().is_file());

    drop(test_context.temporary_directory);
}

#[test]
fn hide_comments_flag_is_present_after_it_is_set() {
    let test_context = get_test_context();

    set_hide_comments_flag(&test_context.cache_directory)
        .expect("hide comments flag should be written");

    assert!(has_hide_comments_flag(&test_context.cache_directory));

    drop(test_context.temporary_directory);
}

#[test]
fn set_hide_comments_flag_leaves_an_existing_flag_untouched() {
    let test_context = get_test_context();

    set_hide_comments_flag(&test_context.cache_directory)
        .expect("hide comments flag should be written");

    fs::write(test_context.flag_file_path(), EXISTING_FLAG_FILE_CONTENT)
        .expect("existing flag file should be written");

    set_hide_comments_flag(&test_context.cache_directory)
        .expect("hide comments flag should stay unchanged");

    let flag_file_content =
        fs::read_to_string(test_context.flag_file_path()).expect("flag file should be readable");

    assert_eq!(flag_file_content, EXISTING_FLAG_FILE_CONTENT);

    drop(test_context.temporary_directory);
}
