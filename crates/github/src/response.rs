use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Response,
    StatusCode,
};

use crate::errors::{
    GitHubAuthorizationFailed,
    UnsuccessfulGitHubResponse,
};

/// Instruct user to clear auth token if response is unauthorized or forbidden
pub async fn read_successful_response_body(response: Response) -> Result<String> {
    let status_code = response.status();

    let response_text = response
        .text()
        .await
        .into_diagnostic()
        .wrap_err("failed to read GitHub response body")?;

    if matches!(status_code, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) {
        let github_authorization_failed_error = GitHubAuthorizationFailed { status_code };

        return Err(github_authorization_failed_error.into());
    }

    if !status_code.is_success() {
        let unsuccessful_github_response_error =
            UnsuccessfulGitHubResponse { status_code, response_text };

        return Err(unsuccessful_github_response_error.into());
    }

    Ok(response_text)
}
