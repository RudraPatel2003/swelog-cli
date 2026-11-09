use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("OpenRouter response for model {model} did not include generated text")]
#[diagnostic(code(swelog::llm::open_router_response_missing_text))]
pub struct OpenRouterResponseMissingText {
    pub model: String,
}
