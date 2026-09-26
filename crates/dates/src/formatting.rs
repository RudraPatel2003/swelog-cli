#[cfg(test)]
#[path = "formatting_tests.rs"]
mod tests;

use chrono::NaiveDate;

use crate::date_format::DATE_FORMAT;

#[must_use]
pub fn format_date(date: &NaiveDate) -> String {
    date.format(DATE_FORMAT).to_string()
}
