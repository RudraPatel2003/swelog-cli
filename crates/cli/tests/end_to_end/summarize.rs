use config::swelog_config::{
    LanguageModelProvider,
    SwelogConfig,
};
use httpmock::MockServer;
use predicates::str::contains;

use crate::support::{
    anthropic::{
        ANTHROPIC_API_KEY,
        ANTHROPIC_MODEL,
        GENERATED_SUMMARY,
        mock_anthropic_messages,
    },
    sandbox::{
        ACTIVITY_DATE,
        DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS,
        SwelogSandbox,
        WRITTEN_WORK_FILE_CONTENT,
    },
};

const MONDAY_DATE: &str = "06-29-2026";

const EXPECTED_SUMMARIZED_DAILY_LOG: &str = "## Summary
- Reviewed the auth PR and paired on the release flow

## Original Notes

### Today's Work

#### Priorities
- Ship end-to-end tests

#### Log
- Reviewed the auth PR
- Paired on the release flow
";

fn get_sandbox_configured_for_anthropic() -> SwelogSandbox {
    let sandbox = SwelogSandbox::new();

    sandbox.write_config(&SwelogConfig {
        language_model_provider: Some(LanguageModelProvider::Anthropic),
        language_model: Some(String::from(ANTHROPIC_MODEL)),
        ..sandbox.default_config()
    });

    sandbox.setup();

    sandbox
}

#[test]
fn summarize_day_writes_the_generated_summary_with_the_original_notes() {
    let sandbox = get_sandbox_configured_for_anthropic();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    let anthropic = MockServer::start();

    let messages = mock_anthropic_messages(&anthropic);

    sandbox
        .swelog()
        .env("ANTHROPIC_API_KEY", ANTHROPIC_API_KEY)
        .env("SWELOG_ANTHROPIC_API_URL", anthropic.base_url())
        .args(["summarize", "day", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Summarizing day with provider Anthropic and model claude-sonnet-4-5..."))
        .stdout(contains(format!(
            "Successfully summarized your daily work into {}",
            sandbox.daily_log_file(ACTIVITY_DATE).display()
        )));

    messages.assert();

    assert_eq!(sandbox.read_daily_log(ACTIVITY_DATE), EXPECTED_SUMMARIZED_DAILY_LOG);

    assert_eq!(sandbox.read_work_file(), DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS);
}

#[test]
fn summarize_defaults_to_the_day_subcommand() {
    let sandbox = get_sandbox_configured_for_anthropic();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    let anthropic = MockServer::start();

    mock_anthropic_messages(&anthropic);

    sandbox
        .swelog()
        .env("ANTHROPIC_API_KEY", ANTHROPIC_API_KEY)
        .env("SWELOG_ANTHROPIC_API_URL", anthropic.base_url())
        .args(["summarize", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    assert_eq!(sandbox.read_daily_log(ACTIVITY_DATE), EXPECTED_SUMMARIZED_DAILY_LOG);
}

#[test]
fn summarize_day_can_be_undone() {
    let sandbox = get_sandbox_configured_for_anthropic();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    let anthropic = MockServer::start();

    mock_anthropic_messages(&anthropic);

    sandbox
        .swelog()
        .env("ANTHROPIC_API_KEY", ANTHROPIC_API_KEY)
        .env("SWELOG_ANTHROPIC_API_URL", anthropic.base_url())
        .args(["summarize", "day", "--date", ACTIVITY_DATE])
        .assert()
        .success();

    sandbox.swelog().arg("undo").assert().success();

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);

    assert!(!sandbox.daily_log_file(ACTIVITY_DATE).exists());
}

#[test]
fn summarize_week_writes_the_generated_weekly_log() {
    let sandbox = get_sandbox_configured_for_anthropic();

    sandbox
        .write_daily_log(MONDAY_DATE, "# Daily Log - 06-29-2026\n\n## Log\n- Planned the week\n");

    sandbox.write_daily_log("07-01-2026", "# Daily Log - 07-01-2026\n\n## Log\n- Shipped tests\n");

    let anthropic = MockServer::start();

    let messages = mock_anthropic_messages(&anthropic);

    sandbox
        .swelog()
        .env("ANTHROPIC_API_KEY", ANTHROPIC_API_KEY)
        .env("SWELOG_ANTHROPIC_API_URL", anthropic.base_url())
        .args(["summarize", "week", "--week-of", MONDAY_DATE])
        .assert()
        .success()
        .stdout(contains("Summarizing week with provider Anthropic and model claude-sonnet-4-5..."))
        .stdout(contains(format!(
            "Successfully summarized your weekly work into {}",
            sandbox.weekly_log_file(MONDAY_DATE).display()
        )));

    messages.assert();

    let weekly_log_content = std::fs::read_to_string(sandbox.weekly_log_file(MONDAY_DATE))
        .expect("weekly log should be readable");

    assert_eq!(weekly_log_content, GENERATED_SUMMARY);
}

#[test]
fn summarize_fails_when_no_provider_is_configured() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    sandbox
        .swelog()
        .args(["summarize", "day", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("summarization is not configured"));

    assert!(!sandbox.daily_log_file(ACTIVITY_DATE).exists());
}

#[test]
fn summarize_fails_fast_without_an_api_key_when_there_is_no_terminal() {
    let sandbox = get_sandbox_configured_for_anthropic();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    let anthropic = MockServer::start();

    let messages = mock_anthropic_messages(&anthropic);

    sandbox
        .swelog()
        .env("SWELOG_ANTHROPIC_API_URL", anthropic.base_url())
        .args(["summarize", "day", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("Anthropic API key is not available"));

    assert_eq!(messages.calls(), 0);

    assert_eq!(sandbox.read_work_file(), WRITTEN_WORK_FILE_CONTENT);
}
