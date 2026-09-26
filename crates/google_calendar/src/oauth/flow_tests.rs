use super::*;

const REDIRECT_URI: &str = "http://127.0.0.1:52341/callback";

fn get_mock_application() -> GoogleOAuthApplication {
    GoogleOAuthApplication {
        client_id: String::from("test-client-id"),
        client_secret: String::from("test-secret"),
    }
}

fn build_test_authorization_url(application: &GoogleOAuthApplication) -> Url {
    build_authorization_url(application, REDIRECT_URI, "test-challenge", "test-state")
        .expect("authorization URL should build")
}

fn find_query_parameter(authorization_url: &Url, name: &str) -> Option<String> {
    authorization_url
        .query_pairs()
        .find(|(parameter_name, _)| parameter_name == name)
        .map(|(_, value)| value.into_owned())
}

#[test]
fn build_authorization_url_targets_the_google_authorization_endpoint() {
    let application = get_mock_application();

    let authorization_url = build_test_authorization_url(&application);

    assert_eq!(authorization_url.scheme(), "https");

    assert_eq!(authorization_url.host_str(), Some("accounts.google.com"));

    assert_eq!(authorization_url.path(), "/o/oauth2/v2/auth");
}

#[test]
fn build_authorization_url_asks_for_an_authorization_code_with_pkce() {
    let application = get_mock_application();

    let authorization_url = build_test_authorization_url(&application);

    assert_eq!(find_query_parameter(&authorization_url, "response_type"), Some("code".to_string()));

    assert_eq!(
        find_query_parameter(&authorization_url, "code_challenge"),
        Some("test-challenge".to_string())
    );

    assert_eq!(
        find_query_parameter(&authorization_url, "code_challenge_method"),
        Some("S256".to_string())
    );

    assert_eq!(find_query_parameter(&authorization_url, "state"), Some("test-state".to_string()));
}

#[test]
fn build_authorization_url_asks_for_offline_access_so_a_refresh_token_is_returned() {
    let application = get_mock_application();

    let authorization_url = build_test_authorization_url(&application);

    assert_eq!(
        find_query_parameter(&authorization_url, "access_type"),
        Some("offline".to_string())
    );

    assert_eq!(find_query_parameter(&authorization_url, "prompt"), Some("consent".to_string()));
}

#[test]
fn build_authorization_url_asks_only_for_read_only_event_access() {
    let application = get_mock_application();

    let authorization_url = build_test_authorization_url(&application);

    assert_eq!(
        find_query_parameter(&authorization_url, "scope"),
        Some("https://www.googleapis.com/auth/calendar.events.readonly".to_string())
    );
}

#[test]
fn build_authorization_url_carries_the_loopback_redirect_uri() {
    let application = get_mock_application();

    let authorization_url = build_test_authorization_url(&application);

    assert_eq!(
        find_query_parameter(&authorization_url, "redirect_uri"),
        Some(REDIRECT_URI.to_string())
    );

    assert_eq!(
        find_query_parameter(&authorization_url, "client_id"),
        Some("test-client-id".to_string())
    );
}

#[test]
fn build_authorization_url_never_carries_the_client_secret() {
    let application = get_mock_application();

    let authorization_url = build_test_authorization_url(&application);

    assert!(find_query_parameter(&authorization_url, "client_secret").is_none());
}
