use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("This build of swelog has no Google OAuth client")]
#[diagnostic(
    code(swelog::google_calendar::oauth_application_missing),
    help(
        "install an official swelog release, or rebuild with SWELOG_GOOGLE_CLIENT_ID and SWELOG_GOOGLE_CLIENT_SECRET set to a Google Cloud desktop app client"
    )
)]
pub struct GoogleOAuthApplicationMissing;

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to authorize with Google: {message}")]
#[diagnostic(
    code(swelog::google_calendar::authorization_failed),
    help(
        "run `swelog auth clear google-calendar`, then run `swelog fetch google-calendar` to authorize again"
    )
)]
pub struct GoogleAuthorizationFailed {
    pub message: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Timed out waiting for Google authorization")]
#[diagnostic(
    code(swelog::google_calendar::authorization_timed_out),
    help("run the command again and complete authorization in the browser")
)]
pub struct GoogleAuthorizationTimedOut;

#[derive(Debug, Diagnostic, Error)]
#[error("Google authorization was denied: {reason}")]
#[diagnostic(
    code(swelog::google_calendar::authorization_denied),
    help("run the command again and grant swelog access to your calendar")
)]
pub struct GoogleAuthorizationDenied {
    pub reason: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Google returned a redirect that swelog did not start")]
#[diagnostic(
    code(swelog::google_calendar::callback_state_mismatch),
    help("run the command again and complete authorization in the browser swelog opened")
)]
pub struct GoogleCallbackStateMismatch;

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to send Google Calendar request")]
#[diagnostic(
    code(swelog::google_calendar::failed_to_send_google_calendar_request),
    help("check your network connection and try again")
)]
pub struct FailedToSendGoogleCalendarRequest;

#[derive(Debug, Diagnostic, Error)]
#[error("Google Calendar returned an unsupported response: {message}")]
#[diagnostic(code(swelog::google_calendar::unsupported_response))]
pub struct UnsupportedGoogleCalendarResponse {
    pub message: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("`{date}` is outside the range of dates Google Calendar can be searched for")]
#[diagnostic(
    code(swelog::google_calendar::date_out_of_range),
    help("choose a date close to today, such as 08-17-2026")
)]
pub struct GoogleCalendarDateOutOfRange {
    pub date: String,
}
