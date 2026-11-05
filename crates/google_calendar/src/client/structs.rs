use chrono::{
    DateTime,
    Local,
};
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Meeting {
    pub title: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub status: MeetingStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeetingStatus {
    Scheduled,
    Cancelled,
    Declined,
}

impl MeetingStatus {
    #[must_use]
    pub const fn was_attended(self) -> bool {
        matches!(self, Self::Scheduled)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventPage {
    #[serde(default)]
    pub items: Vec<CalendarEvent>,

    #[serde(default)]
    next_page_token: Option<String>,
}

impl CalendarEventPage {
    pub fn take_next_page_token(&mut self) -> Option<String> {
        self.next_page_token
            .take()
            .filter(|next_page_token| !next_page_token.is_empty())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    #[serde(default)]
    pub(crate) summary: Option<String>,

    #[serde(default)]
    pub(crate) status: Option<String>,

    #[serde(default)]
    pub(crate) start: Option<CalendarEventTime>,

    #[serde(default)]
    pub(crate) end: Option<CalendarEventTime>,

    #[serde(default)]
    pub(crate) attendees: Vec<CalendarEventAttendee>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventTime {
    // Absent on all-day events, which carry a `date` instead.
    #[serde(default)]
    pub(crate) date_time: Option<DateTime<Local>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventAttendee {
    #[serde(default, rename = "self")]
    pub(crate) is_self: bool,

    #[serde(default)]
    pub(crate) response_status: Option<String>,
}
