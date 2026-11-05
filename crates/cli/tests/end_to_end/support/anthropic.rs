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
            .json_body_includes(format!(r#"{{ "model": "{ANTHROPIC_MODEL}" }}"#));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "content": [{ "type": "text", "text": GENERATED_SUMMARY }]
            }));
    })
}
