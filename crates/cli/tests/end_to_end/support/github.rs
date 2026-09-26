use httpmock::{
    Method::GET,
    Mock,
    MockServer,
};

use crate::support::sandbox::GITHUB_TOKEN;

pub const GITHUB_USERNAME: &str = "octocat";

const USER_RESPONSE: &str = r#"{ "login": "octocat", "id": 1 }"#;

const OPENED_PRS_RESPONSE: &str = r#"{
  "total_count": 1,
  "incomplete_results": false,
  "items": [
    {
      "title": "Add end-to-end tests",
      "number": 42,
      "repository_url": "https://api.github.com/repos/example/swelog",
      "pull_request": { "html_url": "https://github.com/example/swelog/pull/42" }
    }
  ]
}"#;

const MERGED_PRS_RESPONSE: &str = r#"{
  "total_count": 1,
  "incomplete_results": false,
  "items": [
    {
      "title": "Fix work file formatting",
      "number": 43,
      "repository_url": "https://api.github.com/repos/example/swelog",
      "pull_request": { "html_url": "https://github.com/example/swelog/pull/43" }
    }
  ]
}"#;

const CLOSED_PRS_RESPONSE: &str = r#"{
  "total_count": 1,
  "incomplete_results": false,
  "items": [
    {
      "title": "Try a separate formatting crate",
      "number": 44,
      "repository_url": "https://api.github.com/repos/example/swelog",
      "pull_request": { "html_url": "https://github.com/example/swelog/pull/44" }
    }
  ]
}"#;

const REVIEWED_PR_CANDIDATES_RESPONSE: &str = r#"{
  "total_count": 2,
  "incomplete_results": false,
  "items": [
    {
      "title": "Add Linear integration",
      "number": 50,
      "repository_url": "https://api.github.com/repos/example/swelog",
      "pull_request": { "html_url": "https://github.com/example/swelog/pull/50" }
    },
    {
      "title": "Speed up weekly summaries",
      "number": 51,
      "repository_url": "https://api.github.com/repos/example/swelog",
      "pull_request": { "html_url": "https://github.com/example/swelog/pull/51" }
    }
  ]
}"#;

const REVIEWED_PR_NUMBER: u64 = 50;

const PREVIOUSLY_REVIEWED_PR_NUMBER: u64 = 51;

const PREVIOUS_REVIEWS_RESPONSE: &str = r#"[
  {
    "user": { "login": "octocat" },
    "state": "COMMENTED",
    "submitted_at": "2026-06-01T10:00:00Z"
  },
  {
    "user": { "login": "hubot" },
    "state": "APPROVED",
    "submitted_at": "2026-06-02T10:00:00Z"
  }
]"#;

const NO_PRS_RESPONSE: &str = r#"{ "total_count": 0, "incomplete_results": false, "items": [] }"#;

pub const GITHUB_SECTION: &str = r#"## GitHub

### Opened

- "Add end-to-end tests" ([#42](https://github.com/example/swelog/pull/42)) in [example/swelog](https://github.com/example/swelog)

### Merged

- "Fix work file formatting" ([#43](https://github.com/example/swelog/pull/43)) in [example/swelog](https://github.com/example/swelog)

### Closed

- "Try a separate formatting crate" ([#44](https://github.com/example/swelog/pull/44)) in [example/swelog](https://github.com/example/swelog)

### Reviewed

- "Add Linear integration" ([#50](https://github.com/example/swelog/pull/50)) in [example/swelog](https://github.com/example/swelog)"#;

pub struct GitHubMocks<'server> {
    pub user: Mock<'server>,
    pub opened_prs: Mock<'server>,
    pub merged_prs: Mock<'server>,
    pub closed_prs: Mock<'server>,
    pub reviewed_pr_candidates: Mock<'server>,
    pub pull_request_reviews: Vec<Mock<'server>>,
}

pub fn mock_github_with_activity_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> GitHubMocks<'server> {
    let reviews_on_activity_date_response = get_reviews_on_activity_date_response(activity_date);

    let pull_request_reviews = vec![
        mock_pull_request_reviews(server, REVIEWED_PR_NUMBER, &reviews_on_activity_date_response),
        mock_pull_request_reviews(server, PREVIOUSLY_REVIEWED_PR_NUMBER, PREVIOUS_REVIEWS_RESPONSE),
    ];

    GitHubMocks {
        user: mock_user(server),
        opened_prs: mock_search(server, &opened_query(activity_date), OPENED_PRS_RESPONSE),
        merged_prs: mock_search(server, &merged_query(activity_date), MERGED_PRS_RESPONSE),
        closed_prs: mock_search(server, &closed_query(activity_date), CLOSED_PRS_RESPONSE),
        reviewed_pr_candidates: mock_search(
            server,
            &reviewed_candidates_query(activity_date),
            REVIEWED_PR_CANDIDATES_RESPONSE,
        ),
        pull_request_reviews,
    }
}

pub fn mock_github_with_no_activity_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> GitHubMocks<'server> {
    GitHubMocks {
        user: mock_user(server),
        opened_prs: mock_search(server, &opened_query(activity_date), NO_PRS_RESPONSE),
        merged_prs: mock_search(server, &merged_query(activity_date), NO_PRS_RESPONSE),
        closed_prs: mock_search(server, &closed_query(activity_date), NO_PRS_RESPONSE),
        reviewed_pr_candidates: mock_search(
            server,
            &reviewed_candidates_query(activity_date),
            NO_PRS_RESPONSE,
        ),
        pull_request_reviews: Vec::new(),
    }
}

impl GitHubMocks<'_> {
    pub fn assert_every_endpoint_was_called(&self) {
        self.user.assert();

        self.opened_prs.assert();

        self.merged_prs.assert();

        self.closed_prs.assert();

        self.reviewed_pr_candidates.assert();

        self.pull_request_reviews.iter().for_each(Mock::assert);
    }
}

pub fn mock_github_rejecting_the_token(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(GET).path("/user");

        then.status(401)
            .header("content-type", "application/json")
            .body(r#"{ "message": "Bad credentials" }"#);
    })
}

fn mock_user(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(GET).path("/user").header("authorization", format!("Bearer {GITHUB_TOKEN}"));

        then.status(200).header("content-type", "application/json").body(USER_RESPONSE);
    })
}

fn mock_search<'server>(
    server: &'server MockServer,
    search_query: &str,
    response: &str,
) -> Mock<'server> {
    server.mock(|when, then| {
        when.method(GET)
            .path("/search/issues")
            .query_param("q", search_query)
            .header("authorization", format!("Bearer {GITHUB_TOKEN}"));

        then.status(200).header("content-type", "application/json").body(response);
    })
}

fn mock_pull_request_reviews<'server>(
    server: &'server MockServer,
    pull_request_number: u64,
    response: &str,
) -> Mock<'server> {
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("/repos/example/swelog/pulls/{pull_request_number}/reviews"))
            .header("authorization", format!("Bearer {GITHUB_TOKEN}"));

        then.status(200).header("content-type", "application/json").body(response);
    })
}

fn get_reviews_on_activity_date_response(activity_date: &str) -> String {
    format!(
        r#"[
  {{
    "user": {{ "login": "{GITHUB_USERNAME}" }},
    "state": "APPROVED",
    "submitted_at": "{}T15:30:00Z"
  }}
]"#,
        to_iso_date(activity_date)
    )
}

fn opened_query(activity_date: &str) -> String {
    format!("author:{GITHUB_USERNAME} is:pr created:{}", to_iso_date(activity_date))
}

fn merged_query(activity_date: &str) -> String {
    format!("author:{GITHUB_USERNAME} is:pr merged:{}", to_iso_date(activity_date))
}

fn closed_query(activity_date: &str) -> String {
    format!("author:{GITHUB_USERNAME} is:pr is:unmerged closed:{}", to_iso_date(activity_date))
}

fn reviewed_candidates_query(activity_date: &str) -> String {
    format!(
        "reviewed-by:{GITHUB_USERNAME} -author:{GITHUB_USERNAME} is:pr updated:>={}",
        to_iso_date(activity_date)
    )
}

fn to_iso_date(date: &str) -> String {
    crate::support::sandbox::parse_date(date).format("%Y-%m-%d").to_string()
}
