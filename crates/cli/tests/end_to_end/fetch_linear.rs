use config::swelog_config::SwelogConfig;
use httpmock::MockServer;
use predicates::str::contains;

use crate::support::{
    linear::{
        LINEAR_SECTION_FOR_ACTIVE_ISSUES,
        LINEAR_SECTION_FOR_THE_DAY,
        LINEAR_USERNAME,
        linear_mcp_url,
        mock_linear_mcp,
        store_linear_authorization,
    },
    sandbox::{
        ACTIVITY_DATE,
        SwelogSandbox,
        WRITTEN_WORK_FILE_CONTENT,
    },
};

const WORK_FILE_WITH_LINEAR_SECTION_FOR_THE_DAY: &str = "# Today's Work

## Priorities

- Ship end-to-end tests

## Linear

### In Progress

- [SWE-42](https://linear.app/example/issue/SWE-42) Ship end-to-end tests

### Done

- [SWE-41](https://linear.app/example/issue/SWE-41) Fix work file formatting

## Log

- Reviewed the auth PR
- Paired on the release flow
";

fn get_sandbox_configured_for_linear() -> SwelogSandbox {
    let sandbox = SwelogSandbox::new();

    sandbox.write_config(&SwelogConfig {
        linear_username: Some(String::from(LINEAR_USERNAME)),
        ..sandbox.default_config()
    });

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    store_linear_authorization(&sandbox);

    sandbox
}

#[test]
fn fetch_linear_records_the_days_issues_grouped_by_status() {
    let sandbox = get_sandbox_configured_for_linear();

    let linear = MockServer::start();

    let linear_mocks = mock_linear_mcp(&linear);

    sandbox
        .swelog()
        .env("SWELOG_LINEAR_MCP_URL", linear_mcp_url(&linear))
        .args(["fetch", "linear", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Fetching Linear issues..."))
        .stdout(contains("Added 2 Linear issues from 07-04-2026 to your work file."));

    linear_mocks.initialize.assert();

    linear_mocks.initialized.assert();

    linear_mocks.list_issues.assert();

    assert!(sandbox.read_work_file().contains(LINEAR_SECTION_FOR_THE_DAY));

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_LINEAR_SECTION_FOR_THE_DAY);
}

#[test]
fn fetch_linear_without_a_date_records_the_active_issues() {
    let sandbox = get_sandbox_configured_for_linear();

    let linear = MockServer::start();

    let linear_mocks = mock_linear_mcp(&linear);

    sandbox
        .swelog()
        .env("SWELOG_LINEAR_MCP_URL", linear_mcp_url(&linear))
        .args(["fetch", "linear"])
        .assert()
        .success()
        .stdout(contains("Added 2 active Linear issues to your work file."));

    linear_mocks.list_issues.assert();

    assert!(sandbox.read_work_file().contains(LINEAR_SECTION_FOR_ACTIVE_ISSUES));
}

#[test]
fn fetch_linear_fails_when_the_username_is_not_configured() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    store_linear_authorization(&sandbox);

    let linear = MockServer::start();

    let linear_mocks = mock_linear_mcp(&linear);

    sandbox
        .swelog()
        .env("SWELOG_LINEAR_MCP_URL", linear_mcp_url(&linear))
        .args(["fetch", "linear", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("Linear username is not configured"));

    assert_eq!(linear_mocks.initialize.calls(), 0);

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}
