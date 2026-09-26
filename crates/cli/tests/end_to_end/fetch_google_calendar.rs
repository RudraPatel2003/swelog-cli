use credentials::credential::Credential;
use httpmock::MockServer;
use predicates::str::contains;

use crate::support::{
    google_calendar::{
        ACCESS_TOKEN,
        GOOGLE_CALENDAR_SECTION,
        GOOGLE_CLIENT_ID,
        GOOGLE_CLIENT_SECRET,
        REFRESHED_ACCESS_TOKEN,
        mock_events_for_access_token,
        mock_token_refresh,
        store_expired_google_credentials,
        store_valid_google_credentials,
    },
    sandbox::{
        ACTIVITY_DATE,
        SwelogSandbox,
        WRITTEN_WORK_FILE_CONTENT,
    },
};

const WORK_FILE_WITH_GOOGLE_CALENDAR_SECTION: &str = "# Today's Work

## Priorities

- Ship end-to-end tests

## Google Calendar

- 9:00 AM - 9:15 AM | Standup
- ~~2:00 PM - 2:30 PM | Optional sync~~

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
fn fetch_google_calendar_records_the_days_timed_meetings() {
    let sandbox = get_sandbox_with_written_work_file();

    store_valid_google_credentials(&sandbox);

    let google = MockServer::start();

    let events = mock_events_for_access_token(&google, ACCESS_TOKEN);

    sandbox
        .swelog()
        .env("SWELOG_GOOGLE_CLIENT_ID", GOOGLE_CLIENT_ID)
        .env("SWELOG_GOOGLE_CLIENT_SECRET", GOOGLE_CLIENT_SECRET)
        .env("SWELOG_GOOGLE_CALENDAR_API_URL", google.base_url())
        .env("SWELOG_GOOGLE_TOKEN_URL", google.base_url())
        .args(["fetch", "google-calendar", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Fetching Google Calendar events..."))
        .stdout(contains("Added 2 meetings from 07-04-2026 to your work file."));

    events.assert();

    assert!(sandbox.read_work_file().contains(GOOGLE_CALENDAR_SECTION));

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_GOOGLE_CALENDAR_SECTION);
}

#[test]
fn fetch_google_calendar_refreshes_an_expired_access_token_and_stores_it() {
    let sandbox = get_sandbox_with_written_work_file();

    store_expired_google_credentials(&sandbox);

    let google = MockServer::start();

    let token_refresh = mock_token_refresh(&google);

    let events = mock_events_for_access_token(&google, REFRESHED_ACCESS_TOKEN);

    sandbox
        .swelog()
        .env("SWELOG_GOOGLE_CLIENT_ID", GOOGLE_CLIENT_ID)
        .env("SWELOG_GOOGLE_CLIENT_SECRET", GOOGLE_CLIENT_SECRET)
        .env("SWELOG_GOOGLE_CALENDAR_API_URL", google.base_url())
        .env("SWELOG_GOOGLE_TOKEN_URL", google.base_url())
        .args(["fetch", "google-calendar", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    token_refresh.assert();

    events.assert();

    let stored_credentials = sandbox
        .read_credential(Credential::GoogleCalendar)
        .expect("Google credentials should still be stored");

    assert!(stored_credentials.contains(REFRESHED_ACCESS_TOKEN));

    assert_eq!(sandbox.read_work_file(), WORK_FILE_WITH_GOOGLE_CALENDAR_SECTION);
}
