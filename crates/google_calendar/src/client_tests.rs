use super::*;

#[test]
fn take_next_page_token_stops_on_the_last_page() {
    let mut event_page = parse_event_page(r#"{ "items": [] }"#).expect("response should parse");

    assert!(event_page.take_next_page_token().is_none());
}

#[test]
fn take_next_page_token_reads_the_following_page() {
    let mut event_page = parse_event_page(r#"{ "items": [], "nextPageToken": "page-2" }"#)
        .expect("response should parse");

    assert_eq!(event_page.take_next_page_token(), Some("page-2".to_string()));
}

#[test]
fn parse_event_page_fails_on_a_body_that_is_not_an_event_page() {
    assert!(parse_event_page("<html>error</html>").is_err());
}

#[test]
fn get_event_page_query_parameters_bounds_the_request_to_the_day_window() {
    let day_window = DayWindow {
        time_minimum: "2026-08-17T00:00:00-05:00".to_string(),
        time_maximum: "2026-08-18T00:00:00-05:00".to_string(),
    };

    let query_parameters = get_event_page_query_parameters(&day_window, None);

    assert!(query_parameters.contains(&("timeMin", "2026-08-17T00:00:00-05:00".to_string())));

    assert!(query_parameters.contains(&("timeMax", "2026-08-18T00:00:00-05:00".to_string())));

    assert!(query_parameters.contains(&("singleEvents", "true".to_string())));

    assert!(query_parameters.contains(&("showDeleted", "true".to_string())));

    assert!(query_parameters.iter().all(|(name, _)| *name != "pageToken"));
}

#[test]
fn get_event_page_query_parameters_carries_the_page_token() {
    let day_window = DayWindow {
        time_minimum: "2026-08-17T00:00:00-05:00".to_string(),
        time_maximum: "2026-08-18T00:00:00-05:00".to_string(),
    };

    let query_parameters = get_event_page_query_parameters(&day_window, Some("page-2"));

    assert!(query_parameters.contains(&("pageToken", "page-2".to_string())));
}
