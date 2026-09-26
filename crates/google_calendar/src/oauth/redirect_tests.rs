use super::*;

const EXPECTED_STATE: &str = "swelog-state";

#[test]
fn parse_authorization_code_reads_the_code() {
    let callback_url = "http://127.0.0.1:52341/callback?code=4/abc123&state=swelog-state";

    let authorization_code =
        parse_authorization_code(callback_url, EXPECTED_STATE).expect("redirect should parse");

    assert_eq!(authorization_code, "4/abc123");
}

#[test]
fn parse_authorization_code_decodes_a_percent_encoded_code() {
    let callback_url = "http://127.0.0.1:52341/callback?code=4%2F0AX%2Babc&state=swelog-state";

    let authorization_code =
        parse_authorization_code(callback_url, EXPECTED_STATE).expect("redirect should parse");

    assert_eq!(authorization_code, "4/0AX+abc");
}

#[test]
fn parse_authorization_code_fails_when_the_user_denied_access() {
    let callback_url = "http://127.0.0.1:52341/callback?error=access_denied&state=swelog-state";

    assert!(parse_authorization_code(callback_url, EXPECTED_STATE).is_err());
}

#[test]
fn parse_authorization_code_fails_when_the_state_does_not_match() {
    let callback_url = "http://127.0.0.1:52341/callback?code=4/abc123&state=someone-else";

    assert!(parse_authorization_code(callback_url, EXPECTED_STATE).is_err());
}

#[test]
fn parse_authorization_code_fails_when_the_state_is_missing() {
    let callback_url = "http://127.0.0.1:52341/callback?code=4/abc123";

    assert!(parse_authorization_code(callback_url, EXPECTED_STATE).is_err());
}

#[test]
fn parse_authorization_code_fails_when_the_code_is_missing() {
    let callback_url = "http://127.0.0.1:52341/callback?state=swelog-state";

    assert!(parse_authorization_code(callback_url, EXPECTED_STATE).is_err());
}
