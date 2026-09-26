use std::fs;

use tempfile::tempdir;

use super::*;

const CONTEXT_FILE_CONTENT: &str = "backend engineer on platform team";

#[test]
fn get_context_file_content_returns_none_when_the_file_is_missing() {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let context_file = temporary_directory.path().join(CONTEXT_FILE_NAME);

    let context_file_content = get_context_file_content(&context_file)
        .expect("missing context file should not be an error");

    assert_eq!(context_file_content, None);

    drop(temporary_directory);
}

#[test]
fn get_context_file_content_returns_the_contents_when_the_file_exists() {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let context_file = temporary_directory.path().join(CONTEXT_FILE_NAME);

    fs::write(&context_file, CONTEXT_FILE_CONTENT).expect("context file should be written");

    let context_file_content =
        get_context_file_content(&context_file).expect("context file should be readable");

    assert_eq!(context_file_content.as_deref(), Some(CONTEXT_FILE_CONTENT));

    drop(temporary_directory);
}

#[test]
fn get_context_file_content_returns_none_when_the_path_is_a_directory() {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let context_file_content = get_context_file_content(temporary_directory.path())
        .expect("a directory should not be read as a context file");

    assert_eq!(context_file_content, None);

    drop(temporary_directory);
}
