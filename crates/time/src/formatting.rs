#[cfg(test)]
#[path = "formatting_tests.rs"]
mod tests;

use chrono::NaiveTime;

use crate::time_format::TIME_FORMAT;

#[must_use]
pub fn format_time(time: &NaiveTime) -> String {
    time.format(TIME_FORMAT).to_string()
}
