use predicates::str::contains;
use undo::snapshot::get_undo_snapshot_file_path;

use crate::support::sandbox::{
    ACTIVITY_DATE,
    DEFAULT_WORK_FILE_CONTENT,
    DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS,
    SwelogSandbox,
    TODAY,
    WRITTEN_WORK_FILE_CONTENT,
};

const EXPECTED_DAILY_LOG_CONTENT: &str = "# Daily Log - 07-04-2026

## Priorities
- Ship end-to-end tests

## Log
- Reviewed the auth PR
- Paired on the release flow
";

fn get_sandbox_with_written_work_file() -> SwelogSandbox {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    sandbox
}

#[test]
fn log_writes_the_work_file_into_a_dated_daily_log_and_resets_the_work_file() {
    let sandbox = get_sandbox_with_written_work_file();

    sandbox.swelog().args(["log", "--date", ACTIVITY_DATE]).assert().success().stdout(contains(
        format!("Logged your work into {}", sandbox.daily_log_file(ACTIVITY_DATE).display()),
    ));

    assert_eq!(sandbox.read_daily_log(ACTIVITY_DATE), EXPECTED_DAILY_LOG_CONTENT);

    assert_eq!(sandbox.read_work_file(), DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS);

    assert!(get_undo_snapshot_file_path(&sandbox.cache_directory()).is_file());
}

#[test]
fn log_defaults_to_today_and_yesterday_is_relative_to_it() {
    let sandbox = get_sandbox_with_written_work_file();

    sandbox.swelog().args(["log", "--keep"]).assert().success();

    sandbox.swelog().args(["log", "--keep", "--yesterday"]).assert().success();

    assert!(sandbox.daily_log_file(TODAY).is_file());

    assert!(sandbox.daily_log_file(ACTIVITY_DATE).is_file());
}

#[test]
fn log_keeps_the_work_file_with_keep() {
    let sandbox = get_sandbox_with_written_work_file();

    sandbox.swelog().args(["log", "--date", ACTIVITY_DATE, "--keep"]).assert().success();

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}

#[test]
fn log_fails_when_the_work_file_is_untouched() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox
        .swelog()
        .args(["log", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("work file not updated"));

    assert!(!sandbox.daily_log_file(ACTIVITY_DATE).exists());
}

#[test]
fn log_fails_when_the_daily_log_exists_without_force() {
    let sandbox = get_sandbox_with_written_work_file();

    sandbox.swelog().args(["log", "--date", ACTIVITY_DATE, "--keep"]).assert().success();

    sandbox.write_work_file("# Today's Work\n\n## Log\n- Second attempt\n");

    sandbox
        .swelog()
        .args(["log", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("daily log already exists at"));

    assert_eq!(sandbox.read_daily_log(ACTIVITY_DATE), EXPECTED_DAILY_LOG_CONTENT);

    sandbox.swelog().args(["log", "--date", ACTIVITY_DATE, "--force"]).assert().success();

    assert_eq!(
        sandbox.read_daily_log(ACTIVITY_DATE),
        "# Daily Log - 07-04-2026\n\n## Log\n- Second attempt\n"
    );
}

#[test]
fn undo_restores_the_work_file_and_deletes_the_daily_log() {
    let sandbox = get_sandbox_with_written_work_file();

    sandbox.swelog().args(["log", "--date", ACTIVITY_DATE]).assert().success();

    sandbox
        .swelog()
        .arg("undo")
        .assert()
        .success()
        .stdout(contains("Restored your work file at"))
        .stdout(contains(format!("Deleted {}", sandbox.daily_log_file(ACTIVITY_DATE).display())));

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);

    assert!(!sandbox.daily_log_file(ACTIVITY_DATE).exists());

    sandbox.swelog().arg("undo").assert().failure().stderr(contains("nothing to undo"));
}

#[test]
fn reset_restores_the_default_work_file_and_can_be_undone() {
    let sandbox = get_sandbox_with_written_work_file();

    sandbox.swelog().arg("reset").assert().success().stdout(contains("Reset work file at"));

    assert_eq!(sandbox.read_work_file(), DEFAULT_WORK_FILE_CONTENT);

    sandbox.swelog().arg("undo").assert().success();

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}

#[test]
fn work_file_comments_stay_hidden_once_a_log_has_been_written() {
    let sandbox = get_sandbox_with_written_work_file();

    sandbox.swelog().args(["log", "--date", ACTIVITY_DATE]).assert().success();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    sandbox.swelog().arg("reset").assert().success();

    assert_eq!(sandbox.read_work_file(), DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS);
}
