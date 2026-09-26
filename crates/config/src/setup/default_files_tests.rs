use super::*;

const EDITED_WORK_FILE_CONTENT: &str = "# Today's Work

## Priorities

## Log
- shipped the checkout retry fix
";

#[test]
fn default_work_file_content_is_recognized_as_default() {
    assert!(is_default_work_file_content(DEFAULT_WORK_FILE_CONTENT));
}

#[test]
fn default_work_file_content_without_comments_is_recognized_as_default() {
    assert!(is_default_work_file_content(DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS));
}

#[test]
fn edited_work_file_content_is_not_recognized_as_default() {
    assert!(!is_default_work_file_content(EDITED_WORK_FILE_CONTENT));
}

#[test]
fn default_work_file_content_without_comments_keeps_the_same_headings() {
    assert!(!DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS.contains("<!--"));

    assert!(DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS.contains("## Priorities"));

    assert!(DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS.contains("## Log"));
}
