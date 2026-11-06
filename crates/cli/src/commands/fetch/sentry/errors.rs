use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Sentry organization is not configured")]
#[diagnostic(
    code(swelog::sentry::missing_organization),
    help(
        "set integrations.sentry.organization in your swelog config before running `swelog fetch sentry`"
    )
)]
pub struct MissingSentryOrganization;
