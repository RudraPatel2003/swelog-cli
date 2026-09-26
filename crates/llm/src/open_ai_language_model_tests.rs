use crate::open_ai_language_model::{
    errors::OpenAiResponseMissingText,
    parse_open_ai_response_text,
};

const MODEL: &str = "gpt-5.4-mini";

#[test]
fn parse_open_ai_response_text_extracts_output_text() {
    let response_body = r#"
            {
              "output": [
                {
                  "type": "message",
                  "content": [
                    {
                      "type": "output_text",
                      "text": "Daily summary"
                    }
                  ]
                }
              ]
            }
        "#;

    let text = parse_open_ai_response_text(response_body, MODEL)
        .expect("OpenAI response text should parse");

    assert_eq!(text, "Daily summary");
}

#[test]
fn parse_open_ai_response_text_joins_multiple_output_text_parts() {
    let response_body = r#"
            {
              "output": [
                {
                  "type": "message",
                  "content": [
                    {
                      "type": "output_text",
                      "text": "Daily "
                    },
                    {
                      "type": "output_text",
                      "text": "summary"
                    }
                  ]
                }
              ]
            }
        "#;

    let text = parse_open_ai_response_text(response_body, MODEL)
        .expect("OpenAI response text should parse");

    assert_eq!(text, "Daily summary");
}

#[test]
fn parse_open_ai_response_text_fails_when_output_text_is_missing() {
    let response_body = r#"
            {
              "output": [
                {
                  "type": "message",
                  "content": []
                }
              ]
            }
        "#;

    let error = parse_open_ai_response_text(response_body, MODEL)
        .expect_err("missing output text should fail");

    let error = error
        .downcast_ref::<OpenAiResponseMissingText>()
        .expect("error should be OpenAiResponseMissingText");

    assert_eq!(error.model, MODEL);
}
