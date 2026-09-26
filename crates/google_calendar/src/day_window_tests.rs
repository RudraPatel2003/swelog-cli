use chrono::{
    FixedOffset,
    Utc,
};

use super::*;

const HOUR_IN_SECONDS: i32 = 3600;

fn eastern_standard_time() -> FixedOffset {
    FixedOffset::west_opt(5 * HOUR_IN_SECONDS).expect("offset should be valid")
}

#[test]
fn get_day_window_spans_local_midnight_to_the_next_local_midnight() {
    let date = NaiveDate::from_ymd_opt(2026, 8, 17).expect("date should be valid");

    let timezone = eastern_standard_time();

    let day_window = get_day_window(date, &timezone).expect("window should build");

    assert_eq!(day_window.time_minimum, "2026-08-17T00:00:00-05:00");

    assert_eq!(day_window.time_maximum, "2026-08-18T00:00:00-05:00");
}

#[test]
fn get_day_window_crosses_a_month_boundary() {
    let date = NaiveDate::from_ymd_opt(2026, 8, 31).expect("date should be valid");

    let timezone = eastern_standard_time();

    let day_window = get_day_window(date, &timezone).expect("window should build");

    assert_eq!(day_window.time_maximum, "2026-09-01T00:00:00-05:00");
}

#[test]
fn get_day_window_uses_the_supplied_time_zone() {
    let date = NaiveDate::from_ymd_opt(2026, 8, 17).expect("date should be valid");

    let day_window = get_day_window(date, &Utc).expect("window should build");

    assert_eq!(day_window.time_minimum, "2026-08-17T00:00:00Z");

    assert_eq!(day_window.time_maximum, "2026-08-18T00:00:00Z");
}
