use crate::client::structs::{
    CalendarEvent,
    Meeting,
    MeetingStatus,
};

const UNTITLED_EVENT_TITLE: &str = "(No title)";

const CANCELLED_STATUS: &str = "cancelled";

const DECLINED_RESPONSE_STATUS: &str = "declined";

pub fn collect_meetings(events: &[CalendarEvent]) -> Vec<Meeting> {
    events.iter().filter_map(to_timed_meeting).collect()
}

fn to_timed_meeting(event: &CalendarEvent) -> Option<Meeting> {
    let start = event.start.as_ref().and_then(|start| start.date_time)?;

    let end = event.end.as_ref().and_then(|end| end.date_time)?;

    Some(Meeting {
        title: get_event_title_or_placeholder(event),
        start,
        end,
        status: get_meeting_status(event),
    })
}

fn get_meeting_status(event: &CalendarEvent) -> MeetingStatus {
    if is_cancelled(event) {
        return MeetingStatus::Cancelled;
    }

    if was_declined(event) {
        return MeetingStatus::Declined;
    }

    MeetingStatus::Scheduled
}

fn is_cancelled(event: &CalendarEvent) -> bool {
    event.status.as_deref() == Some(CANCELLED_STATUS)
}

fn was_declined(event: &CalendarEvent) -> bool {
    event
        .attendees
        .iter()
        .filter(|attendee| attendee.is_self)
        .any(|attendee| attendee.response_status.as_deref() == Some(DECLINED_RESPONSE_STATUS))
}

fn get_event_title_or_placeholder(event: &CalendarEvent) -> String {
    let title = event.summary.as_deref().map(collapse_whitespace).unwrap_or_default();

    if title.is_empty() { UNTITLED_EVENT_TITLE.to_string() } else { title }
}

fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
#[path = "meetings_tests.rs"]
mod tests;
