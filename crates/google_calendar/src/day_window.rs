use chrono::{
    Days,
    NaiveDate,
    SecondsFormat,
    TimeZone,
};
use dates::formatting::format_date;
use miette::Result;

use crate::errors::GoogleCalendarDateOutOfRange;

pub struct DayWindow {
    pub time_minimum: String,
    pub time_maximum: String,
}

pub fn get_day_window(date: NaiveDate, timezone: &impl TimeZone) -> Result<DayWindow> {
    let next_date = date
        .checked_add_days(Days::new(1))
        .ok_or_else(|| GoogleCalendarDateOutOfRange { date: format_date(&date) })?;

    Ok(DayWindow {
        time_minimum: format_midnight_as_rfc3339(date, timezone)?,
        time_maximum: format_midnight_as_rfc3339(next_date, timezone)?,
    })
}

fn format_midnight_as_rfc3339(date: NaiveDate, timezone: &impl TimeZone) -> Result<String> {
    let midnight = date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| GoogleCalendarDateOutOfRange { date: format_date(&date) })?;

    let local_midnight = timezone
        .from_local_datetime(&midnight)
        .earliest()
        .ok_or_else(|| GoogleCalendarDateOutOfRange { date: format_date(&date) })?;

    Ok(local_midnight.to_rfc3339_opts(SecondsFormat::Secs, true))
}

#[cfg(test)]
#[path = "day_window_tests.rs"]
mod tests;
