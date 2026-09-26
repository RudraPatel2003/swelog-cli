use chrono::{
    DateTime,
    Local,
};
use google_calendar::client::structs::Meeting;
use time::formatting::format_time;

pub fn format_meetings(meetings: &[Meeting]) -> String {
    let mut sorted_meetings = meetings.iter().collect::<Vec<_>>();

    sorted_meetings.sort_by_key(|meeting| meeting.start);

    sorted_meetings.iter().map(|meeting| format_meeting(meeting)).collect::<Vec<_>>().join("\n")
}

fn format_meeting(meeting: &Meeting) -> String {
    let entry = format!("{} | {}", format_time_range(meeting.start, meeting.end), meeting.title);

    if meeting.status.was_attended() {
        return format!("- {entry}");
    }

    format!("- ~~{entry}~~")
}

fn format_time_range(start: DateTime<Local>, end: DateTime<Local>) -> String {
    format!("{} - {}", format_time(&start.time()), format_time(&end.time()))
}

#[cfg(test)]
#[path = "formatting_tests.rs"]
mod tests;
