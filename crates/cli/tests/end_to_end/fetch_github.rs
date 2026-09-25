use httpmock::MockServer;
use predicates::str::contains;

use crate::support::{
    github::{
        GITHUB_SECTION,
        mock_github_rejecting_the_token,
        mock_github_with_activity_on,
        mock_github_with_no_activity_on,
    },
    sandbox::{
        ACTIVITY_DATE,
        DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS,
        GITHUB_TOKEN,
        LAST_FRIDAY,
        SwelogSandbox,
        WRITTEN_WORK_FILE_CONTENT,
    },
};

const WORK_FILE_WITH_GITHUB_SECTION: &str = r#"# Today's Work

## Priorities
- Ship end-to-end tests

## GitHub
### Opened
- "Add end-to-end tests" ([#42](https://github.com/example/swelog/pull/42)) in [example/swelog](https://github.com/example/swelog)

### Merged
- "Fix work file formatting" ([#43](https://github.com/example/swelog/pull/43)) in [example/swelog](https://github.com/example/swelog)

### Closed
- "Try a separate formatting crate" ([#44](https://github.com/example/swelog/pull/44)) in [example/swelog](https://github.com/example/swelog)

### Reviewed
- "Add Linear integration" ([#50](https://github.com/example/swelog/pull/50)) in [example/swelog](https://github.com/example/swelog)

## Log
- Reviewed the auth PR
- Paired on the release flow
"#;

const DAILY_LOG_WITH_GITHUB_SECTION: &str = r#"# Daily Log - 07-04-2026

## Priorities
- Ship end-to-end tests

## GitHub
### Opened
- "Add end-to-end tests" ([#42](https://github.com/example/swelog/pull/42)) in [example/swelog](https://github.com/example/swelog)

### Merged
- "Fix work file formatting" ([#43](https://github.com/example/swelog/pull/43)) in [example/swelog](https://github.com/example/swelog)

### Closed
- "Try a separate formatting crate" ([#44](https://github.com/example/swelog/pull/44)) in [example/swelog](https://github.com/example/swelog)

### Reviewed
- "Add Linear integration" ([#50](https://github.com/example/swelog/pull/50)) in [example/swelog](https://github.com/example/swelog)

## Log
- Reviewed the auth PR
- Paired on the release flow
"#;

fn get_sandbox_with_written_work_file() -> SwelogSandbox {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    sandbox
}

#[test]
fn fetch_github_records_the_days_pull_requests_above_the_log_section() {
    let sandbox = get_sandbox_with_written_work_file();

    let github = MockServer::start();

    let github_mocks = mock_github_with_activity_on(&github, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", github.base_url())
        .args(["fetch", "github", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Fetching GitHub PRs..."))
        .stdout(contains("Recorded 4 GitHub PRs in your work file."));

    github_mocks.assert_every_endpoint_was_called();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_GITHUB_SECTION);
}

#[test]
fn fetch_github_records_last_fridays_pull_requests() {
    let sandbox = get_sandbox_with_written_work_file();

    let github = MockServer::start();

    let github_mocks = mock_github_with_activity_on(&github, LAST_FRIDAY);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", github.base_url())
        .args(["fetch", "github", "--last-friday"])
        .assert()
        .success()
        .stdout(contains("Recorded 4 GitHub PRs in your work file."));

    github_mocks.assert_every_endpoint_was_called();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_GITHUB_SECTION);
}

#[test]
fn fetched_activity_flows_from_the_work_file_into_the_daily_log_and_back_through_undo() {
    let sandbox = get_sandbox_with_written_work_file();

    let github = MockServer::start();

    mock_github_with_activity_on(&github, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", github.base_url())
        .args(["fetch", "github", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    assert!(sandbox.read_work_file().contains(GITHUB_SECTION));

    sandbox.swelog().args(["log", "--date", ACTIVITY_DATE]).assert().success();

    assert_eq!(sandbox.read_daily_log(ACTIVITY_DATE), DAILY_LOG_WITH_GITHUB_SECTION);

    assert_eq!(sandbox.read_work_file(), DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS);

    sandbox.swelog().arg("undo").assert().success();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_GITHUB_SECTION);

    assert!(!sandbox.daily_log_file(ACTIVITY_DATE).exists());
}

#[test]
fn fetch_github_replaces_an_existing_section_and_removes_it_when_there_is_no_activity() {
    let sandbox = get_sandbox_with_written_work_file();

    let active_github = MockServer::start();

    mock_github_with_activity_on(&active_github, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", active_github.base_url())
        .args(["fetch", "github", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", active_github.base_url())
        .args(["fetch", "github", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_GITHUB_SECTION);

    let quiet_github = MockServer::start();

    mock_github_with_no_activity_on(&quiet_github, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", quiet_github.base_url())
        .args(["fetch", "github", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("No GitHub activity found."));

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}

#[test]
fn fetch_github_uses_the_stored_token_when_the_environment_variable_is_unset() {
    let sandbox = get_sandbox_with_written_work_file();

    sandbox.store_credential(credentials::credential::Credential::Github, GITHUB_TOKEN);

    let github = MockServer::start();

    let github_mocks = mock_github_with_activity_on(&github, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SWELOG_GITHUB_API_URL", github.base_url())
        .args(["fetch", "github", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    github_mocks.user.assert();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_GITHUB_SECTION);
}

#[test]
fn fetch_github_fails_fast_without_a_token_when_there_is_no_terminal() {
    let sandbox = get_sandbox_with_written_work_file();

    let github = MockServer::start();

    let github_mocks = mock_github_with_activity_on(&github, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SWELOG_GITHUB_API_URL", github.base_url())
        .args(["fetch", "github", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("GitHub token is not available"))
        .stderr(contains("GITHUB_TOKEN"));

    assert_eq!(github_mocks.user.calls(), 0);

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}

#[test]
fn fetch_github_reports_a_rejected_token() {
    let sandbox = get_sandbox_with_written_work_file();

    let github = MockServer::start();

    let rejection = mock_github_rejecting_the_token(&github);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", github.base_url())
        .args(["fetch", "github", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("GitHub rejected your token with status 401"))
        .stderr(contains("swelog auth clear github"));

    rejection.assert();

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}
