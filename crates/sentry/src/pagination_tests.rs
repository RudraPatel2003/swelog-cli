use super::*;

const LINK_HEADER_WITH_NEXT_PAGE: &str = r#"<https://sentry.io/api/0/organizations/acme/issues/?query=assigned%3Ame&cursor=1420837590:0:1>; rel="previous"; results="false", <https://sentry.io/api/0/organizations/acme/issues/?query=assigned%3Ame&cursor=1420837533:0:0>; rel="next"; results="true""#;

const LINK_HEADER_ON_LAST_PAGE: &str = r#"<https://sentry.io/api/0/organizations/acme/issues/?&cursor=1420837590:0:1>; rel="previous"; results="true", <https://sentry.io/api/0/organizations/acme/issues/?&cursor=1420837533:0:0>; rel="next"; results="false""#;

#[test]
fn parse_next_cursor_returns_the_cursor_of_the_next_page() {
    let next_cursor = parse_next_cursor(LINK_HEADER_WITH_NEXT_PAGE);

    assert_eq!(next_cursor, Some("1420837533:0:0".to_string()));
}

#[test]
fn parse_next_cursor_is_empty_on_the_last_page() {
    let next_cursor = parse_next_cursor(LINK_HEADER_ON_LAST_PAGE);

    assert_eq!(next_cursor, None);
}

#[test]
fn parse_next_cursor_ignores_a_previous_link_with_results() {
    let link_header = r#"<https://sentry.io/api/0/organizations/acme/issues/?&cursor=1:0:1>; rel="previous"; results="true""#;

    assert_eq!(parse_next_cursor(link_header), None);
}

#[test]
fn parse_next_cursor_decodes_an_encoded_cursor() {
    let link_header = r#"<https://sentry.io/api/0/organizations/acme/issues/?cursor=1420837533%3A0%3A0>; rel="next"; results="true""#;

    assert_eq!(parse_next_cursor(link_header), Some("1420837533:0:0".to_string()));
}

#[test]
fn parse_next_cursor_is_empty_for_a_malformed_header() {
    assert_eq!(parse_next_cursor("not a link header"), None);

    assert_eq!(parse_next_cursor(r#"<not a url>; rel="next"; results="true""#), None);
}
