use super::*;

#[test]
fn format_date_writes_month_day_year_order() {
    let date = NaiveDate::from_ymd_opt(2026, 8, 17).expect("date should be valid");

    assert_eq!(format_date(&date), "08-17-2026");
}

#[test]
fn format_date_pads_single_digit_months_and_days() {
    let date = NaiveDate::from_ymd_opt(2026, 1, 5).expect("date should be valid");

    assert_eq!(format_date(&date), "01-05-2026");
}
