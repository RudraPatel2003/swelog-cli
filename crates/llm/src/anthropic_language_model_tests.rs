use crate::anthropic_language_model::{
    errors::AnthropicResponseMissingText,
    parse_anthropic_response_text,
};

const MODEL: &str = "claude-opus-5";

#[test]
fn parse_anthropic_response_text_extracts_text_blocks() {
    let response_body = r#"
            {
              "content": [
                {
                  "type": "text",
                  "text": "Daily summary"
                }
              ]
            }
        "#;

    let text = parse_anthropic_response_text(response_body, MODEL)
        .expect("Anthropic response text should parse");

    assert_eq!(text, "Daily summary");
}

#[test]
fn parse_anthropic_response_text_joins_multiple_text_blocks() {
    let response_body = r#"
            {
              "content": [
                {
                  "type": "text",
                  "text": "Daily "
                },
                {
                  "type": "text",
                  "text": "summary"
                }
              ]
            }
        "#;

    let text = parse_anthropic_response_text(response_body, MODEL)
        .expect("Anthropic response text should parse");

    assert_eq!(text, "Daily summary");
}

#[test]
fn parse_anthropic_response_text_ignores_thinking_blocks() {
    let response_body = r#"
            {
              "content": [
                {
                  "type": "thinking",
                  "thinking": ""
                },
                {
                  "type": "text",
                  "text": "Daily summary"
                }
              ]
            }
        "#;

    let text = parse_anthropic_response_text(response_body, MODEL)
        .expect("Anthropic response text should parse");

    assert_eq!(text, "Daily summary");
}

#[test]
fn parse_anthropic_response_text_fails_when_text_is_missing() {
    let response_body = r#"
            {
              "content": []
            }
        "#;

    let error =
        parse_anthropic_response_text(response_body, MODEL).expect_err("missing text should fail");

    let error = error
        .downcast_ref::<AnthropicResponseMissingText>()
        .expect("error should be AnthropicResponseMissingText");

    assert_eq!(error.model, MODEL);
}
