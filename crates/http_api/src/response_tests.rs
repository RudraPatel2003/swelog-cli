use miette::Diagnostic;

use super::*;

const MOCK_API_SERVICE: ApiService = ApiService {
    label: "Mock Service",
    credential_name: "token",
    credential_command_name: "mock-service",
    authorization_hint: "check that your token has the `read` scope",
    accept_header: None,
};

const RESPONSE_TEXT: &str = r#"{ "message": "response" }"#;

fn get_response_text(status_code: StatusCode) -> Result<String> {
    get_successful_response_text(&MOCK_API_SERVICE, status_code, RESPONSE_TEXT.to_string())
}

#[test]
fn successful_response_returns_its_text() {
    let response_text = get_response_text(StatusCode::OK).expect("response should be successful");

    assert_eq!(response_text, RESPONSE_TEXT);
}

#[test]
fn unauthorized_response_asks_to_replace_the_credential() {
    let error =
        get_response_text(StatusCode::UNAUTHORIZED).expect_err("unauthorized response should fail");

    let authorization_failed_error = error
        .downcast_ref::<ApiAuthorizationFailed>()
        .expect("error should be an authorization failure");

    assert_eq!(authorization_failed_error.status_code, StatusCode::UNAUTHORIZED);

    assert_eq!(
        authorization_failed_error.to_string(),
        "Mock Service rejected your token with status 401 Unauthorized"
    );
}

#[test]
fn forbidden_response_asks_to_replace_the_credential() {
    let error =
        get_response_text(StatusCode::FORBIDDEN).expect_err("forbidden response should fail");

    assert!(
        error
            .downcast_ref::<ApiAuthorizationFailed>()
            .is_some()
    );
}

#[test]
fn authorization_failure_help_names_the_clear_command_and_the_hint() {
    let authorization_failed_error =
        ApiAuthorizationFailed::new(&MOCK_API_SERVICE, StatusCode::UNAUTHORIZED);

    let help = authorization_failed_error
        .help()
        .expect("authorization failure should have help")
        .to_string();

    assert_eq!(
        help,
        "run `swelog auth clear mock-service` and run the command again to replace your token, or check that your token has the `read` scope"
    );
}

#[test]
fn unsuccessful_response_keeps_the_status_and_text() {
    let error = get_response_text(StatusCode::INTERNAL_SERVER_ERROR)
        .expect_err("server error response should fail");

    let unsuccessful_response_error = error
        .downcast_ref::<UnsuccessfulApiResponse>()
        .expect("error should be an unsuccessful response");

    assert_eq!(unsuccessful_response_error.status_code, StatusCode::INTERNAL_SERVER_ERROR);

    assert_eq!(unsuccessful_response_error.response_text, RESPONSE_TEXT);
}
