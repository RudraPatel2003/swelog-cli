use crate::commands::fetch::sources::FetchSource;

pub fn format_running_notice(fetch_sources: &[FetchSource]) -> String {
    format!(
        "Running the fetch commands you have configured: {}.",
        format_fetch_source_labels(fetch_sources)
    )
}

pub fn format_fetch_source_labels(fetch_sources: &[FetchSource]) -> String {
    fetch_sources.iter().map(|fetch_source| fetch_source.label()).collect::<Vec<_>>().join(", ")
}

#[cfg(test)]
#[path = "formatting_tests.rs"]
mod tests;
