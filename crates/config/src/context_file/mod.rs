use std::{
    fs,
    path::Path,
};

use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

pub const CONTEXT_FILE_NAME: &str = "CONTEXT.md";

pub fn get_context_file_content(context_file: &Path) -> Result<Option<String>> {
    if !context_file.is_file() {
        return Ok(None);
    }

    let context_file_content = fs::read_to_string(context_file)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read context file at {}", path_link(context_file)))?;

    Ok(Some(context_file_content))
}

#[cfg(test)]
mod tests;
