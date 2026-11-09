use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("OpenAI response for model {model} did not include generated text")]
#[diagnostic(code(swelog::llm::open_ai_response_missing_text))]
pub struct OpenAiResponseMissingText {
    pub model: String,
}
