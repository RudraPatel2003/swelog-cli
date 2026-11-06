use chrono::FixedOffset;

use super::*;

fn get_mock_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("test date should be valid")
}

/// UTC-05:00, so local midnight lands in the early morning in UTC.
fn get_mock_timezone() -> FixedOffset {
    FixedOffset::west_opt(18_000).expect("test offset should be valid")
}

#[test]
fn get_activity_window_covers_the_date_in_utc() {
    let activity_date = get_mock_date(2026, 8, 17);

    let activity_window = get_activity_window(activity_date, &Utc).expect("window should be built");

    assert_eq!(activity_window.format_start(), "2026-08-17T00:00:00");

    assert_eq!(activity_window.format_end(), "2026-08-18T00:00:00");
}

#[test]
fn get_activity_window_converts_the_supplied_time_zone_to_utc() {
    let activity_date = get_mock_date(2026, 8, 17);

    let activity_window =
        get_activity_window(activity_date, &get_mock_timezone()).expect("window should be built");

    assert_eq!(activity_window.format_start(), "2026-08-17T05:00:00");

    assert_eq!(activity_window.format_end(), "2026-08-18T05:00:00");
}

#[test]
fn get_activity_window_crosses_a_year_boundary() {
    let activity_date = get_mock_date(2026, 12, 31);

    let activity_window = get_activity_window(activity_date, &Utc).expect("window should be built");

    assert_eq!(activity_window.format_start(), "2026-12-31T00:00:00");

    assert_eq!(activity_window.format_end(), "2027-01-01T00:00:00");
}

#[test]
fn activity_window_contains_its_start_but_not_its_end() {
    let activity_date = get_mock_date(2026, 8, 17);

    let activity_window = get_activity_window(activity_date, &Utc).expect("window should be built");

    let start_of_date = Utc
        .with_ymd_and_hms(2026, 8, 17, 0, 0, 0)
        .single()
        .expect("test instant should be valid");

    let start_of_following_date = Utc
        .with_ymd_and_hms(2026, 8, 18, 0, 0, 0)
        .single()
        .expect("test instant should be valid");

    assert!(activity_window.contains(start_of_date));

    assert!(!activity_window.contains(start_of_following_date));
}

#[test]
fn get_activity_window_fails_past_the_last_representable_date() {
    get_activity_window(NaiveDate::MAX, &Utc).expect_err("the following day should not exist");
}
