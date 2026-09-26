use chrono::NaiveDate;
use dates::formatting::format_date;

const DAILY_LOG_HEADING_PREFIX: &str = "# Daily Log - ";

#[must_use]
pub fn build_daily_log_content(work_file_content: &str, log_date: &NaiveDate) -> String {
    let daily_log_heading = format_daily_log_heading(*log_date);

    let daily_log_body = replace_leading_heading(work_file_content, &daily_log_heading);

    format!("{}\n", daily_log_body.trim_end())
}

fn format_daily_log_heading(log_date: NaiveDate) -> String {
    let formatted_date = format_date(&log_date);

    format!("{DAILY_LOG_HEADING_PREFIX}{formatted_date}")
}

fn replace_leading_heading(markdown: &str, heading: &str) -> String {
    let rest = markdown.lines().skip(1).collect::<Vec<_>>().join("\n");

    format!("{heading}\n{rest}")
}

#[cfg(test)]
#[path = "content_tests.rs"]
mod tests;
