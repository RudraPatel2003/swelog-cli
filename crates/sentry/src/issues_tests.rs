use chrono::{
    NaiveDate,
    Utc,
};

use super::*;
use crate::issues::structs::SentryIssueSubstatus;

fn get_mock_activity_window() -> ActivityWindow {
    let activity_date = NaiveDate::from_ymd_opt(2026, 7, 4).expect("test date should be valid");

    get_activity_window(activity_date, &Utc).expect("window should be built")
}

#[test]
fn get_issues_endpoint_path_is_scoped_to_the_organization() {
    assert_eq!(get_issues_endpoint_path("acme"), "organizations/acme/issues/");
}

#[test]
fn get_issues_query_parameters_asks_for_the_unresolved_issues_assigned_to_the_user() {
    let query_parameters = get_issues_query_parameters(None, None);

    assert_eq!(
        query_parameters,
        vec![
            ("query", "assigned:me is:unresolved".to_string()),
            ("project", "-1".to_string()),
            ("limit", "100".to_string()),
            ("collapse", "stats".to_string()),
        ]
    );
}

#[test]
fn get_issues_query_parameters_bounds_the_search_to_the_activity_window() {
    let activity_window = get_mock_activity_window();

    let query_parameters = get_issues_query_parameters(Some(&activity_window), None);

    assert!(query_parameters.contains(&("start", "2026-07-04T00:00:00".to_string())));

    assert!(query_parameters.contains(&("end", "2026-07-05T00:00:00".to_string())));
}

#[test]
fn get_issues_query_parameters_continues_from_the_cursor() {
    let query_parameters = get_issues_query_parameters(None, Some("1420837533:0:0"));

    assert!(query_parameters.contains(&("cursor", "1420837533:0:0".to_string())));
}

#[test]
fn ensure_cursor_advanced_accepts_a_new_cursor() {
    ensure_cursor_advanced(Some("1:0:0"), "2:0:0").expect("a new cursor should be accepted");

    ensure_cursor_advanced(None, "1:0:0").expect("the first cursor should be accepted");
}

#[test]
fn ensure_cursor_advanced_rejects_a_repeated_cursor() {
    ensure_cursor_advanced(Some("1:0:0"), "1:0:0").expect_err("a repeated cursor should fail");
}

#[test]
fn parse_issues_response_text_extracts_issues() {
    let response_body = r#"
            [
              {
                "id": "1",
                "shortId": "PUMP-STATION-1",
                "title": "This is an example Python exception",
                "culprit": "raven.scripts.runner in main",
                "permalink": "https://sentry.io/the-interstellar-jurisdiction/pump-station/issues/1/",
                "status": "unresolved",
                "substatus": "escalating",
                "firstSeen": "2018-11-06T21:19:55Z",
                "lastSeen": "2018-12-06T21:19:55Z",
                "count": "150",
                "userCount": 0,
                "level": "error",
                "project": { "id": "2", "name": "Pump Station", "slug": "pump-station" },
                "assignedTo": { "type": "user", "id": "1", "name": "John Doe" }
              },
              {
                "id": "2",
                "shortId": "PUMP-STATION-2",
                "title": "ConnectionError: database unreachable",
                "permalink": "https://sentry.io/the-interstellar-jurisdiction/pump-station/issues/2/",
                "status": "unresolved",
                "substatus": "ongoing",
                "firstSeen": "2018-11-07T08:00:00Z",
                "lastSeen": "2018-12-07T08:00:00Z"
              }
            ]
        "#;

    let issues =
        parse_issues_response_text(response_body).expect("Sentry issues response should parse");

    let expected_first_issue = SentryIssue {
        short_id: "PUMP-STATION-1".to_string(),
        title: "This is an example Python exception".to_string(),
        permalink: "https://sentry.io/the-interstellar-jurisdiction/pump-station/issues/1/"
            .to_string(),
        substatus: Some(SentryIssueSubstatus::Escalating),
    };

    let expected_second_issue = SentryIssue {
        short_id: "PUMP-STATION-2".to_string(),
        title: "ConnectionError: database unreachable".to_string(),
        permalink: "https://sentry.io/the-interstellar-jurisdiction/pump-station/issues/2/"
            .to_string(),
        substatus: Some(SentryIssueSubstatus::Ongoing),
    };

    assert_eq!(issues, vec![expected_first_issue, expected_second_issue]);
}

#[test]
fn parse_issues_response_text_extracts_empty_issues() {
    let issues = parse_issues_response_text("[]").expect("empty response should parse");

    assert_eq!(issues.len(), 0);
}

#[test]
fn parse_issues_response_text_maps_an_unknown_substatus_to_other() {
    let response_body = r#"
            [
              {
                "shortId": "PUMP-STATION-3",
                "title": "Something new",
                "permalink": "https://sentry.io/issues/3/",
                "status": "unresolved",
                "substatus": "archived_until_escalating"
              }
            ]
        "#;

    let issues = parse_issues_response_text(response_body).expect("response should parse");

    assert_eq!(
        issues
            .first()
            .map(|issue| issue.substatus),
        Some(Some(SentryIssueSubstatus::Other))
    );
}

#[test]
fn parse_issues_response_text_accepts_a_missing_substatus() {
    let response_body = r#"
            [
              {
                "shortId": "PUMP-STATION-3",
                "title": "Something new",
                "permalink": "https://sentry.io/issues/3/",
                "status": "unresolved",
                "substatus": null
              }
            ]
        "#;

    let issues = parse_issues_response_text(response_body).expect("response should parse");

    assert_eq!(
        issues
            .first()
            .map(|issue| issue.substatus),
        Some(None)
    );
}

#[test]
fn parse_issues_response_text_fails_when_required_field_is_missing() {
    let response_body = r#"
            [
              {
                "shortId": "PUMP-STATION-1",
                "title": "This is an example Python exception"
              }
            ]
        "#;

    parse_issues_response_text(response_body).expect_err("missing permalink should fail");
}
