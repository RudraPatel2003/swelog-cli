mod cli;
mod commands;
mod environment;
mod error_report;
mod shared;

use clap::Parser;
use cli::Cli;
use commands::Commands;
use environment::{
    Environment,
    resolve_environment,
    update_check::UpdateCheck,
};
use error_report::configure_miette_error_handling;
use miette::Result;
use updates::check::{
    PendingUpdateNotice,
    print_update_notice,
    start_version_check,
};

#[tokio::main]
async fn main() -> Result<()> {
    configure_miette_error_handling()?;

    let cli = Cli::parse();

    let environment = resolve_environment(cli.global_args)?;

    let pending_update_notice = start_update_check(&environment);

    // Store result so update is printed even if command fails
    let command_result = run_command(cli.command, &environment).await;

    print_update_notice(pending_update_notice).await;

    command_result
}

fn start_update_check(environment: &Environment) -> PendingUpdateNotice {
    let cargo_package_version = env!("CARGO_PKG_VERSION");

    match environment.update_check {
        UpdateCheck::On => start_version_check(
            cargo_package_version,
            &environment.cache_directory,
            environment.endpoints.npm_registry.clone(),
        ),

        UpdateCheck::Off => PendingUpdateNotice::skipped(),
    }
}

async fn run_command(command: Commands, environment: &Environment) -> Result<()> {
    match command {
        Commands::Init(init_args) => init_args.run(environment),

        Commands::Setup(setup_args) => setup_args.run(environment),

        Commands::Summarize(summarize_args) => summarize_args.run(environment).await,

        Commands::Log(log_args) => log_args.run(environment),

        Commands::Reset(reset_args) => reset_args.run(environment),

        Commands::Undo(undo_args) => undo_args.run(environment),

        Commands::Fetch(fetch_args) => fetch_args.run(environment).await,

        Commands::Config(config_args) => config_args.run(environment),

        Commands::Auth(auth_args) => auth_args.run(environment),
    }
}
