use chrono::{
    NaiveDate,
    SubsecRound,
    TimeZone,
    Utc,
};

use super::*;
use crate::{
    activity::get_activity_window,
    issue_activity::structs::SentryActivityType,
};

fn get_mock_activity_window() -> ActivityWindow {
    let activity_date = NaiveDate::from_ymd_opt(2026, 7, 4).expect("test date should be valid");

    get_activity_window(activity_date, &Utc).expect("window should be built")
}

fn get_mock_activity_entry(
    activity_type: SentryActivityType,
    hour: u32,
    day: u32,
) -> SentryActivityEntry {
    let date_created = Utc
        .with_ymd_and_hms(2026, 7, day, hour, 0, 0)
        .single()
        .expect("test instant should be valid");

    SentryActivityEntry { activity_type, date_created }
}

#[test]
fn get_issue_details_endpoint_path_is_scoped_to_the_organization_and_issue() {
    assert_eq!(get_issue_details_endpoint_path("acme", "42"), "organizations/acme/issues/42/");
}

#[test]
fn parse_issue_activity_response_text_extracts_the_activity_entries() {
    let response_body = r#"
            {
              "id": "6837811207",
              "shortId": "SENTRY-4EDG",
              "status": "resolved",
              "activity": [
                {
                  "id": "1000190086",
                  "type": "set_resolved",
                  "user": { "id": "4828697", "name": "Rudra Patel" },
                  "data": {},
                  "source": "web",
                  "dateCreated": "2026-11-06T21:05:36.817868Z"
                },
                {
                  "id": "970598411",
                  "type": "assigned",
                  "user": { "id": "4828697", "name": "Rudra Patel" },
                  "data": { "assignee": "4828697", "assigneeType": "user" },
                  "source": "web",
                  "dateCreated": "2026-11-03T17:55:43.048052Z"
                },
                {
                  "id": "23722",
                  "type": "set_resolved_by_age",
                  "user": null,
                  "data": { "age": 168 },
                  "source": "backfill:activity",
                  "dateCreated": "2025-11-13T15:10:17.450804Z"
                }
              ]
            }
        "#;

    let activity_entries =
        parse_issue_activity_response_text(response_body).expect("issue details should parse");

    let activity_types: Vec<SentryActivityType> = activity_entries
        .iter()
        .map(|activity_entry| activity_entry.activity_type)
        .collect();

    assert_eq!(
        activity_types,
        vec![SentryActivityType::SetResolved, SentryActivityType::Other, SentryActivityType::Other]
    );

    let resolved_at = Utc
        .with_ymd_and_hms(2026, 11, 6, 21, 5, 36)
        .single()
        .expect("test instant should be valid");

    assert_eq!(
        activity_entries
            .first()
            .map(|activity_entry| activity_entry
                .date_created
                .trunc_subsecs(0)),
        Some(resolved_at)
    );
}

#[test]
fn parse_issue_activity_response_text_reads_release_and_commit_resolutions() {
    let response_body = r#"
            {
              "activity": [
                { "type": "set_resolved_in_release", "dateCreated": "2026-07-04T12:00:00Z" },
                { "type": "set_resolved_in_commit", "dateCreated": "2026-07-04T13:00:00Z" },
                { "type": "set_resolved_in_pull_request", "dateCreated": "2026-07-04T14:00:00Z" }
              ]
            }
        "#;

    let activity_entries =
        parse_issue_activity_response_text(response_body).expect("issue details should parse");

    let activity_types: Vec<SentryActivityType> = activity_entries
        .iter()
        .map(|activity_entry| activity_entry.activity_type)
        .collect();

    assert_eq!(
        activity_types,
        vec![
            SentryActivityType::SetResolvedInRelease,
            SentryActivityType::SetResolvedInCommit,
            SentryActivityType::Other,
        ]
    );
}

#[test]
fn parse_issue_activity_response_text_fails_without_activity() {
    parse_issue_activity_response_text(r#"{ "id": "42" }"#)
        .expect_err("a response without activity should fail");
}

#[test]
fn was_resolved_during_finds_a_resolution_inside_the_window() {
    let activity_entries = vec![
        get_mock_activity_entry(SentryActivityType::Other, 9, 4),
        get_mock_activity_entry(SentryActivityType::SetResolved, 15, 4),
    ];

    assert!(was_resolved_during(&activity_entries, &get_mock_activity_window()));
}

#[test]
fn was_resolved_during_counts_release_and_commit_resolutions() {
    let release_resolution =
        vec![get_mock_activity_entry(SentryActivityType::SetResolvedInRelease, 10, 4)];

    let commit_resolution =
        vec![get_mock_activity_entry(SentryActivityType::SetResolvedInCommit, 10, 4)];

    assert!(was_resolved_during(&release_resolution, &get_mock_activity_window()));

    assert!(was_resolved_during(&commit_resolution, &get_mock_activity_window()));
}

#[test]
fn was_resolved_during_ignores_a_resolution_on_another_date() {
    let activity_entries = vec![
        get_mock_activity_entry(SentryActivityType::SetResolved, 23, 3),
        get_mock_activity_entry(SentryActivityType::SetResolved, 0, 5),
    ];

    assert!(!was_resolved_during(&activity_entries, &get_mock_activity_window()));
}

#[test]
fn was_resolved_during_ignores_other_activity_inside_the_window() {
    let activity_entries = vec![get_mock_activity_entry(SentryActivityType::Other, 15, 4)];

    assert!(!was_resolved_during(&activity_entries, &get_mock_activity_window()));
}
