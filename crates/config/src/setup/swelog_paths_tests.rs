use std::path::Path;

use super::*;

fn get_mock_config() -> SwelogConfig {
    SwelogConfig {
        obsidian_vault_path: PathBuf::from("/home/user/vault"),
        ..SwelogConfig::get_default_config()
    }
}

#[test]
fn swelog_paths_nest_every_path_under_the_swelog_folder() {
    let config = get_mock_config();

    let swelog_paths = SwelogPaths::new(&config);

    let expected_swelog_paths = SwelogPaths {
        swelog_directory: PathBuf::from("/home/user/vault/swelog"),
        context_file: PathBuf::from("/home/user/vault/swelog/CONTEXT.md"),
        work_file: PathBuf::from("/home/user/vault/swelog/WORK.md"),
        daily_log_directory: PathBuf::from("/home/user/vault/swelog/Daily"),
        weekly_log_directory: PathBuf::from("/home/user/vault/swelog/Weekly"),
    };

    assert_eq!(swelog_paths, expected_swelog_paths);
}

#[test]
fn swelog_paths_put_the_context_file_beside_the_work_file() {
    let config = get_mock_config();

    let swelog_paths = SwelogPaths::new(&config);

    assert_eq!(swelog_paths.context_file, Path::new("/home/user/vault/swelog/CONTEXT.md"));

    assert_eq!(swelog_paths.context_file.parent(), swelog_paths.work_file.parent());
}

#[test]
fn swelog_paths_follow_the_configured_folder_and_file_names() {
    let config = SwelogConfig {
        obsidian_vault_path: PathBuf::from("/home/user/vault"),
        swelog_folder_name: String::from("accomplishments"),
        work_file_name: String::from("daily-work.md"),
        daily_log_folder_name: String::from("days"),
        weekly_log_folder_name: String::from("weeks"),
        ..SwelogConfig::get_default_config()
    };

    let swelog_paths = SwelogPaths::new(&config);

    let expected_swelog_paths = SwelogPaths {
        swelog_directory: PathBuf::from("/home/user/vault/accomplishments"),
        context_file: PathBuf::from("/home/user/vault/accomplishments/CONTEXT.md"),
        work_file: PathBuf::from("/home/user/vault/accomplishments/daily-work.md"),
        daily_log_directory: PathBuf::from("/home/user/vault/accomplishments/days"),
        weekly_log_directory: PathBuf::from("/home/user/vault/accomplishments/weeks"),
    };

    assert_eq!(swelog_paths, expected_swelog_paths);
}

#[test]
fn context_file_name_is_not_configurable() {
    let config = SwelogConfig {
        obsidian_vault_path: PathBuf::from("/home/user/vault"),
        work_file_name: String::from("daily-work.md"),
        ..SwelogConfig::get_default_config()
    };

    let swelog_paths = SwelogPaths::new(&config);

    assert_eq!(swelog_paths.context_file, Path::new("/home/user/vault/swelog/CONTEXT.md"));
}

#[test]
fn all_paths_omits_the_context_file() {
    let config = get_mock_config();

    let swelog_paths = SwelogPaths::new(&config);

    let all_paths = swelog_paths.all_paths();

    assert_eq!(
        all_paths,
        [
            &swelog_paths.work_file,
            &swelog_paths.daily_log_directory,
            &swelog_paths.weekly_log_directory
        ]
    );

    assert!(!all_paths.contains(&&swelog_paths.context_file));
}
