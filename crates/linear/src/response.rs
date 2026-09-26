use miette::Result;
use rmcp::model::CallToolResult;
use serde_json::Value;

use crate::{
    client::structs::LinearIssuePage,
    errors::UnsupportedLinearResponse,
};

const NO_ISSUES_FOUND_MESSAGE: &str = "No issues found";

pub fn parse_issue_page(result: CallToolResult) -> Result<LinearIssuePage> {
    let value = extract_result_value(result)?;

    serde_json::from_value(value).map_err(|error| {
        UnsupportedLinearResponse {
            message: format!("issue response did not match the expected shape: {error}"),
        }
        .into()
    })
}

fn extract_result_value(result: CallToolResult) -> Result<Value> {
    if result.is_error == Some(true) {
        let text = collect_result_text(&result);

        let message =
            if text.is_empty() { "Linear MCP tool returned an error".to_string() } else { text };

        let unsupported_linear_response_error = UnsupportedLinearResponse { message };

        return Err(unsupported_linear_response_error.into());
    }

    if let Some(value) = result.structured_content {
        return Ok(value);
    }

    let text = collect_result_text(&result);

    if text.contains(NO_ISSUES_FOUND_MESSAGE) {
        return Ok(serde_json::json!({ "issues": [] }));
    }

    serde_json::from_str(&text).map_err(|error| {
        UnsupportedLinearResponse {
            message: format!("tool result did not contain structured JSON: {error}"),
        }
        .into()
    })
}

fn collect_result_text(result: &CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|content| content.as_text())
        .map(|text| text.text.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
#[path = "response_tests.rs"]
mod tests;
