use chrono::{
    DateTime,
    Days,
    NaiveDate,
    NaiveTime,
    TimeZone,
    Utc,
};
use dates::formatting::format_date;
use miette::Result;

use crate::errors::SentryDateOutOfRange;

const SENTRY_FILTER_DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

/// The UTC instants that bound a calendar date read in a time zone, so Sentry
/// only returns the issues that received events on that date.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivityWindow {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl ActivityWindow {
    #[must_use]
    pub fn format_start(&self) -> String {
        self.start
            .format(SENTRY_FILTER_DATETIME_FORMAT)
            .to_string()
    }

    #[must_use]
    pub fn format_end(&self) -> String {
        self.end
            .format(SENTRY_FILTER_DATETIME_FORMAT)
            .to_string()
    }
}

pub fn get_activity_window(date: NaiveDate, timezone: &impl TimeZone) -> Result<ActivityWindow> {
    let following_date = date
        .checked_add_days(Days::new(1))
        .ok_or_else(|| SentryDateOutOfRange { date: format_date(&date) })?;

    let start = get_start_of_date(date, timezone)?;

    let end = get_start_of_date(following_date, timezone)?;

    Ok(ActivityWindow { start, end })
}

fn get_start_of_date(date: NaiveDate, timezone: &impl TimeZone) -> Result<DateTime<Utc>> {
    let start_of_date = timezone
        .from_local_datetime(&date.and_time(NaiveTime::MIN))
        .earliest()
        .ok_or_else(|| SentryDateOutOfRange { date: format_date(&date) })?;

    Ok(start_of_date.to_utc())
}

#[cfg(test)]
#[path = "activity_tests.rs"]
mod tests;
