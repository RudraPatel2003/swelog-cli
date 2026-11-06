use clap::Args;
use credentials::credential::Credential;
use miette::Result;

use crate::environment::Environment;

#[derive(Debug, Args)]
pub struct ClearArgs {
    /// Credential to remove from your operating system keyring.
    #[arg(value_name = "CREDENTIAL", required_unless_present = "all")]
    credential: Option<Credential>,

    /// Remove every credential swelog has stored.
    #[arg(long, conflicts_with = "credential")]
    all: bool,
}

impl ClearArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let credentials = self
            .credential
            .map_or_else(|| Credential::all().to_vec(), |credential| vec![credential]);

        for credential in credentials {
            let was_credential_cleared = environment
                .credential_store
                .clear(credential)?;

            if was_credential_cleared {
                println!(
                    "Removed the stored {}.",
                    credential
                        .spec()
                        .label
                );
            } else {
                println!(
                    "No {} was stored.",
                    credential
                        .spec()
                        .label
                );
            }
        }

        Ok(())
    }
}
