use config::context_file::CONTEXT_FILE_NAME;
use tempfile::tempdir;

use super::*;

const UNFORMATTED_CONTEXT_FILE_CONTENT: &str = r"# Engineer Context
## Systems Owned
* Checkout APIs
";

const FORMATTED_CONTEXT_FILE_CONTENT: &str = r"# Engineer Context

## Systems Owned

- Checkout APIs
";

#[test]
fn format_context_file_does_nothing_when_the_file_is_missing() {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let context_file = temporary_directory.path().join(CONTEXT_FILE_NAME);

    format_context_file(&context_file).expect("missing context file should not be an error");

    assert!(!context_file.exists());

    drop(temporary_directory);
}

#[test]
fn format_context_file_formats_the_context_file_in_place() {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let context_file = temporary_directory.path().join(CONTEXT_FILE_NAME);

    fs::write(&context_file, UNFORMATTED_CONTEXT_FILE_CONTENT)
        .expect("context file should be written");

    format_context_file(&context_file).expect("context file should be formatted");

    let context_file_content =
        fs::read_to_string(&context_file).expect("context file should be readable");

    assert_eq!(context_file_content, FORMATTED_CONTEXT_FILE_CONTENT);

    drop(temporary_directory);
}
