use super::*;

#[test]
fn parse_date_reads_month_day_year_order() {
    let parsed_date = parse_date("08-17-2026").expect("date should parse");

    let expected_date = NaiveDate::from_ymd_opt(2026, 8, 17).expect("date should be valid");

    assert_eq!(parsed_date, expected_date);
}

#[test]
fn parse_date_fails_for_other_date_formats() {
    let error = parse_date("2026-08-17").expect_err("date should not parse");

    assert_eq!(error.to_string(), "invalid date `2026-08-17`; expected MM-DD-YYYY");
}

#[test]
fn parse_date_fails_for_a_day_that_does_not_exist() {
    let error = parse_date("02-30-2026").expect_err("date should not parse");

    assert_eq!(error.to_string(), "invalid date `02-30-2026`; expected MM-DD-YYYY");
}

#[test]
fn parse_monday_date_accepts_a_monday() {
    let parsed_date = parse_monday_date("08-17-2026").expect("date should parse");

    assert_eq!(parsed_date.weekday(), Weekday::Mon);
}

#[test]
fn parse_monday_date_fails_when_the_date_is_not_a_monday() {
    let error = parse_monday_date("08-18-2026").expect_err("date should not parse");

    assert_eq!(error.to_string(), "`08-18-2026` is not a Monday; a week must start on a Monday");
}
