use super::*;

fn get_mock_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("date should be valid")
}

#[test]
fn selected_date_is_the_supplied_date_when_a_date_is_given() {
    let supplied_date = get_mock_date(2026, 8, 17);

    let today = get_mock_date(2026, 8, 21);

    let selected_date = resolve_selected_date(DateSelection::On(supplied_date), today)
        .expect("date should be resolved");

    assert_eq!(selected_date, Some(supplied_date));
}

#[test]
fn selected_date_is_yesterday_when_the_yesterday_flag_is_set() {
    let today = get_mock_date(2026, 8, 21);

    let yesterday = get_mock_date(2026, 8, 20);

    let selected_date =
        resolve_selected_date(DateSelection::Yesterday, today).expect("date should be resolved");

    assert_eq!(selected_date, Some(yesterday));
}

#[test]
fn selected_date_is_none_when_neither_flag_is_set() {
    let today = get_mock_date(2026, 8, 21);

    let selected_date =
        resolve_selected_date(DateSelection::Unspecified, today).expect("date should be resolved");

    assert_eq!(selected_date, None);
}

#[test]
fn yesterday_crosses_a_month_boundary() {
    let today = get_mock_date(2026, 3, 1);

    let expected_yesterday = get_mock_date(2026, 2, 28);

    let selected_date =
        resolve_selected_date(DateSelection::Yesterday, today).expect("date should be resolved");

    assert_eq!(selected_date, Some(expected_yesterday));
}

#[test]
fn yesterday_crosses_a_year_boundary() {
    let today = get_mock_date(2026, 1, 1);

    let expected_yesterday = get_mock_date(2025, 12, 31);

    let selected_date =
        resolve_selected_date(DateSelection::Yesterday, today).expect("date should be resolved");

    assert_eq!(selected_date, Some(expected_yesterday));
}

#[test]
fn yesterday_resolves_to_a_leap_day() {
    let today = get_mock_date(2028, 3, 1);

    let expected_yesterday = get_mock_date(2028, 2, 29);

    let selected_date =
        resolve_selected_date(DateSelection::Yesterday, today).expect("date should be resolved");

    assert_eq!(selected_date, Some(expected_yesterday));
}

#[test]
fn last_friday_from_a_monday_is_the_friday_before_the_weekend() {
    // Monday 09-21-2026, the morning you log the work you did before the weekend.
    let today = get_mock_date(2026, 9, 21);

    let expected_last_friday = get_mock_date(2026, 9, 18);

    let selected_date =
        resolve_selected_date(DateSelection::LastFriday, today).expect("date should be resolved");

    assert_eq!(selected_date, Some(expected_last_friday));
}

#[test]
fn last_friday_from_a_friday_is_the_previous_friday() {
    // Friday 09-18-2026. Today is a Friday, so last Friday is a full week back.
    let today = get_mock_date(2026, 9, 18);

    let expected_last_friday = get_mock_date(2026, 9, 11);

    let selected_date =
        resolve_selected_date(DateSelection::LastFriday, today).expect("date should be resolved");

    assert_eq!(selected_date, Some(expected_last_friday));
}

#[test]
fn last_friday_from_a_saturday_is_the_day_before() {
    // Saturday 09-19-2026.
    let today = get_mock_date(2026, 9, 19);

    let expected_last_friday = get_mock_date(2026, 9, 18);

    let selected_date =
        resolve_selected_date(DateSelection::LastFriday, today).expect("date should be resolved");

    assert_eq!(selected_date, Some(expected_last_friday));
}

#[test]
fn last_friday_from_a_sunday_is_the_friday_that_started_the_weekend() {
    // Sunday 09-20-2026.
    let today = get_mock_date(2026, 9, 20);

    let expected_last_friday = get_mock_date(2026, 9, 18);

    let selected_date =
        resolve_selected_date(DateSelection::LastFriday, today).expect("date should be resolved");

    assert_eq!(selected_date, Some(expected_last_friday));
}

#[test]
fn last_friday_crosses_a_year_boundary() {
    // Friday 01-01-2027.
    let today = get_mock_date(2027, 1, 1);

    let expected_last_friday = get_mock_date(2026, 12, 25);

    let selected_date =
        resolve_selected_date(DateSelection::LastFriday, today).expect("date should be resolved");

    assert_eq!(selected_date, Some(expected_last_friday));
}

#[test]
fn monday_date_is_the_supplied_monday_when_one_is_given() {
    let supplied_monday = get_mock_date(2026, 8, 17);

    let today = get_mock_date(2026, 8, 21);

    let monday_date = resolve_monday_date(WeekSelection::WeekOf(supplied_monday), today)
        .expect("Monday should be resolved");

    assert_eq!(monday_date, supplied_monday);
}

#[test]
fn default_week_is_the_monday_of_the_current_week_midweek() {
    // Friday 08-21-2026.
    let today = get_mock_date(2026, 8, 21);

    let expected_monday = get_mock_date(2026, 8, 17);

    let monday_date =
        resolve_monday_date(WeekSelection::Current, today).expect("Monday should be resolved");

    assert_eq!(monday_date, expected_monday);
}

#[test]
fn default_week_is_today_when_today_is_a_monday() {
    // Monday 08-17-2026. A week you only worked its Monday still summarizes.
    let today = get_mock_date(2026, 8, 17);

    let expected_monday = get_mock_date(2026, 8, 17);

    let monday_date =
        resolve_monday_date(WeekSelection::Current, today).expect("Monday should be resolved");

    assert_eq!(monday_date, expected_monday);
}

#[test]
fn default_week_is_the_monday_of_the_current_week_on_a_sunday() {
    // Sunday 08-23-2026.
    let today = get_mock_date(2026, 8, 23);

    let expected_monday = get_mock_date(2026, 8, 17);

    let monday_date =
        resolve_monday_date(WeekSelection::Current, today).expect("Monday should be resolved");

    assert_eq!(monday_date, expected_monday);
}

#[test]
fn last_week_is_seven_days_before_the_monday_of_the_current_week() {
    // Friday 08-21-2026, whose default Monday is 08-17-2026.
    let today = get_mock_date(2026, 8, 21);

    let expected_monday = get_mock_date(2026, 8, 10);

    let monday_date =
        resolve_monday_date(WeekSelection::LastWeek, today).expect("Monday should be resolved");

    assert_eq!(monday_date, expected_monday);
}

#[test]
fn last_week_from_a_monday_is_the_week_that_just_finished() {
    // Monday 08-17-2026, for the Friday you left without summarizing the week.
    let today = get_mock_date(2026, 8, 17);

    let expected_monday = get_mock_date(2026, 8, 10);

    let monday_date =
        resolve_monday_date(WeekSelection::LastWeek, today).expect("Monday should be resolved");

    assert_eq!(monday_date, expected_monday);
}

fn get_date_flags(
    date: Option<NaiveDate>,
    use_yesterday: bool,
    use_last_friday: bool,
) -> DateFlags {
    DateFlags { date, use_yesterday, use_last_friday }
}

#[test]
fn date_flags_map_to_the_day_they_stand_for() {
    let date = get_mock_date(2026, 8, 17);

    assert_eq!(
        DateSelection::from_date_flags(get_date_flags(None, false, false)),
        DateSelection::Unspecified
    );

    assert_eq!(
        DateSelection::from_date_flags(get_date_flags(None, true, false)),
        DateSelection::Yesterday
    );

    assert_eq!(
        DateSelection::from_date_flags(get_date_flags(None, false, true)),
        DateSelection::LastFriday
    );

    assert_eq!(
        DateSelection::from_date_flags(get_date_flags(Some(date), false, false)),
        DateSelection::On(date)
    );
}

#[test]
fn week_flags_map_to_the_week_they_stand_for() {
    let monday_date = get_mock_date(2026, 8, 17);

    assert_eq!(WeekSelection::from_week_flags(None, false), WeekSelection::Current);

    assert_eq!(WeekSelection::from_week_flags(None, true), WeekSelection::LastWeek);

    assert_eq!(
        WeekSelection::from_week_flags(Some(monday_date), false),
        WeekSelection::WeekOf(monday_date)
    );
}
