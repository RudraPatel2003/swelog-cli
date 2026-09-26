use std::{
    fs,
    path::Path,
};

use config::context_file::get_context_file_content;
use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::formatting::format_markdown;

pub fn format_context_file(context_file: &Path) -> Result<()> {
    let Some(context_file_content) = get_context_file_content(context_file)? else {
        return Ok(());
    };

    let formatted_context_file_content = format_markdown(&context_file_content);

    if formatted_context_file_content == context_file_content {
        return Ok(());
    }

    fs::write(context_file, formatted_context_file_content)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write context file at {}", path_link(context_file)))
}

#[cfg(test)]
mod tests;
