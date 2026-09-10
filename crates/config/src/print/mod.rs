use std::path::Path;

use highlight::stdout::{
    highlight_cyan,
    path_link,
};

use crate::swelog_config::{
    LanguageModelProvider,
    SwelogConfig,
};

const LABEL_WIDTH: usize = 21;

const NOT_CONFIGURED: &str = "Not configured";

struct ConfigSection {
    title: &'static str,
    rows: Vec<ConfigRow>,
}

struct ConfigRow {
    label: &'static str,
    value: String,
}

pub fn print_config(config_file_path: &Path, config: &SwelogConfig) {
    let formatted_config = format_config(config);

    println!("Displaying config at {}:", highlight_cyan(path_link(config_file_path)));

    println!();

    print!("{formatted_config}");
}

fn format_config(config: &SwelogConfig) -> String {
    let sections = collect_config_sections(config);

    sections.iter().map(format_config_section).collect::<Vec<_>>().join("\n")
}

fn collect_config_sections(config: &SwelogConfig) -> Vec<ConfigSection> {
    vec![
        ConfigSection {
            title: "Vault",
            rows: vec![
                ConfigRow {
                    label: "Obsidian vault path",
                    value: path_link(&config.obsidian_vault_path),
                },
                ConfigRow { label: "Swelog folder name", value: config.swelog_folder_name.clone() },
            ],
        },
        ConfigSection {
            title: "Files",
            rows: vec![ConfigRow { label: "Work file", value: config.work_file_name.clone() }],
        },
        ConfigSection {
            title: "Logs",
            rows: vec![
                ConfigRow {
                    label: "Daily log folder",
                    value: config.daily_log_folder_name.clone(),
                },
                ConfigRow {
                    label: "Weekly log folder",
                    value: config.weekly_log_folder_name.clone(),
                },
            ],
        },
        ConfigSection {
            title: "Summarization",
            rows: vec![
                ConfigRow {
                    label: "Provider",
                    value: format_language_model_provider(config.language_model_provider)
                        .to_string(),
                },
                ConfigRow {
                    label: "Model",
                    value: config.language_model.as_deref().unwrap_or(NOT_CONFIGURED).to_string(),
                },
            ],
        },
        ConfigSection {
            title: "Integrations",
            rows: vec![ConfigRow {
                label: "Linear username",
                value: config.linear_username.as_deref().unwrap_or(NOT_CONFIGURED).to_string(),
            }],
        },
    ]
}

fn format_config_section(section: &ConfigSection) -> String {
    let formatted_rows = section.rows.iter().map(format_config_row).collect::<String>();

    format!("{}\n{formatted_rows}", section.title)
}

fn format_config_row(row: &ConfigRow) -> String {
    format!("  {:<LABEL_WIDTH$}{}\n", row.label, row.value)
}

const fn format_language_model_provider(
    language_model_provider: Option<LanguageModelProvider>,
) -> &'static str {
    match language_model_provider {
        Some(language_model_provider) => language_model_provider.label(),
        None => NOT_CONFIGURED,
    }
}

#[cfg(test)]
mod tests;
