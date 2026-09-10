use std::{
    fs,
    ops::Range,
};

use config::{
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
    swelog_file_existence::ensure_swelog_file_exists,
};
use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::sections::{
    find_section_bounds,
    format_section,
    remove_block,
    replace_block,
};

/// Integration sections are inserted directly above this section so the user's
/// own notes stay at the bottom of the work file.
const LOG_SECTION_TITLE: &str = "Log";

pub fn upsert_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    section_title: &str,
    content: &str,
) -> Result<()> {
    update_work_file_from_config(swelog_config, |work_file_content| {
        upsert_section(work_file_content, section_title, content)
    })
}

pub fn remove_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    section_title: &str,
) -> Result<()> {
    update_work_file_from_config(swelog_config, |work_file_content| {
        remove_section(work_file_content, section_title)
    })
}

fn update_work_file_from_config(
    swelog_config: &SwelogConfig,
    update_work_file: impl FnOnce(&str) -> String,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_file_exists(&swelog_paths.work_file)?;

    let work_file_content =
        fs::read_to_string(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", path_link(&swelog_paths.work_file))
        })?;

    let updated_work_file_content = update_work_file(&work_file_content);

    fs::write(&swelog_paths.work_file, updated_work_file_content).into_diagnostic().wrap_err_with(
        || format!("failed to write work file at {}", path_link(&swelog_paths.work_file)),
    )?;

    Ok(())
}

fn upsert_section(markdown: &str, section_title: &str, content: &str) -> String {
    let section_block = format_section(section_title, content);

    let section_bounds = find_section_bounds(markdown, section_title)
        .unwrap_or_else(|| insertion_bounds_above_log(markdown));

    replace_block(markdown, section_bounds, &section_block)
}

fn remove_section(markdown: &str, section_title: &str) -> String {
    find_section_bounds(markdown, section_title).map_or_else(
        || markdown.to_owned(),
        |section_bounds| remove_block(markdown, section_bounds),
    )
}

fn insertion_bounds_above_log(markdown: &str) -> Range<usize> {
    let insertion_index = find_section_bounds(markdown, LOG_SECTION_TITLE)
        .map_or(markdown.len(), |log_section_bounds| log_section_bounds.start);

    insertion_index..insertion_index
}

#[cfg(test)]
mod tests;
