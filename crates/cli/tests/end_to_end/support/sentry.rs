use httpmock::{
    Method::GET,
    Mock,
    MockServer,
};

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

pub const SENTRY_SECTION: &str = "## Sentry

### Escalating

- [SWELOG-42](https://example.sentry.io/issues/42/) TypeError: cannot read properties of undefined

### Regressed

- [SWELOG-41](https://example.sentry.io/issues/41/) ConnectionError: vault is unreachable

### Ongoing

- [SWELOG-40](https://example.sentry.io/issues/40/) DeprecationWarning: the llm config key moved";

pub struct SentryMocks<'server> {
    pub issue_pages: Vec<Mock<'server>>,
}

impl SentryMocks<'_> {
    pub fn assert_every_endpoint_was_called(&self) {
        self.issue_pages
            .iter()
            .for_each(Mock::assert);
    }
}

pub fn mock_sentry_with_issues_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> SentryMocks<'server> {
    SentryMocks { issue_pages: vec![mock_issues_on(server, activity_date, ISSUES_RESPONSE)] }
}

pub fn mock_sentry_with_no_issues_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> SentryMocks<'server> {
    SentryMocks { issue_pages: vec![mock_issues_on(server, activity_date, NO_ISSUES_RESPONSE)] }
}

pub fn mock_sentry_with_current_issues(server: &MockServer) -> SentryMocks<'_> {
    let issues = server.mock(|when, then| {
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
    });

    SentryMocks { issue_pages: vec![issues] }
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

    SentryMocks { issue_pages: vec![first_page, second_page] }
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

fn mock_issues_on<'server>(
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
