use super::parse_user_response_text;

#[test]
fn parse_user_response_text_extracts_login() {
    let response_body = r#"
            {
              "login": "octocat",
              "id": 1,
              "html_url": "https://github.com/octocat"
            }
        "#;

    let username =
        parse_user_response_text(response_body).expect("GitHub user response should parse");

    assert_eq!(username, "octocat");
}

#[test]
fn parse_user_response_text_fails_when_login_is_missing() {
    let response_body = r#"
            {
              "id": 1,
              "html_url": "https://github.com/octocat"
            }
        "#;

    parse_user_response_text(response_body).expect_err("missing login field should fail");
}
