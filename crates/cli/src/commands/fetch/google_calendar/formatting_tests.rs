use chrono::{
    NaiveDate,
    TimeZone,
};
use google_calendar::client::structs::MeetingStatus;

use super::*;

fn get_mock_meeting(title: &str, start_hour: u32, end_hour: u32, status: MeetingStatus) -> Meeting {
    let mock_start_time = get_mock_local_date_time(start_hour);

    let mock_end_time = get_mock_local_date_time(end_hour);

    Meeting { title: title.to_string(), start: mock_start_time, end: mock_end_time, status }
}

fn get_mock_local_date_time(hour: u32) -> DateTime<Local> {
    let naive_date_time = NaiveDate::from_ymd_opt(2026, 8, 17)
        .expect("date should be valid")
        .and_hms_opt(hour, 0, 0)
        .expect("time should be valid");

    Local
        .from_local_datetime(&naive_date_time)
        .earliest()
        .expect("local date time should be unambiguous")
}

#[test]
fn format_meetings_separates_the_time_range_from_the_title_with_a_pipe() {
    let mock_meeting = get_mock_meeting("Standup", 10, 11, MeetingStatus::Scheduled);

    let meetings = vec![mock_meeting];

    assert_eq!(format_meetings(&meetings), "- 10:00 AM - 11:00 AM | Standup");
}

#[test]
fn format_meetings_strikes_through_a_declined_meeting() {
    let mock_meeting = get_mock_meeting("Eng all-hands", 11, 12, MeetingStatus::Declined);

    let meetings = vec![mock_meeting];

    assert_eq!(format_meetings(&meetings), "- ~~11:00 AM - 12:00 PM | Eng all-hands~~");
}

#[test]
fn format_meetings_strikes_through_a_cancelled_meeting() {
    let mock_meeting = get_mock_meeting("Cancelled 1:1", 9, 10, MeetingStatus::Cancelled);

    let meetings = vec![mock_meeting];

    assert_eq!(format_meetings(&meetings), "- ~~9:00 AM - 10:00 AM | Cancelled 1:1~~");
}

const MEETINGS_ORDERED_BY_START_TIME: &str = "- 10:00 AM - 11:00 AM | Standup
- 1:00 PM - 2:00 PM | Design review";

#[test]
fn format_meetings_orders_meetings_by_start_time() {
    let design_review = get_mock_meeting("Design review", 13, 14, MeetingStatus::Scheduled);

    let standup = get_mock_meeting("Standup", 10, 11, MeetingStatus::Scheduled);

    let meetings = vec![design_review, standup];

    assert_eq!(format_meetings(&meetings), MEETINGS_ORDERED_BY_START_TIME);
}

const STRUCK_THROUGH_MEETING_IN_ITS_TIME_SLOT: &str = "- 10:00 AM - 11:00 AM | Standup
- ~~11:00 AM - 12:00 PM | Eng all-hands~~
- 1:00 PM - 2:00 PM | Design review";

#[test]
fn format_meetings_keeps_a_struck_through_meeting_in_its_time_slot() {
    let design_review = get_mock_meeting("Design review", 13, 14, MeetingStatus::Scheduled);

    let eng_all_hands = get_mock_meeting("Eng all-hands", 11, 12, MeetingStatus::Declined);

    let standup = get_mock_meeting("Standup", 10, 11, MeetingStatus::Scheduled);

    let meetings = vec![design_review, eng_all_hands, standup];

    assert_eq!(format_meetings(&meetings), STRUCK_THROUGH_MEETING_IN_ITS_TIME_SLOT);
}

#[test]
fn format_meetings_writes_nothing_when_there_are_no_meetings() {
    assert_eq!(format_meetings(&[]), "");
}
