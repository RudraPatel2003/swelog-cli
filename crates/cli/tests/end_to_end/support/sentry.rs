use httpmock::{
    Method::GET,
    Mock,
    MockServer,
};
use serde_json::json;

use crate::support::sandbox::parse_date;

pub const SENTRY_AUTH_TOKEN: &str = "sntrys_end_to_end";

pub const SENTRY_ORGANIZATION: &str = "example";

const ISSUES_PATH: &str = "/organizations/example/issues/";

const SECOND_PAGE_CURSOR: &str = "1720000000000:0:0";

/// Three unresolved issues assigned to the user, one per substatus.
const ISSUES_RESPONSE: &str = r#"[
  {
    "id": "42",
    "shortId": "SWELOG-42",
    "title": "TypeError: cannot read  properties of undefined",
    "permalink": "https://example.sentry.io/issues/42/",
    "status": "unresolved",
    "substatus": "escalating",
    "firstSeen": "2026-07-01T10:00:00Z",
    "lastSeen": "2026-07-04T15:30:00Z",
    "project": { "id": "1", "name": "Swelog", "slug": "swelog" }
  },
  {
    "id": "41",
    "shortId": "SWELOG-41",
    "title": "ConnectionError: vault is unreachable",
    "permalink": "https://example.sentry.io/issues/41/",
    "status": "unresolved",
    "substatus": "regressed",
    "firstSeen": "2026-06-30T10:00:00Z",
    "lastSeen": "2026-07-04T09:00:00Z",
    "project": { "id": "1", "name": "Swelog", "slug": "swelog" }
  },
  {
    "id": "40",
    "shortId": "SWELOG-40",
    "title": "DeprecationWarning: the llm config key moved",
    "permalink": "https://example.sentry.io/issues/40/",
    "status": "unresolved",
    "substatus": "ongoing",
    "firstSeen": "2026-06-20T10:00:00Z",
    "lastSeen": "2026-07-04T10:00:00Z",
    "project": { "id": "1", "name": "Swelog", "slug": "swelog" }
  }
]"#;

const FIRST_PAGE_RESPONSE: &str = r#"[
  {
    "id": "42",
    "shortId": "SWELOG-42",
    "title": "TypeError: cannot read  properties of undefined",
    "permalink": "https://example.sentry.io/issues/42/",
    "status": "unresolved",
    "substatus": "escalating"
  }
]"#;

const SECOND_PAGE_RESPONSE: &str = r#"[
  {
    "id": "41",
    "shortId": "SWELOG-41",
    "title": "ConnectionError: vault is unreachable",
    "permalink": "https://example.sentry.io/issues/41/",
    "status": "unresolved",
    "substatus": "regressed"
  },
  {
    "id": "40",
    "shortId": "SWELOG-40",
    "title": "DeprecationWarning: the llm config key moved",
    "permalink": "https://example.sentry.io/issues/40/",
    "status": "unresolved",
    "substatus": "ongoing"
  }
]"#;

const NO_ISSUES_RESPONSE: &str = "[]";

/// Two resolved issues assigned to the user. Only `SWELOG-39` is resolved by
/// hand on the date; Sentry resolves `SWELOG-38` by age.
const RESOLVED_ISSUES_RESPONSE: &str = r#"[
  {
    "id": "39",
    "shortId": "SWELOG-39",
    "title": "KeyError: 'vault_path'",
    "permalink": "https://example.sentry.io/issues/39/",
    "status": "resolved",
    "substatus": null
  },
  {
    "id": "38",
    "shortId": "SWELOG-38",
    "title": "TimeoutError: the summary request timed out",
    "permalink": "https://example.sentry.io/issues/38/",
    "status": "resolved",
    "substatus": null
  }
]"#;

pub const SENTRY_SECTION: &str = "## Sentry

### Escalating

- [SWELOG-42](https://example.sentry.io/issues/42/) TypeError: cannot read properties of undefined

### Regressed

- [SWELOG-41](https://example.sentry.io/issues/41/) ConnectionError: vault is unreachable

### Ongoing

- [SWELOG-40](https://example.sentry.io/issues/40/) DeprecationWarning: the llm config key moved";

pub const SENTRY_SECTION_WITH_RESOLVED_ISSUES: &str = "## Sentry

### Escalating

- [SWELOG-42](https://example.sentry.io/issues/42/) TypeError: cannot read properties of undefined

### Regressed

- [SWELOG-41](https://example.sentry.io/issues/41/) ConnectionError: vault is unreachable

### Ongoing

- [SWELOG-40](https://example.sentry.io/issues/40/) DeprecationWarning: the llm config key moved

### Resolved

- [SWELOG-39](https://example.sentry.io/issues/39/) KeyError: 'vault_path'";

pub struct SentryMocks<'server> {
    pub issue_pages: Vec<Mock<'server>>,
    pub issue_activities: Vec<Mock<'server>>,
}

impl SentryMocks<'_> {
    pub fn assert_every_endpoint_was_called(&self) {
        self.issue_pages
            .iter()
            .chain(&self.issue_activities)
            .for_each(Mock::assert);
    }
}

pub fn mock_sentry_with_issues_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> SentryMocks<'server> {
    let unresolved_issues = mock_unresolved_issues_on(server, activity_date, ISSUES_RESPONSE);

    let resolved_issues = mock_resolved_issues(server, NO_ISSUES_RESPONSE);

    SentryMocks { issue_pages: vec![unresolved_issues, resolved_issues], issue_activities: vec![] }
}

pub fn mock_sentry_with_resolved_issues_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> SentryMocks<'server> {
    let unresolved_issues = mock_unresolved_issues_on(server, activity_date, ISSUES_RESPONSE);

    let resolved_issues = mock_resolved_issues(server, RESOLVED_ISSUES_RESPONSE);

    SentryMocks {
        issue_pages: vec![unresolved_issues, resolved_issues],
        issue_activities: mock_resolution_activities_on(server, activity_date),
    }
}

pub fn mock_sentry_with_no_issues_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> SentryMocks<'server> {
    let unresolved_issues = mock_unresolved_issues_on(server, activity_date, NO_ISSUES_RESPONSE);

    let resolved_issues = mock_resolved_issues(server, NO_ISSUES_RESPONSE);

    SentryMocks { issue_pages: vec![unresolved_issues, resolved_issues], issue_activities: vec![] }
}

pub fn mock_sentry_with_current_issues(server: &MockServer) -> SentryMocks<'_> {
    let unresolved_issues = mock_current_unresolved_issues(server);

    let resolved_issues = mock_resolved_issues(server, NO_ISSUES_RESPONSE);

    SentryMocks { issue_pages: vec![unresolved_issues, resolved_issues], issue_activities: vec![] }
}

pub fn mock_sentry_with_current_issues_and_resolutions_on<'server>(
    server: &'server MockServer,
    resolution_date: &str,
) -> SentryMocks<'server> {
    let unresolved_issues = mock_current_unresolved_issues(server);

    let resolved_issues = mock_resolved_issues(server, RESOLVED_ISSUES_RESPONSE);

    SentryMocks {
        issue_pages: vec![unresolved_issues, resolved_issues],
        issue_activities: mock_resolution_activities_on(server, resolution_date),
    }
}

pub fn mock_sentry_with_two_pages_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> SentryMocks<'server> {
    let next_page_link = format!(
        r#"<{}{ISSUES_PATH}?cursor={SECOND_PAGE_CURSOR}>; rel="next"; results="true""#,
        server.base_url()
    );

    let first_page = server.mock(|when, then| {
        when.method(GET)
            .path(ISSUES_PATH)
            .query_param("query", "assigned:me is:unresolved")
            .query_param("start", start_of(activity_date))
            .query_param("end", start_of_the_day_after(activity_date))
            .query_param_missing("cursor")
            .header("authorization", format!("Bearer {SENTRY_AUTH_TOKEN}"));

        then.status(200)
            .header("content-type", "application/json")
            .header("link", next_page_link)
            .body(FIRST_PAGE_RESPONSE);
    });

    let resolved_issues = mock_resolved_issues(server, NO_ISSUES_RESPONSE);

    let second_page = server.mock(|when, then| {
        when.method(GET)
            .path(ISSUES_PATH)
            .query_param("query", "assigned:me is:unresolved")
            .query_param("cursor", SECOND_PAGE_CURSOR)
            .header("authorization", format!("Bearer {SENTRY_AUTH_TOKEN}"));

        then.status(200)
            .header("content-type", "application/json")
            .header("link", last_page_link(server))
            .body(SECOND_PAGE_RESPONSE);
    });

    SentryMocks {
        issue_pages: vec![first_page, second_page, resolved_issues],
        issue_activities: vec![],
    }
}

pub fn mock_sentry_rejecting_the_token(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(GET)
            .path(ISSUES_PATH);

        then.status(401)
            .header("content-type", "application/json")
            .body(r#"{ "detail": "Invalid token" }"#);
    })
}

fn mock_unresolved_issues_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
    response: &str,
) -> Mock<'server> {
    server.mock(|when, then| {
        when.method(GET)
            .path(ISSUES_PATH)
            .query_param("query", "assigned:me is:unresolved")
            .query_param("project", "-1")
            .query_param("limit", "100")
            .query_param("collapse", "stats")
            .query_param("start", start_of(activity_date))
            .query_param("end", start_of_the_day_after(activity_date))
            .query_param_missing("cursor")
            .header("authorization", format!("Bearer {SENTRY_AUTH_TOKEN}"));

        then.status(200)
            .header("content-type", "application/json")
            .header("link", last_page_link(server))
            .body(response);
    })
}

fn mock_current_unresolved_issues(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(GET)
            .path(ISSUES_PATH)
            .query_param("query", "assigned:me is:unresolved")
            .query_param_missing("start")
            .query_param_missing("end")
            .query_param_missing("cursor")
            .header("authorization", format!("Bearer {SENTRY_AUTH_TOKEN}"));

        then.status(200)
            .header("content-type", "application/json")
            .body(ISSUES_RESPONSE);
    })
}

/// Sentry's date window only matches issues that received events in it, so the
/// resolved issues are searched without one.
fn mock_resolved_issues<'server>(server: &'server MockServer, response: &str) -> Mock<'server> {
    server.mock(|when, then| {
        when.method(GET)
            .path(ISSUES_PATH)
            .query_param("query", "assigned:me is:resolved")
            .query_param("project", "-1")
            .query_param("limit", "100")
            .query_param("collapse", "stats")
            .query_param_missing("start")
            .query_param_missing("end")
            .query_param_missing("cursor")
            .header("authorization", format!("Bearer {SENTRY_AUTH_TOKEN}"));

        then.status(200)
            .header("content-type", "application/json")
            .header("link", last_page_link(server))
            .body(response);
    })
}

fn mock_resolution_activities_on<'server>(
    server: &'server MockServer,
    resolution_date: &str,
) -> Vec<Mock<'server>> {
    let resolved_on = parse_date(resolution_date).format("%Y-%m-%d");

    let resolved_by_hand_activity = json!({
        "id": "39",
        "shortId": "SWELOG-39",
        "status": "resolved",
        "activity": [
            {
                "id": "3",
                "type": "set_resolved",
                "user": { "id": "1", "name": "Swelog User" },
                "data": {},
                "dateCreated": format!("{resolved_on}T15:30:00.123456Z")
            },
            {
                "id": "2",
                "type": "assigned",
                "user": { "id": "1", "name": "Swelog User" },
                "data": { "assignee": "1", "assigneeType": "user" },
                "dateCreated": "2026-06-30T10:00:00Z"
            }
        ]
    });

    let resolved_by_age_activity = json!({
        "id": "38",
        "shortId": "SWELOG-38",
        "status": "resolved",
        "activity": [
            {
                "id": "5",
                "type": "set_resolved_by_age",
                "user": null,
                "data": { "age": 168 },
                "dateCreated": format!("{resolved_on}T08:00:00Z")
            },
            {
                "id": "4",
                "type": "set_resolved",
                "user": { "id": "1", "name": "Swelog User" },
                "data": {},
                "dateCreated": "2026-06-30T10:00:00Z"
            }
        ]
    });

    vec![
        mock_issue_activity(server, "39", &resolved_by_hand_activity.to_string()),
        mock_issue_activity(server, "38", &resolved_by_age_activity.to_string()),
    ]
}

fn mock_issue_activity<'server>(
    server: &'server MockServer,
    issue_id: &str,
    response: &str,
) -> Mock<'server> {
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{ISSUES_PATH}{issue_id}/"))
            .query_param("collapse", "release")
            .query_param("collapse", "stats")
            .query_param("collapse", "tags")
            .header("authorization", format!("Bearer {SENTRY_AUTH_TOKEN}"));

        then.status(200)
            .header("content-type", "application/json")
            .body(response);
    })
}

fn last_page_link(server: &MockServer) -> String {
    format!(r#"<{}{ISSUES_PATH}?cursor=0:0:0>; rel="next"; results="false""#, server.base_url())
}

/// The sandbox runs with `TZ=UTC`, so a local day starts at midnight UTC.
fn start_of(date: &str) -> String {
    format!("{}T00:00:00", parse_date(date).format("%Y-%m-%d"))
}

fn start_of_the_day_after(date: &str) -> String {
    let following_date = parse_date(date)
        .succ_opt()
        .expect("test date should have a next day");

    format!("{}T00:00:00", following_date.format("%Y-%m-%d"))
}
