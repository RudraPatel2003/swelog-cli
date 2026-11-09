use reqwest::{
    Request,
    header::AUTHORIZATION,
};

use super::*;

const MOCK_URL: &str = "https://api.example.com/resource";

const MOCK_SECRET: &str = "secret";

fn build_request(request: RequestBuilder) -> Request {
    request
        .build()
        .expect("request should build")
}

fn get_header<'request>(request: &'request Request, header_name: &str) -> Option<&'request str> {
    request
        .headers()
        .get(header_name)
        .and_then(|header_value| {
            header_value
                .to_str()
                .ok()
        })
}

fn get_mock_request() -> RequestBuilder {
    Client::new().get(MOCK_URL)
}

#[test]
fn anonymous_authentication_adds_no_credential() {
    let request = build_request(add_authentication(get_mock_request(), Authentication::Anonymous));

    assert!(
        request
            .headers()
            .is_empty()
    );
}

#[test]
fn bearer_token_authentication_sets_the_authorization_header() {
    let request = build_request(add_authentication(
        get_mock_request(),
        Authentication::BearerToken(MOCK_SECRET),
    ));

    assert_eq!(get_header(&request, AUTHORIZATION.as_str()), Some("Bearer secret"));
}

#[test]
fn api_key_header_authentication_sets_the_named_header() {
    let authentication =
        Authentication::ApiKeyHeader { header_name: "x-api-key", api_key: MOCK_SECRET };

    let request = build_request(add_authentication(get_mock_request(), authentication));

    assert_eq!(get_header(&request, "x-api-key"), Some(MOCK_SECRET));

    assert_eq!(get_header(&request, AUTHORIZATION.as_str()), None);
}

#[test]
fn default_headers_are_added_to_the_request() {
    let default_headers = [("accept", "application/json"), ("api-version", "2023-06-01")];

    let request = build_request(add_default_headers(get_mock_request(), &default_headers));

    assert_eq!(get_header(&request, "accept"), Some("application/json"));

    assert_eq!(get_header(&request, "api-version"), Some("2023-06-01"));
}

#[test]
fn request_timeout_is_applied_when_set() {
    let request_timeout = Duration::from_secs(5);

    let request = build_request(add_request_timeout(get_mock_request(), Some(request_timeout)));

    assert_eq!(request.timeout(), Some(&request_timeout));
}

#[test]
fn request_timeout_is_left_unset_by_default() {
    let request = build_request(add_request_timeout(get_mock_request(), None));

    assert_eq!(request.timeout(), None);
}
