use std::{
    fs,
    path::Path,
};

use chrono::NaiveDate;
use config::{
    overwrite::Overwrite,
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
    swelog_file_existence::ensure_swelog_directory_exists,
};
use daily_log::{
    file::resolve_daily_log_file,
    work_file::{
        KeepWorkFile,
        finish_work_file,
        read_work_file_notes,
    },
};
use highlight::stderr::path_link;
use llm::{
    language_model::LanguageModel,
    prompts::get_daily_log_prompt,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use undo::snapshot::{
    UndoSnapshot,
    get_undo_snapshot_file_path,
    write_undo_snapshot,
};

pub async fn summarize_daily_work_from_config(
    swelog_config: &SwelogConfig,
    cache_directory: &Path,
    language_model: &dyn LanguageModel,
    log_date: &NaiveDate,
    context_file_content: Option<&str>,
    overwrite: Overwrite,
    keep_work_file: KeepWorkFile,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_directory_exists(&swelog_paths.daily_log_directory)?;

    let daily_log_file = resolve_daily_log_file(&swelog_paths, log_date, overwrite)?;

    let work_file_content = read_work_file_notes(&swelog_paths)?;

    let prompt = get_daily_log_prompt(&work_file_content, context_file_content, log_date);

    let generated_daily_log_content = language_model.generate_response(&prompt).await?;

    let daily_log_content =
        build_summarized_daily_log_content(&generated_daily_log_content, &work_file_content);

    fs::write(&daily_log_file, daily_log_content).into_diagnostic().wrap_err_with(|| {
        format!("failed to write daily log file at {}", path_link(&daily_log_file))
    })?;

    let undo_snapshot = UndoSnapshot { created_file: Some(daily_log_file), work_file_content };

    write_undo_snapshot(&get_undo_snapshot_file_path(cache_directory), &undo_snapshot)?;

    finish_work_file(swelog_config, cache_directory, keep_work_file)
}

fn build_summarized_daily_log_content(
    generated_daily_log_content: &str,
    work_file_content: &str,
) -> String {
    let original_notes_content = demote_markdown_headings(work_file_content);

    format!(
        "{}\n\n## Original Notes\n\n{}\n",
        generated_daily_log_content.trim_end(),
        original_notes_content.trim_end()
    )
}

fn demote_markdown_headings(markdown: &str) -> String {
    let mut demoted_lines = Vec::new();

    for line in markdown.lines() {
        demoted_lines.push(demote_markdown_heading(line));
    }

    let mut demoted_markdown = demoted_lines.join("\n");

    if markdown.ends_with('\n') {
        demoted_markdown.push('\n');
    }

    demoted_markdown
}

fn demote_markdown_heading(line: &str) -> String {
    if line.starts_with('#') { format!("##{line}") } else { String::from(line) }
}

#[cfg(test)]
mod tests;
