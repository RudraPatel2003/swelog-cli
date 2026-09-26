use std::path::PathBuf;

use tempfile::{
    TempDir,
    tempdir,
};

use super::*;
use crate::overwrite::Overwrite;

const CONFIG_FILE_NAME: &str = "swelog.json";

const EXISTING_CONFIG_FILE_CONTENT: &str = "existing";

struct TestContext {
    temporary_directory: TempDir,
    config_file_path: PathBuf,
    config: SwelogConfig,
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let config_file_path = temporary_directory.path().join(CONFIG_FILE_NAME);

    let config = SwelogConfig::get_default_config();

    TestContext { temporary_directory, config_file_path, config }
}

#[test]
fn write_config_creates_parent_directories_and_json_file() {
    let TestContext { temporary_directory, config_file_path, config } = get_test_context();

    let overwrite_existing_config = Overwrite::No;

    write_default_config(&config_file_path, &config, overwrite_existing_config)
        .expect("config should be written");

    let file_contents =
        fs::read_to_string(&config_file_path).expect("config file should be readable");

    let parsed_config: SwelogConfig =
        serde_json::from_str(&file_contents).expect("config should contain valid JSON");

    assert_eq!(parsed_config, config);

    drop(temporary_directory);
}

#[test]
fn write_config_fails_when_file_exists_without_force() {
    let TestContext { temporary_directory, config_file_path, config } = get_test_context();

    fs::write(&config_file_path, EXISTING_CONFIG_FILE_CONTENT)
        .expect("existing config should be written");

    let overwrite_existing_config = Overwrite::No;

    let result = write_default_config(&config_file_path, &config, overwrite_existing_config);

    let error = result.expect_err("existing config should not be overwritten without force");

    let error =
        error.downcast_ref::<ConfigAlreadyExists>().expect("error should be ConfigAlreadyExists");

    assert_eq!(error.config_file_path, config_file_path);

    let file_contents =
        fs::read_to_string(&config_file_path).expect("config file should be readable");

    assert_eq!(file_contents, EXISTING_CONFIG_FILE_CONTENT,);

    drop(temporary_directory);
}

#[test]
fn write_config_overwrites_when_force_is_set() {
    let TestContext { temporary_directory, config_file_path, config } = get_test_context();

    fs::write(&config_file_path, EXISTING_CONFIG_FILE_CONTENT)
        .expect("existing config should be written");

    let overwrite_existing_config = Overwrite::Yes;

    let result = write_default_config(&config_file_path, &config, overwrite_existing_config);

    assert!(result.is_ok());

    let file_contents =
        fs::read_to_string(&config_file_path).expect("config file should be readable");

    let parsed_config: SwelogConfig =
        serde_json::from_str(&file_contents).expect("config should contain valid JSON");

    assert_eq!(parsed_config, config);

    drop(temporary_directory);
}

#[test]
fn write_default_config_omits_optional_fields() {
    let TestContext { temporary_directory, config_file_path, config } = get_test_context();

    write_default_config(&config_file_path, &config, Overwrite::No)
        .expect("config should be written");

    let file_contents =
        fs::read_to_string(&config_file_path).expect("config file should be readable");

    assert!(!file_contents.contains("llm"));

    assert!(!file_contents.contains("llmModel"));

    assert!(!file_contents.contains("linearUsername"));

    drop(temporary_directory);
}
