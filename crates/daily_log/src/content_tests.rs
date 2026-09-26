use super::*;

fn get_mock_log_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 5, 23).expect("test date should be valid")
}

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

#[test]
fn build_daily_log_content_replaces_the_work_file_heading() {
    let log_date = get_mock_log_date();

    let daily_log_content = build_daily_log_content(WORK_FILE_CONTENT, &log_date);

    assert_eq!(daily_log_content, EXPECTED_DAILY_LOG_CONTENT);
}

#[test]
fn build_daily_log_content_keeps_integration_sections_verbatim() {
    let work_file_content = r"# Today's Work

## GitHub
- Merged [#412](https://example.com)

## Log
<!-- Quick capture. -->
- Fixed flaky billing test
";

    let log_date = get_mock_log_date();

    let daily_log_content = build_daily_log_content(work_file_content, &log_date);

    let expected_daily_log_content = r"# Daily Log - 05-23-2026

## GitHub
- Merged [#412](https://example.com)

## Log
<!-- Quick capture. -->
- Fixed flaky billing test
";

    assert_eq!(daily_log_content, expected_daily_log_content);
}

#[test]
fn build_daily_log_content_replaces_only_the_first_heading() {
    let work_file_content = r"# Today's Work

# Second Heading
- Reviewed auth PR
";

    let log_date = get_mock_log_date();

    let daily_log_content = build_daily_log_content(work_file_content, &log_date);

    let expected_daily_log_content = r"# Daily Log - 05-23-2026

# Second Heading
- Reviewed auth PR
";

    assert_eq!(daily_log_content, expected_daily_log_content);
}
