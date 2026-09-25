use chrono::NaiveDate;

use super::{
    get_closed_prs_search_query,
    get_merged_prs_search_query,
    get_opened_prs_search_query,
    parse_search_issues_response_text,
};
use crate::issues::{
    Issue,
    PullRequest,
};

fn test_activity_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 7, 4).expect("test date should be valid")
}

#[test]
fn opened_prs_search_query_filters_to_activity_date() {
    let activity_date = test_activity_date();

    let search_query = get_opened_prs_search_query("octocat", activity_date);

    assert_eq!(search_query, "author:octocat is:pr created:2026-07-04");
}

#[test]
fn merged_prs_search_query_filters_to_activity_date() {
    let activity_date = test_activity_date();

    let search_query = get_merged_prs_search_query("octocat", activity_date);

    assert_eq!(search_query, "author:octocat is:pr merged:2026-07-04");
}

#[test]
fn closed_prs_search_query_filters_to_unmerged_pull_requests_closed_on_activity_date() {
    let activity_date = test_activity_date();

    let search_query = get_closed_prs_search_query("octocat", activity_date);

    assert_eq!(search_query, "author:octocat is:pr is:unmerged closed:2026-07-04");
}

#[test]
fn parse_search_issues_response_text_extracts_issues() {
    let response_body = r#"
            {
              "total_count": 2,
              "incomplete_results": false,
              "items": [
                {
                  "title": "Add daily summary command",
                  "number": 42,
                  "repository_url": "https://api.github.com/repos/example/swelog",
                  "pull_request": {
                    "html_url": "https://github.com/example/swelog/pull/42"
                  }
                },
                {
                  "title": "Fix work log formatting",
                  "number": 43,
                  "repository_url": "https://api.github.com/repos/example/swelog",
                  "pull_request": {
                    "html_url": "https://github.com/example/swelog/pull/43"
                  }
                }
              ]
            }
        "#;

    let issues = parse_search_issues_response_text(response_body)
        .expect("GitHub search issues response should parse");

    assert_eq!(issues.len(), 2);

    let first_issue = &issues[0];

    let second_issue = &issues[1];

    let expected_first_issue = Issue {
        title: "Add daily summary command".to_string(),
        number: 42,
        repository_url: "https://api.github.com/repos/example/swelog".to_string(),
        pull_request: PullRequest {
            html_url: "https://github.com/example/swelog/pull/42".to_string(),
        },
    };

    let expected_second_issue = Issue {
        title: "Fix work log formatting".to_string(),
        number: 43,
        repository_url: "https://api.github.com/repos/example/swelog".to_string(),
        pull_request: PullRequest {
            html_url: "https://github.com/example/swelog/pull/43".to_string(),
        },
    };

    assert_eq!(*first_issue, expected_first_issue);

    assert_eq!(*second_issue, expected_second_issue);
}

#[test]
fn parse_search_issues_response_text_extracts_empty_issues() {
    let response_body = r#"
            {
              "total_count": 0,
              "incomplete_results": false,
              "items": []
            }
        "#;

    let issues = parse_search_issues_response_text(response_body)
        .expect("empty GitHub search issues response should parse");

    assert_eq!(issues.len(), 0);
}

#[test]
fn parse_search_issues_response_text_fails_when_required_field_is_missing() {
    let response_body = r#"
            {
              "items": [
                {
                  "title": "Add daily summary command",
                  "number": 42
                }
              ]
            }
        "#;

    parse_search_issues_response_text(response_body)
        .expect_err("missing pull_request field should fail");
}
