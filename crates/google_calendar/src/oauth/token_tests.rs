use super::*;

const RECEIVED_AT: u64 = 1_000_000;

#[test]
fn parse_token_response_reads_an_authorization_code_exchange() {
    let response_text = r#"{
        "access_token": "ya29.access",
        "expires_in": 3599,
        "refresh_token": "1//refresh",
        "scope": "https://www.googleapis.com/auth/calendar.events.readonly",
        "token_type": "Bearer"
    }"#;

    let token_response = parse_token_response(response_text).expect("response should parse");

    let google_credentials = token_response.into_credentials("1//refresh".to_string(), RECEIVED_AT);

    let expected_google_credentials = GoogleCredentials {
        access_token: "ya29.access".to_string(),
        refresh_token: "1//refresh".to_string(),
        expires_at: 1_003_599,
        scopes: vec!["https://www.googleapis.com/auth/calendar.events.readonly".to_string()],
    };

    assert_eq!(google_credentials, expected_google_credentials);
}

#[test]
fn get_refresh_token_to_store_keeps_the_stored_token_when_a_refresh_omits_one() {
    let response_text = r#"{
        "access_token": "ya29.refreshed",
        "expires_in": 3599,
        "scope": "https://www.googleapis.com/auth/calendar.events.readonly",
        "token_type": "Bearer"
    }"#;

    let token_response = parse_token_response(response_text).expect("response should parse");

    assert_eq!(get_refresh_token_to_store(&token_response, "1//stored"), "1//stored");
}

#[test]
fn get_refresh_token_to_store_prefers_a_rotated_token() {
    let response_text = r#"{
        "access_token": "ya29.refreshed",
        "expires_in": 3599,
        "refresh_token": "1//rotated",
        "token_type": "Bearer"
    }"#;

    let token_response = parse_token_response(response_text).expect("response should parse");

    assert_eq!(get_refresh_token_to_store(&token_response, "1//stored"), "1//rotated");
}

#[test]
fn parse_token_response_fails_when_the_access_token_is_missing() {
    let response_text = r#"{ "expires_in": 3599 }"#;

    assert!(parse_token_response(response_text).is_err());
}

#[test]
fn describe_token_error_reads_the_error_and_its_description() {
    let response_text = r#"{ "error": "invalid_client", "error_description": "Unauthorized" }"#;

    let token_error = parse_token_error_response(response_text);

    assert_eq!(
        describe_token_error(token_error.as_ref(), response_text),
        "invalid_client (Unauthorized)"
    );
}

#[test]
fn describe_token_error_falls_back_to_the_raw_body() {
    let response_text = "<html>gateway timeout</html>";

    let token_error = parse_token_error_response(response_text);

    assert_eq!(describe_token_error(token_error.as_ref(), response_text), response_text);
}
