use httpmock::{
    Method::POST,
    Mock,
    MockServer,
};

pub const ANTHROPIC_API_KEY: &str = "sk-ant-end-to-end";

pub const ANTHROPIC_MODEL: &str = "claude-sonnet-4-5";

pub const GENERATED_SUMMARY: &str =
    "## Summary\n- Reviewed the auth PR and paired on the release flow";

pub fn mock_anthropic_messages(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(POST)
            .path("/v1/messages")
            .header("x-api-key", ANTHROPIC_API_KEY)
            .header("anthropic-version", "2023-06-01")
            .json_body_includes(format!(r#"{{ "model": "{ANTHROPIC_MODEL}" }}"#));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "content": [{ "type": "text", "text": GENERATED_SUMMARY }]
            }));
    })
}

pub fn mock_anthropic_rejecting_the_api_key(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(POST)
            .path("/v1/messages");

        then.status(401)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "type": "error",
                "error": { "type": "authentication_error", "message": "invalid x-api-key" }
            }));
    })
}
