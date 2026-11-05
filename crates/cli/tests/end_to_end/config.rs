use std::fs;

use config::swelog_config::SwelogConfig;
use predicates::str::contains;

use crate::support::sandbox::{
    DEFAULT_WORK_FILE_CONTENT,
    SwelogSandbox,
};

#[test]
fn init_creates_a_default_config_file_at_the_configured_path() {
    let sandbox = SwelogSandbox::new();

    fs::remove_file(sandbox.config_file_path()).expect("config file should be removed");

    sandbox
        .swelog()
        .arg("init")
        .assert()
        .success()
        .stdout(contains("Created swelog config at"))
        .stdout(contains(
            sandbox
                .config_file_path()
                .display()
                .to_string(),
        ));

    let config_json =
        fs::read_to_string(sandbox.config_file_path()).expect("config file should be readable");

    let config: SwelogConfig = serde_json::from_str(&config_json).expect("config should parse");

    assert_eq!(config, SwelogConfig::get_default_config());
}

#[test]
fn init_fails_when_the_config_file_exists_without_force() {
    let sandbox = SwelogSandbox::new();

    sandbox
        .swelog()
        .arg("init")
        .assert()
        .failure()
        .stderr(contains("config already exists at"));

    assert_eq!(sandbox.read_config(), sandbox.default_config());
}

#[test]
fn init_overwrites_the_config_file_with_force() {
    let sandbox = SwelogSandbox::new();

    sandbox
        .swelog()
        .args(["init", "--force"])
        .assert()
        .success();

    assert_eq!(sandbox.read_config(), SwelogConfig::get_default_config());
}

#[test]
fn config_prints_the_config_and_where_it_is_stored() {
    let sandbox = SwelogSandbox::new();

    sandbox
        .swelog()
        .arg("config")
        .assert()
        .success()
        .stdout(contains(format!(
            "Displaying config at {}:",
            sandbox
                .config_file_path()
                .display()
        )))
        .stdout(contains(format!(
            "Obsidian vault path  {}",
            sandbox
                .vault_directory()
                .display()
        )))
        .stdout(contains("Swelog folder name   swelog"));
}

#[test]
fn config_flag_takes_precedence_over_the_environment_variable() {
    let sandbox = SwelogSandbox::new();

    let other_config_file_path = sandbox
        .cache_directory()
        .join("other")
        .join("swelog.json");

    let other_config =
        SwelogConfig { swelog_folder_name: String::from("elsewhere"), ..sandbox.default_config() };

    fs::create_dir_all(
        other_config_file_path
            .parent()
            .expect("path should have a parent"),
    )
    .expect("other config directory should be created");

    fs::write(
        &other_config_file_path,
        serde_json::to_string(&other_config).expect("config should serialize"),
    )
    .expect("other config should be written");

    sandbox
        .swelog()
        .args([
            "--config",
            &other_config_file_path
                .display()
                .to_string(),
            "config",
        ])
        .assert()
        .success()
        .stdout(contains(format!("Displaying config at {}:", other_config_file_path.display())))
        .stdout(contains("Swelog folder name   elsewhere"));
}

#[test]
fn commands_fail_when_the_config_file_is_missing() {
    let sandbox = SwelogSandbox::new();

    fs::remove_file(sandbox.config_file_path()).expect("config file should be removed");

    sandbox
        .swelog()
        .arg("setup")
        .assert()
        .failure()
        .stderr(contains("config not found at"))
        .stderr(contains("run `swelog init` to create a config file"));
}

#[test]
fn setup_creates_the_swelog_files_in_the_vault() {
    let sandbox = SwelogSandbox::new();

    sandbox
        .swelog()
        .arg("setup")
        .assert()
        .success()
        .stdout(contains("Created swelog files in your Obsidian vault at"));

    let swelog_paths = sandbox.swelog_paths();

    assert_eq!(sandbox.read_work_file(), DEFAULT_WORK_FILE_CONTENT);

    assert!(
        swelog_paths
            .daily_log_directory
            .is_dir()
    );

    assert!(
        swelog_paths
            .weekly_log_directory
            .is_dir()
    );
}

#[test]
fn setup_fails_when_the_swelog_files_exist_without_force() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox.write_work_file("edited");

    sandbox
        .swelog()
        .arg("setup")
        .assert()
        .failure()
        .stderr(contains("swelog setup files already exist at"));

    assert_eq!(sandbox.read_work_file(), "edited");
}

#[test]
fn setup_overwrites_the_work_file_with_force() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox.write_work_file("edited");

    sandbox
        .swelog()
        .args(["setup", "--force"])
        .assert()
        .success();

    assert_eq!(sandbox.read_work_file(), DEFAULT_WORK_FILE_CONTENT);
}
