use chrono::{
    FixedOffset,
    NaiveDate,
    TimeZone,
};

use super::*;
use crate::client::structs::{
    LinearIssueTimestamps,
    LinearStatusType,
};

fn get_mock_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("test date should be valid")
}

/// UTC-05:00, so a late evening timestamp here lands on the next day in UTC.
fn get_mock_timezone() -> FixedOffset {
    FixedOffset::west_opt(18_000).expect("test offset should be valid")
}

fn get_mock_instant(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    get_mock_timezone()
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("test instant should be unambiguous")
        .to_utc()
}

fn get_mock_issue(timestamps: LinearIssueTimestamps) -> LinearIssue {
    get_mock_issue_with_status(LinearStatusType::Started, timestamps)
}

fn get_mock_issue_with_status(
    status_type: LinearStatusType,
    timestamps: LinearIssueTimestamps,
) -> LinearIssue {
    LinearIssue {
        identifier: "ENG-1".to_string(),
        title: "Investigate the outage".to_string(),
        url: "https://linear.app/acme/issue/ENG-1".to_string(),
        status_name: "In Progress".to_string(),
        status_type,
        timestamps,
    }
}

#[test]
fn get_updated_after_filter_starts_a_day_before_the_requested_date() {
    let activity_date = get_mock_date(2026, 8, 17);

    let updated_after = get_updated_after_filter(activity_date).expect("filter should be built");

    assert_eq!(updated_after, "2026-08-16T00:00:00Z");
}

#[test]
fn get_updated_after_filter_crosses_a_year_boundary() {
    let activity_date = get_mock_date(2026, 1, 1);

    let updated_after = get_updated_after_filter(activity_date).expect("filter should be built");

    assert_eq!(updated_after, "2025-12-31T00:00:00Z");
}

#[test]
fn was_issue_worked_on_matches_an_issue_updated_on_the_date() {
    let updated_at = get_mock_instant(2026, 8, 17, 14, 30);

    let timestamps =
        LinearIssueTimestamps { updated_at: Some(updated_at), ..LinearIssueTimestamps::default() };

    let mock_issue = get_mock_issue(timestamps);

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(was_issue_worked_on(&mock_issue, activity_date, &timezone));
}

#[test]
fn was_issue_worked_on_matches_an_issue_completed_on_the_date_but_updated_later() {
    let completed_at = get_mock_instant(2026, 8, 17, 16, 0);

    let updated_at = get_mock_instant(2026, 8, 19, 9, 0);

    let timestamps = LinearIssueTimestamps {
        completed_at: Some(completed_at),
        updated_at: Some(updated_at),
        ..LinearIssueTimestamps::default()
    };

    let mock_issue = get_mock_issue(timestamps);

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(was_issue_worked_on(&mock_issue, activity_date, &timezone));
}

#[test]
fn was_issue_worked_on_matches_an_issue_canceled_on_the_date_but_updated_later() {
    let canceled_at = get_mock_instant(2026, 8, 17, 16, 0);

    let updated_at = get_mock_instant(2026, 8, 19, 9, 0);

    let timestamps = LinearIssueTimestamps {
        canceled_at: Some(canceled_at),
        updated_at: Some(updated_at),
        ..LinearIssueTimestamps::default()
    };

    let mock_issue = get_mock_issue(timestamps);

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(was_issue_worked_on(&mock_issue, activity_date, &timezone));
}

#[test]
fn was_issue_worked_on_ignores_an_issue_from_a_different_date() {
    let created_at = get_mock_instant(2026, 8, 10, 9, 0);

    let started_at = get_mock_instant(2026, 8, 11, 9, 0);

    let updated_at = get_mock_instant(2026, 8, 18, 9, 0);

    let timestamps = LinearIssueTimestamps {
        created_at: Some(created_at),
        started_at: Some(started_at),
        updated_at: Some(updated_at),
        ..LinearIssueTimestamps::default()
    };

    let mock_issue = get_mock_issue(timestamps);

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(!was_issue_worked_on(&mock_issue, activity_date, &timezone));
}

#[test]
fn was_issue_worked_on_ignores_an_issue_without_timestamps() {
    let mock_issue = get_mock_issue(LinearIssueTimestamps::default());

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(!was_issue_worked_on(&mock_issue, activity_date, &timezone));
}

#[test]
fn was_issue_worked_on_reads_timestamps_in_the_supplied_time_zone() {
    let updated_at = get_mock_instant(2026, 8, 17, 23, 30);

    let timestamps =
        LinearIssueTimestamps { updated_at: Some(updated_at), ..LinearIssueTimestamps::default() };

    let mock_issue = get_mock_issue(timestamps);

    let timezone = get_mock_timezone();

    let matching_date = get_mock_date(2026, 8, 17);

    let following_date = get_mock_date(2026, 8, 18);

    assert!(was_issue_worked_on(&mock_issue, matching_date, &timezone));

    assert!(!was_issue_worked_on(&mock_issue, following_date, &timezone));
}

#[test]
fn is_issue_active_or_finished_today_keeps_an_unfinished_issue() {
    let mock_issue =
        get_mock_issue_with_status(LinearStatusType::Unstarted, LinearIssueTimestamps::default());

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(is_issue_active_or_finished_today(&mock_issue, activity_date, &timezone));
}

#[test]
fn is_issue_active_or_finished_today_keeps_an_issue_completed_today() {
    let completed_at = get_mock_instant(2026, 8, 17, 16, 0);

    let timestamps = LinearIssueTimestamps {
        completed_at: Some(completed_at),
        ..LinearIssueTimestamps::default()
    };

    let mock_issue = get_mock_issue_with_status(LinearStatusType::Completed, timestamps);

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(is_issue_active_or_finished_today(&mock_issue, activity_date, &timezone));
}

#[test]
fn is_issue_active_or_finished_today_keeps_an_issue_canceled_today() {
    let canceled_at = get_mock_instant(2026, 8, 17, 16, 0);

    let timestamps = LinearIssueTimestamps {
        canceled_at: Some(canceled_at),
        ..LinearIssueTimestamps::default()
    };

    let mock_issue = get_mock_issue_with_status(LinearStatusType::Canceled, timestamps);

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(is_issue_active_or_finished_today(&mock_issue, activity_date, &timezone));
}

#[test]
fn is_issue_active_or_finished_today_ignores_an_issue_completed_on_an_earlier_day() {
    let completed_at = get_mock_instant(2026, 8, 10, 16, 0);

    let timestamps = LinearIssueTimestamps {
        completed_at: Some(completed_at),
        ..LinearIssueTimestamps::default()
    };

    let mock_issue = get_mock_issue_with_status(LinearStatusType::Completed, timestamps);

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(!is_issue_active_or_finished_today(&mock_issue, activity_date, &timezone));
}

#[test]
fn is_issue_active_or_finished_today_ignores_an_issue_completed_earlier_but_updated_today() {
    let completed_at = get_mock_instant(2026, 8, 10, 16, 0);

    let updated_at = get_mock_instant(2026, 8, 17, 9, 0);

    let timestamps = LinearIssueTimestamps {
        completed_at: Some(completed_at),
        updated_at: Some(updated_at),
        ..LinearIssueTimestamps::default()
    };

    let mock_issue = get_mock_issue_with_status(LinearStatusType::Completed, timestamps);

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(!is_issue_active_or_finished_today(&mock_issue, activity_date, &timezone));
}

#[test]
fn is_issue_active_or_finished_today_ignores_a_finished_issue_without_timestamps() {
    let mock_issue =
        get_mock_issue_with_status(LinearStatusType::Completed, LinearIssueTimestamps::default());

    let activity_date = get_mock_date(2026, 8, 17);

    let timezone = get_mock_timezone();

    assert!(!is_issue_active_or_finished_today(&mock_issue, activity_date, &timezone));
}

#[test]
fn is_issue_active_or_finished_today_reads_timestamps_in_the_supplied_time_zone() {
    let completed_at = get_mock_instant(2026, 8, 17, 23, 30);

    let timestamps = LinearIssueTimestamps {
        completed_at: Some(completed_at),
        ..LinearIssueTimestamps::default()
    };

    let mock_issue = get_mock_issue_with_status(LinearStatusType::Completed, timestamps);

    let timezone = get_mock_timezone();

    let matching_date = get_mock_date(2026, 8, 17);

    let following_date = get_mock_date(2026, 8, 18);

    assert!(is_issue_active_or_finished_today(&mock_issue, matching_date, &timezone));

    assert!(!is_issue_active_or_finished_today(&mock_issue, following_date, &timezone));
}
