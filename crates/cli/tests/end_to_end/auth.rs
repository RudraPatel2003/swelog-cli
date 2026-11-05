use credentials::credential::Credential;
use predicates::str::contains;

use crate::support::{
    anthropic::ANTHROPIC_API_KEY,
    sandbox::{
        GITHUB_TOKEN,
        SwelogSandbox,
    },
};

#[test]
fn auth_status_reports_stored_and_environment_credentials() {
    let sandbox = SwelogSandbox::new();

    sandbox.store_credential(Credential::Anthropic, ANTHROPIC_API_KEY);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .args(["auth", "status"])
        .assert()
        .success()
        .stdout(contains(format!(
            "Credentials stored in the credential file at {}:",
            sandbox
                .credential_file()
                .display()
        )))
        .stdout(contains("GitHub token                   set by $GITHUB_TOKEN"))
        .stdout(contains("Anthropic API key              stored"))
        .stdout(contains("OpenAI API key                 not stored"));
}

#[test]
fn auth_clear_removes_a_stored_credential() {
    let sandbox = SwelogSandbox::new();

    sandbox.store_credential(Credential::Github, GITHUB_TOKEN);

    sandbox
        .swelog()
        .args(["auth", "clear", "github"])
        .assert()
        .success()
        .stdout(contains("Removed the stored GitHub token."));

    assert_eq!(sandbox.read_credential(Credential::Github), None);

    sandbox
        .swelog()
        .args(["auth", "clear", "github"])
        .assert()
        .success()
        .stdout(contains("No GitHub token was stored."));
}

#[test]
fn auth_clear_all_removes_every_stored_credential() {
    let sandbox = SwelogSandbox::new();

    sandbox.store_credential(Credential::Github, GITHUB_TOKEN);

    sandbox.store_credential(Credential::Anthropic, ANTHROPIC_API_KEY);

    sandbox
        .swelog()
        .args(["auth", "clear", "--all"])
        .assert()
        .success()
        .stdout(contains("Removed the stored GitHub token."))
        .stdout(contains("Removed the stored Anthropic API key."))
        .stdout(contains("No OpenAI API key was stored."));

    assert_eq!(sandbox.read_credential(Credential::Github), None);

    assert_eq!(sandbox.read_credential(Credential::Anthropic), None);
}
