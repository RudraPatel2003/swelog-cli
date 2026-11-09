use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Anthropic response for model {model} did not include generated text")]
#[diagnostic(code(swelog::llm::anthropic_response_missing_text))]
pub struct AnthropicResponseMissingText {
    pub model: String,
}
