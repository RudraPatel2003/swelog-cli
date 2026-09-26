use super::*;

const CURRENT_VERSION: &str = "0.10.0";

const NEWER_VERSION: &str = "0.11.0";

const UNPARSEABLE_VERSION: &str = "not-a-version";

#[test]
fn newer_version_is_detected_when_the_latest_version_is_greater() {
    assert!(is_newer_version(CURRENT_VERSION, NEWER_VERSION));
}

#[test]
fn newer_version_is_detected_when_the_latest_minor_version_has_more_digits() {
    // A string comparison would call "0.9.0" the newer of the two.
    assert!(is_newer_version("0.9.0", "0.10.0"));
}

#[test]
fn newer_version_is_not_detected_when_the_versions_match() {
    assert!(!is_newer_version(CURRENT_VERSION, CURRENT_VERSION));
}

#[test]
fn newer_version_is_not_detected_when_the_current_version_is_ahead() {
    assert!(!is_newer_version(NEWER_VERSION, CURRENT_VERSION));
}

#[test]
fn newer_version_is_not_detected_when_a_version_cannot_be_parsed() {
    assert!(!is_newer_version(CURRENT_VERSION, UNPARSEABLE_VERSION));

    assert!(!is_newer_version(UNPARSEABLE_VERSION, NEWER_VERSION));
}

#[test]
fn update_notice_contains_both_versions_and_the_upgrade_command() {
    let notice =
        get_update_notice(CURRENT_VERSION, Some(NEWER_VERSION)).expect("notice should be built");

    assert!(notice.contains(CURRENT_VERSION));

    assert!(notice.contains(NEWER_VERSION));

    assert!(notice.contains(UPGRADE_COMMAND));
}

#[test]
fn there_is_no_update_notice_when_swelog_is_already_current() {
    assert_eq!(get_update_notice(CURRENT_VERSION, Some(CURRENT_VERSION)), None);
}

#[test]
fn there_is_no_update_notice_when_no_version_has_been_fetched() {
    assert_eq!(get_update_notice(CURRENT_VERSION, None), None);
}
