use miette::Diagnostic;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to send Sentry request")]
#[diagnostic(
    code(swelog::sentry::failed_to_send_sentry_request),
    help("check your Sentry auth token and network connection and try again")
)]
pub struct FailedToSendSentryRequest;

#[derive(Debug, Diagnostic, Error)]
#[error("Unsuccessful Sentry response with status {status_code} and body {response_text}")]
#[diagnostic(code(swelog::sentry::unsuccessful_sentry_response))]
pub struct UnsuccessfulSentryResponse {
    pub status_code: StatusCode,
    pub response_text: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Sentry rejected your auth token with status {status_code}")]
#[diagnostic(
    code(swelog::sentry::authorization_failed),
    help(
        "run `swelog auth clear sentry` and run the command again to enter a new token, or check that your token has the `event:read` scope"
    )
)]
pub struct SentryAuthorizationFailed {
    pub status_code: StatusCode,
}

#[derive(Debug, Diagnostic, Error)]
#[error("`{date}` is outside the range of dates Sentry can be searched for")]
#[diagnostic(
    code(swelog::sentry::date_out_of_range),
    help("choose a date close to today, such as 08-17-2026")
)]
pub struct SentryDateOutOfRange {
    pub date: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Sentry returned a repeated pagination cursor")]
#[diagnostic(
    code(swelog::sentry::repeated_pagination_cursor),
    help("run the command again, and report the problem if it keeps happening")
)]
pub struct RepeatedSentryPaginationCursor;
