use super::*;

fn get_mock_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("date should be valid")
}

#[test]
fn unspecified_date_selection_is_the_current_window() {
    let today = get_mock_date(2026, 8, 21);

    let fetch_window = FetchWindow::from_date_selection(DateSelection::Unspecified, today)
        .expect("fetch window should be resolved");

    assert_eq!(fetch_window, FetchWindow::Current);
}

#[test]
fn supplied_date_selection_is_a_window_on_that_date() {
    let supplied_date = get_mock_date(2026, 8, 17);

    let today = get_mock_date(2026, 8, 21);

    let fetch_window = FetchWindow::from_date_selection(DateSelection::On(supplied_date), today)
        .expect("fetch window should be resolved");

    assert_eq!(fetch_window, FetchWindow::Date(supplied_date));
}

#[test]
fn yesterday_date_selection_is_a_window_on_yesterday() {
    let today = get_mock_date(2026, 8, 21);

    let yesterday = get_mock_date(2026, 8, 20);

    let fetch_window = FetchWindow::from_date_selection(DateSelection::Yesterday, today)
        .expect("fetch window should be resolved");

    assert_eq!(fetch_window, FetchWindow::Date(yesterday));
}

#[test]
fn current_window_falls_back_to_today() {
    let today = get_mock_date(2026, 8, 21);

    assert_eq!(FetchWindow::Current.date_or_today(today), today);
}

#[test]
fn date_window_keeps_its_date_instead_of_today() {
    let window_date = get_mock_date(2026, 8, 17);

    let today = get_mock_date(2026, 8, 21);

    assert_eq!(FetchWindow::Date(window_date).date_or_today(today), window_date);
}
