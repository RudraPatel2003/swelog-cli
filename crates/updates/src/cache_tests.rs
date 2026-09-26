use chrono::TimeDelta;
use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const LATEST_VERSION: &str = "0.11.0";

const NEWER_LATEST_VERSION: &str = "0.12.0";

const ONE_HOUR_IN_SECONDS: i64 = 3_600;

const TWO_DAYS_IN_SECONDS: i64 = 172_800;

struct TestContext {
    temporary_directory: TempDir,
    cache_file_path: PathBuf,
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let cache_file_path = get_version_cache_file_path(&temporary_directory.path().join("cache"));

    TestContext { temporary_directory, cache_file_path }
}

fn get_mock_version_cache(latest_version: &str, checked_seconds_ago: i64) -> VersionCache {
    let checked_at = Utc::now()
        .checked_sub_signed(TimeDelta::seconds(checked_seconds_ago))
        .expect("timestamp should be representable");

    VersionCache { latest_version: String::from(latest_version), checked_at }
}

#[test]
fn read_version_cache_fails_when_the_cache_file_is_missing() {
    let TestContext { temporary_directory, cache_file_path } = get_test_context();

    let result = read_version_cache(&cache_file_path);

    assert!(result.is_err());

    drop(temporary_directory);
}

#[test]
fn write_version_cache_creates_parent_directories() {
    let TestContext { temporary_directory, cache_file_path } = get_test_context();

    let version_cache = get_mock_version_cache(LATEST_VERSION, 0);

    write_version_cache(&cache_file_path, &version_cache).expect("cache should be written");

    let written_cache = read_version_cache(&cache_file_path).expect("cache should be readable");

    assert_eq!(written_cache, version_cache);

    drop(temporary_directory);
}

#[test]
fn write_version_cache_replaces_an_existing_cache_file() {
    let TestContext { temporary_directory, cache_file_path } = get_test_context();

    let version_cache = get_mock_version_cache(LATEST_VERSION, TWO_DAYS_IN_SECONDS);

    write_version_cache(&cache_file_path, &version_cache).expect("cache should be written");

    let newer_version_cache = get_mock_version_cache(NEWER_LATEST_VERSION, 0);

    write_version_cache(&cache_file_path, &newer_version_cache)
        .expect("cache should be overwritten");

    let written_cache = read_version_cache(&cache_file_path).expect("cache should be readable");

    assert_eq!(written_cache, newer_version_cache);

    drop(temporary_directory);
}

#[test]
fn write_version_cache_leaves_no_temporary_file_behind() {
    let TestContext { temporary_directory, cache_file_path } = get_test_context();

    let version_cache = get_mock_version_cache(LATEST_VERSION, 0);

    write_version_cache(&cache_file_path, &version_cache).expect("cache should be written");

    let cache_directory = cache_file_path.parent().expect("cache file should have a parent");

    let entries =
        fs::read_dir(cache_directory).expect("cache directory should be readable").count();

    assert_eq!(entries, 1);

    drop(temporary_directory);
}

#[test]
fn refresh_is_due_when_last_checked_more_than_a_day_ago() {
    let version_cache = get_mock_version_cache(LATEST_VERSION, TWO_DAYS_IN_SECONDS);

    assert!(is_refresh_due(Some(&version_cache), Utc::now()));
}

#[test]
fn refresh_is_not_due_when_last_checked_within_a_day() {
    let version_cache = get_mock_version_cache(LATEST_VERSION, ONE_HOUR_IN_SECONDS);

    assert!(!is_refresh_due(Some(&version_cache), Utc::now()));
}

#[test]
fn refresh_is_due_when_there_is_no_cache() {
    assert!(is_refresh_due(None, Utc::now()));
}
