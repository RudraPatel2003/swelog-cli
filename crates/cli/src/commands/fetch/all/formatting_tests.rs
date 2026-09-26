use super::*;

#[test]
fn format_fetch_source_labels_is_empty_when_there_are_no_sources() {
    assert_eq!(format_fetch_source_labels(&[]), "");
}

#[test]
fn format_fetch_source_labels_names_a_single_source() {
    assert_eq!(format_fetch_source_labels(&[FetchSource::GoogleCalendar]), "Google Calendar");
}

#[test]
fn format_fetch_source_labels_separates_sources_with_commas() {
    let fetch_sources = [FetchSource::Github, FetchSource::Linear, FetchSource::GoogleCalendar];

    assert_eq!(format_fetch_source_labels(&fetch_sources), "GitHub, Linear, Google Calendar");
}

#[test]
fn format_fetch_source_labels_keeps_the_order_it_is_given() {
    let fetch_sources = [FetchSource::Linear, FetchSource::Github];

    assert_eq!(format_fetch_source_labels(&fetch_sources), "Linear, GitHub");
}

#[test]
fn format_running_notice_lists_every_source_it_will_run() {
    let fetch_sources = [FetchSource::Github, FetchSource::GoogleCalendar];

    assert_eq!(
        format_running_notice(&fetch_sources),
        "Running the fetch commands you have configured: GitHub, Google Calendar."
    );
}

#[test]
fn format_running_notice_names_a_single_source() {
    assert_eq!(
        format_running_notice(&[FetchSource::Linear]),
        "Running the fetch commands you have configured: Linear."
    );
}
