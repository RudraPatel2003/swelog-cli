use config::context_file::CONTEXT_FILE_NAME;
use highlight::stdout::highlight_cyan;
use llm::summarization_settings::SummarizationSettings;

#[derive(Debug, Clone, Copy)]
pub enum SummarizationPeriod {
    Day,
    Week,
}

impl SummarizationPeriod {
    const fn label(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
        }
    }
}

#[must_use]
pub fn format_summarization_notice(
    period: SummarizationPeriod,
    summarization_settings: &SummarizationSettings,
    context_file_content: Option<&str>,
) -> String {
    let summarizing_line = format!(
        "Summarizing {} with provider {} and model {}...\n",
        period.label(),
        highlight_cyan(summarization_settings.language_model_provider.label()),
        highlight_cyan(&summarization_settings.language_model)
    );

    let context_line = format_context_line(context_file_content);

    format!("{summarizing_line}{context_line}")
}

fn format_context_line(context_file_content: Option<&str>) -> String {
    match context_file_content {
        Some(_) => format!("Incorporating {CONTEXT_FILE_NAME}...\n"),
        None => String::new(),
    }
}

#[cfg(test)]
#[path = "summarization_notice_tests.rs"]
mod tests;
