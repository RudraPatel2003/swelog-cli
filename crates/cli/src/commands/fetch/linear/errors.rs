use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Linear username is not configured")]
#[diagnostic(
    code(swelog::linear::missing_username),
    help(
        "set integrations.linear.username in your swelog config before running `swelog fetch linear`"
    )
)]
pub struct MissingLinearUsername;
