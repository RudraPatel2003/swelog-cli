use crate::commands::fetch::integration::Integration;

pub fn format_running_notice(integrations: &[&dyn Integration]) -> String {
    format!(
        "Running the fetch commands you have configured: {}.",
        format_integration_labels(integrations)
    )
}

pub fn format_integration_labels(integrations: &[&dyn Integration]) -> String {
    integrations
        .iter()
        .map(|integration| {
            integration
                .spec()
                .label
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
#[path = "formatting_tests.rs"]
mod tests;
