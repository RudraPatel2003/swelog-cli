#[cfg(test)]
#[path = "parsing_tests.rs"]
mod tests;

use chrono::{
    Datelike,
    NaiveDate,
    Weekday,
};
use miette::Result;

use crate::{
    date_format::DATE_FORMAT,
    errors::DateParseError,
};

pub fn parse_date(date: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(date, DATE_FORMAT)
        .map_err(|_| DateParseError::InvalidDate { date: date.to_string() }.into())
}

pub fn parse_monday_date(date: &str) -> Result<NaiveDate> {
    let monday_date = parse_date(date)?;

    if monday_date.weekday() != Weekday::Mon {
        return Err(DateParseError::DateIsNotAMonday { date: date.to_string() }.into());
    }

    Ok(monday_date)
}
