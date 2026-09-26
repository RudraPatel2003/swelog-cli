use std::path::{
    Path,
    PathBuf,
};

use super::*;

#[test]
fn format_config_groups_fields_under_section_headings() {
    let config = SwelogConfig {
        obsidian_vault_path: PathBuf::from("/home/user/vault"),
        language_model_provider: Some(LanguageModelProvider::Ollama),
        language_model: Some(String::from("llama3.2")),
        ..SwelogConfig::get_default_config()
    };

    let output = format_config(&config);

    assert!(output.contains("Vault\n"));

    assert!(output.contains("Files\n"));

    assert!(output.contains("Logs\n"));

    assert!(output.contains("Summarization\n"));

    assert!(output.contains("Integrations\n"));

    let vault_path_link = path_link(Path::new("/home/user/vault"));

    assert!(output.contains(&format!("Obsidian vault path  {vault_path_link}\n")));

    assert!(output.contains("Provider             Ollama\n"));

    assert!(output.contains("Model                llama3.2\n"));

    assert!(output.contains("Linear username      Not configured\n"));
}

#[test]
fn format_config_reports_summarization_as_not_configured_when_absent() {
    let config = SwelogConfig::get_default_config();

    let output = format_config(&config);

    assert!(output.contains("Provider             Not configured\n"));

    assert!(output.contains("Model                Not configured\n"));
}
