use miette::Diagnostic;
use thiserror::Error;

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
