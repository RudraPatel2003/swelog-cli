use config::{
    integrations_config::IntegrationsConfig,
    swelog_config::SwelogConfig,
};
use credentials::credential::Credential;
use httpmock::MockServer;
use predicates::str::contains;

use crate::support::{
    sandbox::{
        ACTIVITY_DATE,
        LAST_FRIDAY,
        SwelogSandbox,
        TODAY,
        WRITTEN_WORK_FILE_CONTENT,
    },
    sentry::{
        SENTRY_AUTH_TOKEN,
        SENTRY_SECTION,
        SENTRY_SECTION_WITH_RESOLVED_ISSUES,
        get_sentry_config,
        mock_sentry_rejecting_the_token,
        mock_sentry_with_current_issues,
        mock_sentry_with_current_issues_and_resolutions_on,
        mock_sentry_with_issues_on,
        mock_sentry_with_no_issues_on,
        mock_sentry_with_resolved_issues_on,
        mock_sentry_with_two_pages_on,
    },
};

const WORK_FILE_WITH_SENTRY_SECTION: &str = "# Today's Work

## Priorities

- Ship end-to-end tests

## Sentry

### Escalating

- [SWELOG-42](https://example.sentry.io/issues/42/) TypeError: cannot read properties of undefined

### Regressed

- [SWELOG-41](https://example.sentry.io/issues/41/) ConnectionError: vault is unreachable

### Ongoing

- [SWELOG-40](https://example.sentry.io/issues/40/) DeprecationWarning: the llm config key moved

## Log

- Reviewed the auth PR
- Paired on the release flow
";

const WORK_FILE_WITH_RESOLVED_SENTRY_ISSUES: &str = "# Today's Work

## Priorities

- Ship end-to-end tests

## Sentry

### Escalating

- [SWELOG-42](https://example.sentry.io/issues/42/) TypeError: cannot read properties of undefined

### Regressed

- [SWELOG-41](https://example.sentry.io/issues/41/) ConnectionError: vault is unreachable

### Ongoing

- [SWELOG-40](https://example.sentry.io/issues/40/) DeprecationWarning: the llm config key moved

### Resolved

- [SWELOG-39](https://example.sentry.io/issues/39/) KeyError: 'vault_path'

## Log

- Reviewed the auth PR
- Paired on the release flow
";

fn get_sandbox_configured_for_sentry() -> SwelogSandbox {
    let sandbox = SwelogSandbox::new();

    sandbox.write_config(&SwelogConfig {
        integrations: IntegrationsConfig {
            sentry: Some(get_sentry_config()),
            ..IntegrationsConfig::default()
        },
        ..sandbox.default_config()
    });

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    sandbox
}

#[test]
fn fetch_sentry_records_the_days_issues_grouped_by_substatus() {
    let sandbox = get_sandbox_configured_for_sentry();

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_issues_on(&sentry, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Fetching Sentry issues..."))
        .stdout(contains("Added 3 Sentry issues from 07-04-2026 to your work file."));

    sentry_mocks.assert_every_endpoint_was_called();

    assert!(
        sandbox
            .read_work_file()
            .contains(SENTRY_SECTION)
    );

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_SENTRY_SECTION);
}

#[test]
fn fetch_sentry_records_last_fridays_issues() {
    let sandbox = get_sandbox_configured_for_sentry();

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_issues_on(&sentry, LAST_FRIDAY);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry", "--last-friday"])
        .assert()
        .success()
        .stdout(contains("Added 3 Sentry issues from 07-03-2026 to your work file."));

    sentry_mocks.assert_every_endpoint_was_called();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_SENTRY_SECTION);
}

#[test]
fn fetch_sentry_without_a_date_records_the_unresolved_issues() {
    let sandbox = get_sandbox_configured_for_sentry();

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_current_issues(&sentry);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry"])
        .assert()
        .success()
        .stdout(contains("Added 3 Sentry issues to your work file."));

    sentry_mocks.assert_every_endpoint_was_called();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_SENTRY_SECTION);
}

#[test]
fn fetch_sentry_records_the_issues_resolved_on_the_date() {
    let sandbox = get_sandbox_configured_for_sentry();

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_resolved_issues_on(&sentry, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Added 4 Sentry issues from 07-04-2026 to your work file."));

    sentry_mocks.assert_every_endpoint_was_called();

    assert!(
        sandbox
            .read_work_file()
            .contains(SENTRY_SECTION_WITH_RESOLVED_ISSUES)
    );

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_RESOLVED_SENTRY_ISSUES);
}

#[test]
fn fetch_sentry_without_a_date_records_the_issues_resolved_today() {
    let sandbox = get_sandbox_configured_for_sentry();

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_current_issues_and_resolutions_on(&sentry, TODAY);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry"])
        .assert()
        .success()
        .stdout(contains("Added 4 Sentry issues to your work file."));

    sentry_mocks.assert_every_endpoint_was_called();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_RESOLVED_SENTRY_ISSUES);
}

#[test]
fn fetch_sentry_follows_pagination_to_the_last_page() {
    let sandbox = get_sandbox_configured_for_sentry();

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_two_pages_on(&sentry, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Added 3 Sentry issues from 07-04-2026 to your work file."));

    sentry_mocks.assert_every_endpoint_was_called();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_SENTRY_SECTION);
}

#[test]
fn fetch_sentry_replaces_an_existing_section_and_removes_it_when_there_are_no_issues() {
    let sandbox = get_sandbox_configured_for_sentry();

    let active_sentry = MockServer::start();

    mock_sentry_with_issues_on(&active_sentry, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", active_sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", active_sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_SENTRY_SECTION);

    let quiet_sentry = MockServer::start();

    mock_sentry_with_no_issues_on(&quiet_sentry, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", quiet_sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("No Sentry activity found for 07-04-2026."));

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}

#[test]
fn fetch_sentry_uses_the_stored_token_when_the_environment_variable_is_unset() {
    let sandbox = get_sandbox_configured_for_sentry();

    sandbox.store_credential(Credential::Sentry, SENTRY_AUTH_TOKEN);

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_issues_on(&sentry, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    sentry_mocks.assert_every_endpoint_was_called();

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_SENTRY_SECTION);
}

#[test]
fn fetch_sentry_fails_fast_without_a_token_when_there_is_no_terminal() {
    let sandbox = get_sandbox_configured_for_sentry();

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_issues_on(&sentry, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("Sentry auth token is not available"))
        .stderr(contains("SENTRY_AUTH_TOKEN"));

    assert!(
        sentry_mocks
            .issue_pages
            .iter()
            .all(|issue_page| issue_page.calls() == 0)
    );

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}

#[test]
fn fetch_sentry_reports_a_rejected_token() {
    let sandbox = get_sandbox_configured_for_sentry();

    let sentry = MockServer::start();

    let rejection = mock_sentry_rejecting_the_token(&sentry);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("Sentry rejected your auth token with status 401"))
        .stderr(contains("swelog auth clear sentry"));

    assert!(rejection.calls() > 0);

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}

#[test]
fn fetch_sentry_fails_when_the_organization_is_not_configured() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    let sentry = MockServer::start();

    let sentry_mocks = mock_sentry_with_issues_on(&sentry, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("SENTRY_AUTH_TOKEN", SENTRY_AUTH_TOKEN)
        .env("SWELOG_SENTRY_API_URL", sentry.base_url())
        .args(["fetch", "sentry", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("Sentry organization is not configured"))
        .stderr(contains("integrations.sentry.organization"));

    assert!(
        sentry_mocks
            .issue_pages
            .iter()
            .all(|issue_page| issue_page.calls() == 0)
    );

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}
