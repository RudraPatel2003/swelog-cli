mod structs;

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use structs::UserResponse;

use crate::client::GitHubClient;

const USER_ENDPOINT_PATH: &str = "user";

const NO_QUERY_PARAMETERS: [(&str, &str); 0] = [];

impl GitHubClient {
    pub async fn get_username(&self) -> Result<String> {
        let response_text = self.get_json_text(USER_ENDPOINT_PATH, &NO_QUERY_PARAMETERS).await?;

        parse_user_response_text(&response_text)
    }
}

fn parse_user_response_text(response_text: &str) -> Result<String> {
    let user_response: UserResponse = serde_json::from_str(response_text)
        .into_diagnostic()
        .wrap_err("failed to parse GitHub user response")?;

    Ok(user_response.login)
}

#[cfg(test)]
#[path = "users_tests.rs"]
mod tests;
