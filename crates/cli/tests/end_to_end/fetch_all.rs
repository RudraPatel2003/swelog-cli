use config::swelog_config::SwelogConfig;
use httpmock::MockServer;
use predicates::str::contains;

use crate::support::{
    github::{
        GITHUB_SECTION,
        mock_github_with_activity_on,
    },
    linear::{
        LINEAR_SECTION_FOR_THE_DAY,
        LINEAR_USERNAME,
        linear_mcp_url,
        mock_linear_mcp,
        store_linear_authorization,
    },
    sandbox::{
        ACTIVITY_DATE,
        GITHUB_TOKEN,
        SwelogSandbox,
        WRITTEN_WORK_FILE_CONTENT,
    },
    sentry::{
        SENTRY_AUTH_TOKEN,
        SENTRY_ORGANIZATION,
        SENTRY_SECTION,
        mock_sentry_with_issues_on,
    },
};

#[test]
fn fetch_all_runs_only_the_sources_with_a_credential() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    let github = MockServer::start();

    let github_mocks = mock_github_with_activity_on(&github, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", github.base_url())
        .args(["fetch", "all", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Running the fetch commands you have configured: GitHub."))
        .stdout(contains("Recorded 4 GitHub PRs in your work file."));

    github_mocks
        .user
        .assert();

    assert!(
        sandbox
            .read_work_file()
            .contains(GITHUB_SECTION)
    );
}

#[test]
fn fetch_all_runs_every_configured_source_and_records_each_section() {
    let sandbox = SwelogSandbox::new();

    sandbox.write_config(&SwelogConfig {
        linear_username: Some(String::from(LINEAR_USERNAME)),
        sentry_organization: Some(String::from(SENTRY_ORGANIZATION)),
        ..sandbox.default_config()
    });

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    store_linear_authorization(&sandbox);

    let github = MockServer::start();

    let github_mocks = mock_github_with_activity_on(&github, ACTIVITY_DATE);

    let linear = MockServer::start();

    let linear_mocks = mock_linear_mcp(&linear);

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_issues_on(&sentry, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", github.base_url())
        .env("SWELOG_LINEAR_MCP_URL", linear_mcp_url(&linear))
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "all", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Running the fetch commands you have configured: GitHub, Linear, Sentry."))
        .stdout(contains("Recorded 4 GitHub PRs in your work file."))
        .stdout(contains("Added 2 Linear issues from 07-04-2026 to your work file."))
        .stdout(contains("Added 3 Sentry issues from 07-04-2026 to your work file."));

    github_mocks
        .user
        .assert();

    linear_mocks
        .list_issues
        .assert();

    sentry_mocks.assert_every_endpoint_was_called();

    let work_file_content = sandbox.read_work_file();

    assert!(work_file_content.contains(GITHUB_SECTION));

    assert!(work_file_content.contains(LINEAR_SECTION_FOR_THE_DAY));

    assert!(work_file_content.contains(SENTRY_SECTION));

    assert!(
        work_file_content
            .ends_with("## Log\n\n- Reviewed the auth PR\n- Paired on the release flow\n")
    );
}

#[test]
fn fetch_all_fails_when_no_source_has_a_credential() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox
        .swelog()
        .args(["fetch", "all", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("no fetch commands are configured"))
        .stderr(contains("swelog fetch status"));
}

#[test]
fn fetch_status_explains_what_each_source_still_needs() {
    let sandbox = SwelogSandbox::new();

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .args(["fetch", "status"])
        .assert()
        .success()
        .stdout(contains("GitHub              included"))
        .stdout(contains("Linear              not included, Linear authorization is not stored"))
        .stdout(contains(
            "Google Calendar     not included, Google Calendar authorization is not stored",
        ))
        .stdout(contains("Sentry              not included, Sentry auth token is not stored"));
}

#[test]
fn fetch_status_reports_missing_sentry_configuration_once_authorized() {
    let sandbox = SwelogSandbox::new();

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .args(["fetch", "status"])
        .assert()
        .success()
        .stdout(contains("Sentry              not included, sentryOrganization is not configured"));
}

#[test]
fn fetch_status_includes_linear_once_authorized_and_configured() {
    let sandbox = SwelogSandbox::new();

    sandbox.write_config(&SwelogConfig {
        linear_username: Some(String::from(LINEAR_USERNAME)),
        ..sandbox.default_config()
    });

    store_linear_authorization(&sandbox);

    sandbox
        .swelog()
        .args(["fetch", "status"])
        .assert()
        .success()
        .stdout(contains("Linear              included"));
}

#[test]
fn fetch_status_reports_missing_linear_configuration_once_authorized() {
    let sandbox = SwelogSandbox::new();

    sandbox.store_credential(credentials::credential::Credential::Linear, "{}");

    sandbox
        .swelog()
        .args(["fetch", "status"])
        .assert()
        .success()
        .stdout(contains("Linear              not included, linearUsername is not configured"));
}
