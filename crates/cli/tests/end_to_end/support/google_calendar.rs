use credentials::credential::Credential;
use httpmock::{
    Method::{
        GET,
        POST,
    },
    Mock,
    MockServer,
};

use crate::support::sandbox::SwelogSandbox;

pub const GOOGLE_CLIENT_ID: &str = "test-client-id";

pub const GOOGLE_CLIENT_SECRET: &str = "test-client-secret";

pub const ACCESS_TOKEN: &str = "ya29.stored";

pub const REFRESHED_ACCESS_TOKEN: &str = "ya29.refreshed";

pub const REFRESH_TOKEN: &str = "1//refresh";

const FAR_FUTURE_EPOCH_SECONDS: u64 = 4_102_444_800;

const EVENTS_RESPONSE: &str = r#"{
  "items": [
    {
      "summary": "Standup",
      "status": "confirmed",
      "start": { "dateTime": "2026-07-04T09:00:00Z" },
      "end": { "dateTime": "2026-07-04T09:15:00Z" },
      "attendees": [{ "self": true, "responseStatus": "accepted" }]
    },
    {
      "summary": "Optional sync",
      "status": "confirmed",
      "start": { "dateTime": "2026-07-04T14:00:00Z" },
      "end": { "dateTime": "2026-07-04T14:30:00Z" },
      "attendees": [{ "self": true, "responseStatus": "declined" }]
    },
    {
      "summary": "Company holiday",
      "status": "confirmed",
      "start": { "date": "2026-07-04" },
      "end": { "date": "2026-07-05" }
    }
  ]
}"#;

pub const GOOGLE_CALENDAR_SECTION: &str = "## Google Calendar

- 9:00 AM - 9:15 AM | Standup
- ~~2:00 PM - 2:30 PM | Optional sync~~";

pub fn store_google_credentials(sandbox: &SwelogSandbox, expires_at: u64) {
    let google_credentials = serde_json::json!({
        "accessToken": ACCESS_TOKEN,
        "refreshToken": REFRESH_TOKEN,
        "expiresAt": expires_at,
        "scopes": ["https://www.googleapis.com/auth/calendar.events.readonly"]
    });

    sandbox.store_credential(Credential::GoogleCalendar, &google_credentials.to_string());
}

pub fn store_valid_google_credentials(sandbox: &SwelogSandbox) {
    store_google_credentials(sandbox, FAR_FUTURE_EPOCH_SECONDS);
}

pub fn store_expired_google_credentials(sandbox: &SwelogSandbox) {
    store_google_credentials(sandbox, 0);
}

pub fn mock_events_for_access_token<'server>(
    server: &'server MockServer,
    access_token: &str,
) -> Mock<'server> {
    server.mock(|when, then| {
        when.method(GET)
            .path("/calendar/v3/calendars/primary/events")
            .query_param("timeMin", "2026-07-04T00:00:00Z")
            .query_param("timeMax", "2026-07-05T00:00:00Z")
            .header("authorization", format!("Bearer {access_token}"));

        then.status(200).header("content-type", "application/json").body(EVENTS_RESPONSE);
    })
}

pub fn mock_token_refresh(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(POST)
            .path("/token")
            .form_urlencoded_tuple("grant_type", "refresh_token")
            .form_urlencoded_tuple("refresh_token", REFRESH_TOKEN)
            .form_urlencoded_tuple("client_id", GOOGLE_CLIENT_ID);

        then.status(200).header("content-type", "application/json").json_body(serde_json::json!({
            "access_token": REFRESHED_ACCESS_TOKEN,
            "expires_in": 3600,
            "scope": "https://www.googleapis.com/auth/calendar.events.readonly",
            "token_type": "Bearer"
        }));
    })
}
