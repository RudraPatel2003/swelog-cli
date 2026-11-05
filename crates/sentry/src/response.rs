use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Response,
    StatusCode,
    header::LINK,
};

use crate::{
    errors::{
        SentryAuthorizationFailed,
        UnsuccessfulSentryResponse,
    },
    pagination::parse_next_cursor,
};

pub struct ResponsePage {
    pub body: String,
    pub next_cursor: Option<String>,
}

/// Instruct user to clear auth token if response is unauthorized or forbidden
pub async fn read_successful_response_page(response: Response) -> Result<ResponsePage> {
    let status_code = response.status();

    let next_cursor = read_next_cursor(&response);

    let response_text =
        response.text().await.into_diagnostic().wrap_err("failed to read Sentry response body")?;

    if matches!(status_code, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) {
        let sentry_authorization_failed_error = SentryAuthorizationFailed { status_code };

        return Err(sentry_authorization_failed_error.into());
    }

    if !status_code.is_success() {
        let unsuccessful_sentry_response_error =
            UnsuccessfulSentryResponse { status_code, response_text };

        return Err(unsuccessful_sentry_response_error.into());
    }

    Ok(ResponsePage { body: response_text, next_cursor })
}

fn read_next_cursor(response: &Response) -> Option<String> {
    let link_header = response.headers().get(LINK)?.to_str().ok()?;

    parse_next_cursor(link_header)
}
