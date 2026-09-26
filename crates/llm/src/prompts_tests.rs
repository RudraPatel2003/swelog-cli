use super::*;

const CONTEXT_FILE_CONTENT: &str = "backend engineer on platform team";

const WORK_FILE_CONTENT: &str = "- Reviewed auth PR";

fn get_log_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 6, 1).expect("log date should be valid")
}

#[test]
fn daily_log_prompt_includes_the_context_when_it_is_given() {
    let log_date = get_log_date();

    let prompt = get_daily_log_prompt(WORK_FILE_CONTENT, Some(CONTEXT_FILE_CONTENT), &log_date);

    assert!(prompt.contains(CONTEXT_FILE_CONTENT));

    assert!(!prompt.contains(NO_CONTEXT_GIVEN));
}

#[test]
fn daily_log_prompt_says_no_context_given_when_context_is_absent() {
    let log_date = get_log_date();

    let prompt = get_daily_log_prompt(WORK_FILE_CONTENT, None, &log_date);

    assert!(prompt.contains(NO_CONTEXT_GIVEN));
}

#[test]
fn weekly_log_prompt_includes_the_context_when_it_is_given() {
    let daily_logs = vec![String::from("# Daily Log - 06-01-2026")];

    let log_date = get_log_date();

    let prompt = get_weekly_log_prompt(&daily_logs, Some(CONTEXT_FILE_CONTENT), &log_date);

    assert!(prompt.contains(CONTEXT_FILE_CONTENT));

    assert!(!prompt.contains(NO_CONTEXT_GIVEN));
}

#[test]
fn weekly_log_prompt_says_no_context_given_when_context_is_absent() {
    let daily_logs = vec![String::from("# Daily Log - 06-01-2026")];

    let log_date = get_log_date();

    let prompt = get_weekly_log_prompt(&daily_logs, None, &log_date);

    assert!(prompt.contains(NO_CONTEXT_GIVEN));
}
