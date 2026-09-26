use chrono::NaiveDate;

use super::{
    get_pull_request_reviews_endpoint_path,
    get_reviewed_pr_candidates_search_query,
    has_review_submitted_on,
    keep_prs_reviewed_on,
    parse_pull_request_reviews_response_text,
    structs::Review,
};
use crate::issues::{
    Issue,
    PullRequest,
};

fn test_activity_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 7, 4).expect("test date should be valid")
}

fn get_mock_pull_request(number: u64) -> Issue {
    Issue {
        title: format!("PR {number}"),
        number,
        repository_url: "https://api.github.com/repos/example/swelog".to_string(),
        pull_request: PullRequest {
            html_url: format!("https://github.com/example/swelog/pull/{number}"),
        },
    }
}

fn parse_reviews(response_body: &str) -> Vec<Review> {
    parse_pull_request_reviews_response_text(response_body)
        .expect("GitHub pull request reviews response should parse")
}

const REVIEW_BY_OCTOCAT_ON_ACTIVITY_DATE: &str = r#"
    [
      {
        "user": { "login": "octocat" },
        "state": "APPROVED",
        "submitted_at": "2026-07-04T15:30:00Z"
      }
    ]
"#;

const REVIEW_BY_OCTOCAT_THE_DAY_BEFORE: &str = r#"
    [
      {
        "user": { "login": "octocat" },
        "state": "COMMENTED",
        "submitted_at": "2026-07-03T23:59:59Z"
      }
    ]
"#;

#[test]
fn reviewed_pr_candidates_search_query_excludes_your_own_pull_requests() {
    let activity_date = test_activity_date();

    let search_query = get_reviewed_pr_candidates_search_query("octocat", activity_date);

    assert_eq!(search_query, "reviewed-by:octocat -author:octocat is:pr updated:>=2026-07-04");
}

#[test]
fn pull_request_reviews_endpoint_path_points_at_the_pull_requests_repository() {
    let pull_request = get_mock_pull_request(42);

    let endpoint_path = get_pull_request_reviews_endpoint_path(&pull_request);

    assert_eq!(endpoint_path, "repos/example/swelog/pulls/42/reviews");
}

#[test]
fn parse_pull_request_reviews_response_text_accepts_pending_reviews_and_deleted_users() {
    let response_body = r#"
        [
          { "user": { "login": "octocat" }, "state": "PENDING" },
          { "user": null, "state": "APPROVED", "submitted_at": "2026-07-04T15:30:00Z" }
        ]
    "#;

    let reviews = parse_reviews(response_body);

    assert_eq!(reviews.len(), 2);
}

#[test]
fn parse_pull_request_reviews_response_text_fails_when_the_response_is_not_a_list() {
    let response_body = r#"{ "message": "Not Found" }"#;

    parse_pull_request_reviews_response_text(response_body)
        .expect_err("a response that is not a list of reviews should fail");
}

#[test]
fn has_review_submitted_on_is_true_for_your_review_on_the_activity_date() {
    let reviews = parse_reviews(REVIEW_BY_OCTOCAT_ON_ACTIVITY_DATE);

    assert!(has_review_submitted_on(&reviews, "octocat", test_activity_date()));
}

#[test]
fn has_review_submitted_on_ignores_the_case_of_your_username() {
    let reviews = parse_reviews(REVIEW_BY_OCTOCAT_ON_ACTIVITY_DATE);

    assert!(has_review_submitted_on(&reviews, "OctoCat", test_activity_date()));
}

#[test]
fn has_review_submitted_on_is_false_for_your_review_on_another_date() {
    let reviews = parse_reviews(REVIEW_BY_OCTOCAT_THE_DAY_BEFORE);

    assert!(!has_review_submitted_on(&reviews, "octocat", test_activity_date()));
}

#[test]
fn has_review_submitted_on_is_false_for_someone_elses_review() {
    let reviews = parse_reviews(REVIEW_BY_OCTOCAT_ON_ACTIVITY_DATE);

    assert!(!has_review_submitted_on(&reviews, "hubot", test_activity_date()));
}

#[test]
fn has_review_submitted_on_is_false_for_your_pending_review() {
    let response_body = r#"[{ "user": { "login": "octocat" }, "state": "PENDING" }]"#;

    let reviews = parse_reviews(response_body);

    assert!(!has_review_submitted_on(&reviews, "octocat", test_activity_date()));
}

#[test]
fn keep_prs_reviewed_on_drops_pull_requests_you_reviewed_on_another_date() {
    let reviewed_pr = get_mock_pull_request(50);

    let previously_reviewed_pr = get_mock_pull_request(51);

    let candidate_prs_with_reviews = vec![
        (reviewed_pr, parse_reviews(REVIEW_BY_OCTOCAT_ON_ACTIVITY_DATE)),
        (previously_reviewed_pr, parse_reviews(REVIEW_BY_OCTOCAT_THE_DAY_BEFORE)),
    ];

    let reviewed_prs =
        keep_prs_reviewed_on(candidate_prs_with_reviews, "octocat", test_activity_date());

    assert_eq!(reviewed_prs, vec![get_mock_pull_request(50)]);
}
