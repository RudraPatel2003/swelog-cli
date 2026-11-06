use super::*;

fn get_integrations_config(
    linear_username: Option<&str>,
    sentry_organization: Option<&str>,
) -> IntegrationsConfig {
    IntegrationsConfig {
        linear: linear_username.map(|username| LinearConfig { username: username.to_string() }),
        sentry: sentry_organization
            .map(|organization| SentryConfig { organization: organization.to_string() }),
    }
}

#[test]
fn integrations_are_read_from_their_nested_sections() {
    let integrations_config: IntegrationsConfig = serde_json::from_str(
        r#"
            {
              "linear": { "username": "rudrapatel" },
              "sentry": { "organization": "acme" }
            }
        "#,
    )
    .expect("integrations config should parse");

    assert_eq!(integrations_config, get_integrations_config(Some("rudrapatel"), Some("acme")));
}

#[test]
fn missing_integration_sections_are_not_configured() {
    let integrations_config: IntegrationsConfig =
        serde_json::from_str("{}").expect("integrations config should parse");

    assert!(integrations_config.is_empty());

    assert_eq!(integrations_config.get_linear_username(), None);

    assert_eq!(integrations_config.get_sentry_organization(), None);
}

#[test]
fn integration_values_are_trimmed() {
    let integrations_config = get_integrations_config(Some("  rudrapatel "), Some(" acme  "));

    assert_eq!(integrations_config.get_linear_username(), Some("rudrapatel"));

    assert_eq!(integrations_config.get_sentry_organization(), Some("acme"));
}

#[test]
fn blank_integration_values_are_not_configured() {
    let integrations_config = get_integrations_config(Some("   "), Some(""));

    assert!(!integrations_config.is_empty());

    assert_eq!(integrations_config.get_linear_username(), None);

    assert_eq!(integrations_config.get_sentry_organization(), None);
}

#[test]
fn empty_integrations_are_left_out_of_the_serialized_config() {
    let serialized_integrations = serde_json::to_string(&IntegrationsConfig::default())
        .expect("integrations config should serialize");

    assert_eq!(serialized_integrations, "{}");
}
