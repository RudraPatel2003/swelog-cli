use super::*;

#[test]
fn format_time_writes_a_twelve_hour_clock_without_padding() {
    let time = NaiveTime::from_hms_opt(9, 5, 0).expect("time should be valid");

    assert_eq!(format_time(&time), "9:05 AM");
}

#[test]
fn format_time_writes_afternoon_times_as_pm() {
    let time = NaiveTime::from_hms_opt(13, 30, 0).expect("time should be valid");

    assert_eq!(format_time(&time), "1:30 PM");
}

#[test]
fn format_time_writes_midnight_as_twelve_am() {
    let time = NaiveTime::from_hms_opt(0, 0, 0).expect("time should be valid");

    assert_eq!(format_time(&time), "12:00 AM");
}
