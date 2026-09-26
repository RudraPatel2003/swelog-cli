use miette::Result;
use url::Url;

use crate::errors::{
    GoogleAuthorizationDenied,
    GoogleAuthorizationFailed,
    GoogleCallbackStateMismatch,
};

// The authorization code is percent-encoded, so the query is parsed rather than
// split.
pub fn parse_authorization_code(callback_url: &str, expected_state: &str) -> Result<String> {
    let callback_url = Url::parse(callback_url).map_err(|error| GoogleAuthorizationFailed {
        message: format!("the Google redirect was not a valid URL: {error}"),
    })?;

    let query_parameters = callback_url.query_pairs().into_owned().collect::<Vec<_>>();

    if let Some(reason) = find_query_parameter(&query_parameters, "error") {
        let google_authorization_denied_error = GoogleAuthorizationDenied { reason };

        return Err(google_authorization_denied_error.into());
    }

    let state = find_query_parameter(&query_parameters, "state");

    if state.as_deref() != Some(expected_state) {
        return Err(GoogleCallbackStateMismatch.into());
    }

    find_query_parameter(&query_parameters, "code").ok_or_else(|| {
        GoogleAuthorizationFailed {
            message: "the Google redirect did not contain an authorization code".to_string(),
        }
        .into()
    })
}

fn find_query_parameter(query_parameters: &[(String, String)], name: &str) -> Option<String> {
    query_parameters
        .iter()
        .find(|(parameter_name, _)| parameter_name == name)
        .map(|(_, value)| value.clone())
}

#[cfg(test)]
#[path = "redirect_tests.rs"]
mod tests;
