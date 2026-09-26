use config::swelog_config::LanguageModelProvider;

use super::*;

const CONTEXT_FILE_CONTENT: &str = "backend engineer on platform team";

fn get_summarization_settings() -> SummarizationSettings {
    SummarizationSettings {
        language_model_provider: LanguageModelProvider::Anthropic,
        language_model: String::from("claude-sonnet-4-5"),
    }
}

#[test]
fn summarization_notice_names_the_period_provider_and_model() {
    let summarization_settings = get_summarization_settings();

    let notice =
        format_summarization_notice(SummarizationPeriod::Day, &summarization_settings, None);

    assert!(notice.contains("Summarizing day with provider "));

    assert!(notice.contains("Anthropic"));

    assert!(notice.contains("claude-sonnet-4-5"));
}

#[test]
fn summarization_notice_mentions_the_context_file_when_it_is_present() {
    let summarization_settings = get_summarization_settings();

    let notice = format_summarization_notice(
        SummarizationPeriod::Week,
        &summarization_settings,
        Some(CONTEXT_FILE_CONTENT),
    );

    assert!(notice.contains("Summarizing week with provider "));

    assert!(notice.contains("Incorporating CONTEXT.md...\n"));
}

#[test]
fn summarization_notice_omits_the_context_file_when_it_is_absent() {
    let summarization_settings = get_summarization_settings();

    let notice =
        format_summarization_notice(SummarizationPeriod::Day, &summarization_settings, None);

    assert!(!notice.contains("Incorporating"));
}
