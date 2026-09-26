use super::*;

fn get_config_with_linear_username(linear_username: Option<&str>) -> SwelogConfig {
    SwelogConfig {
        linear_username: linear_username.map(str::to_string),
        ..SwelogConfig::get_default_config()
    }
}

#[test]
fn get_fetch_source_availability_includes_github_when_authorized() {
    let swelog_config = get_config_with_linear_username(None);

    let availability = get_fetch_source_availability(
        FetchSource::Github,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, FetchSourceAvailability::Included);
}

#[test]
fn get_fetch_source_availability_excludes_source_without_authorization() {
    let swelog_config = get_config_with_linear_username(Some("rudra"));

    let availability = get_fetch_source_availability(
        FetchSource::Linear,
        &swelog_config,
        AuthorizationPresence::Missing,
    );

    assert_eq!(availability, FetchSourceAvailability::MissingAuthorization);
}

#[test]
fn get_fetch_source_availability_excludes_linear_without_a_username() {
    let swelog_config = get_config_with_linear_username(None);

    let availability = get_fetch_source_availability(
        FetchSource::Linear,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(
        availability,
        FetchSourceAvailability::MissingConfiguration {
            reason: "linearUsername is not configured"
        }
    );
}

#[test]
fn get_fetch_source_availability_excludes_linear_when_the_username_is_blank() {
    let swelog_config = get_config_with_linear_username(Some("   "));

    let availability = get_fetch_source_availability(
        FetchSource::Linear,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(
        availability,
        FetchSourceAvailability::MissingConfiguration {
            reason: "linearUsername is not configured"
        }
    );
}

#[test]
fn describe_missing_configuration_is_empty_for_a_source_with_no_configuration() {
    let swelog_config = get_config_with_linear_username(None);

    assert_eq!(FetchSource::Github.describe_missing_configuration(&swelog_config), None);

    assert_eq!(FetchSource::GoogleCalendar.describe_missing_configuration(&swelog_config), None);
}

#[test]
fn get_fetch_source_availability_includes_linear_with_a_username() {
    let swelog_config = get_config_with_linear_username(Some("rudra"));

    let availability = get_fetch_source_availability(
        FetchSource::Linear,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, FetchSourceAvailability::Included);
}

#[test]
fn get_fetch_source_availability_ignores_the_linear_username_for_google_calendar() {
    let swelog_config = get_config_with_linear_username(None);

    let availability = get_fetch_source_availability(
        FetchSource::GoogleCalendar,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, FetchSourceAvailability::Included);
}
