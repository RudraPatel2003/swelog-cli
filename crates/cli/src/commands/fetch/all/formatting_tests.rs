use super::*;
use crate::commands::fetch::{
    github::GithubIntegration,
    google_calendar::GoogleCalendarIntegration,
    linear::LinearIntegration,
};

#[test]
fn format_integration_labels_is_empty_when_there_are_no_integrations() {
    assert_eq!(format_integration_labels(&[]), "");
}

#[test]
fn format_integration_labels_names_a_single_integration() {
    assert_eq!(format_integration_labels(&[&GoogleCalendarIntegration]), "Google Calendar");
}

#[test]
fn format_integration_labels_separates_integrations_with_commas() {
    let integrations: [&dyn Integration; 3] =
        [&GithubIntegration, &LinearIntegration, &GoogleCalendarIntegration];

    assert_eq!(format_integration_labels(&integrations), "GitHub, Linear, Google Calendar");
}

#[test]
fn format_integration_labels_keeps_the_order_it_is_given() {
    let integrations: [&dyn Integration; 2] = [&LinearIntegration, &GithubIntegration];

    assert_eq!(format_integration_labels(&integrations), "Linear, GitHub");
}

#[test]
fn format_running_notice_lists_every_integration_it_will_run() {
    let integrations: [&dyn Integration; 2] = [&GithubIntegration, &GoogleCalendarIntegration];

    assert_eq!(
        format_running_notice(&integrations),
        "Running the fetch commands you have configured: GitHub, Google Calendar."
    );
}

#[test]
fn format_running_notice_names_a_single_integration() {
    assert_eq!(
        format_running_notice(&[&LinearIntegration]),
        "Running the fetch commands you have configured: Linear."
    );
}
