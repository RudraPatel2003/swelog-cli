use super::*;

#[test]
fn parse_appends_a_trailing_slash_to_a_bare_path() {
    let base_url = BaseUrl::parse("https://api.openai.com/v1").expect("base URL should parse");

    assert_eq!(base_url.as_str(), "https://api.openai.com/v1/");
}

#[test]
fn parse_keeps_an_existing_trailing_slash() {
    let base_url = BaseUrl::parse("https://api.github.com/").expect("base URL should parse");

    assert_eq!(base_url.as_str(), "https://api.github.com/");
}

#[test]
fn join_appends_the_endpoint_path_to_a_nested_base() {
    let base_url = BaseUrl::parse("http://127.0.0.1:5555/api").expect("base URL should parse");

    let endpoint_url = base_url.join("v1/responses").expect("endpoint should join");

    assert_eq!(endpoint_url.as_str(), "http://127.0.0.1:5555/api/v1/responses");
}

#[test]
fn join_appends_the_endpoint_path_to_a_root_base() {
    let base_url = BaseUrl::parse("http://127.0.0.1:5555").expect("base URL should parse");

    let endpoint_url = base_url.join("search/issues").expect("endpoint should join");

    assert_eq!(endpoint_url.as_str(), "http://127.0.0.1:5555/search/issues");
}

#[test]
fn parse_fails_for_a_relative_value() {
    let error = BaseUrl::parse("api.github.com").expect_err("relative value should fail");

    assert_eq!(error.value, "api.github.com");
}

#[test]
fn parse_fails_for_a_url_that_cannot_be_a_base() {
    let error = BaseUrl::parse("mailto:someone@example.com").expect_err("mailto should fail");

    assert!(error.message.contains("cannot have paths joined"));
}
