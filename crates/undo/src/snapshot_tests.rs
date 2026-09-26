use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const WORK_FILE_CONTENT: &str = "# Today's Work\n\n## Log\n- Reviewed auth PR\n";

const INVALID_UNDO_SNAPSHOT_JSON: &str = "{ not json";

struct TestContext {
    temporary_directory: TempDir,
    undo_snapshot_file: PathBuf,
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let undo_snapshot_file = get_undo_snapshot_file_path(&temporary_directory.path().join("cache"));

    TestContext { temporary_directory, undo_snapshot_file }
}

fn get_mock_undo_snapshot() -> UndoSnapshot {
    UndoSnapshot {
        created_file: Some(PathBuf::from("/vault/swelog/Daily/05-23-2026.md")),
        work_file_content: String::from(WORK_FILE_CONTENT),
    }
}

#[test]
fn read_undo_snapshot_returns_what_write_undo_snapshot_stored() {
    let TestContext { temporary_directory, undo_snapshot_file } = get_test_context();

    let undo_snapshot = get_mock_undo_snapshot();

    write_undo_snapshot(&undo_snapshot_file, &undo_snapshot)
        .expect("undo snapshot should be written");

    let stored_undo_snapshot =
        read_undo_snapshot(&undo_snapshot_file).expect("undo snapshot should be read");

    assert_eq!(stored_undo_snapshot, undo_snapshot);

    drop(temporary_directory);
}

#[test]
fn write_undo_snapshot_creates_the_cache_directory() {
    let TestContext { temporary_directory, undo_snapshot_file } = get_test_context();

    write_undo_snapshot(&undo_snapshot_file, &get_mock_undo_snapshot())
        .expect("undo snapshot should be written");

    assert!(undo_snapshot_file.is_file());

    drop(temporary_directory);
}

#[test]
fn write_undo_snapshot_replaces_an_existing_snapshot() {
    let TestContext { temporary_directory, undo_snapshot_file } = get_test_context();

    write_undo_snapshot(&undo_snapshot_file, &get_mock_undo_snapshot())
        .expect("undo snapshot should be written");

    let reset_undo_snapshot =
        UndoSnapshot { created_file: None, work_file_content: String::from("newer notes") };

    write_undo_snapshot(&undo_snapshot_file, &reset_undo_snapshot)
        .expect("undo snapshot should be replaced");

    let stored_undo_snapshot =
        read_undo_snapshot(&undo_snapshot_file).expect("undo snapshot should be read");

    assert_eq!(stored_undo_snapshot, reset_undo_snapshot);

    drop(temporary_directory);
}

#[test]
fn read_undo_snapshot_fails_when_the_snapshot_is_missing() {
    let TestContext { temporary_directory, undo_snapshot_file } = get_test_context();

    let error = read_undo_snapshot(&undo_snapshot_file).expect_err("missing snapshot should fail");

    error.downcast_ref::<NoUndoSnapshot>().expect("error should be NoUndoSnapshot");

    drop(temporary_directory);
}

#[test]
fn read_undo_snapshot_fails_when_the_snapshot_is_not_valid_json() {
    let TestContext { temporary_directory, undo_snapshot_file } = get_test_context();

    let cache_directory =
        undo_snapshot_file.parent().expect("undo snapshot should have a parent directory");

    fs::create_dir_all(cache_directory).expect("cache directory should be created");

    fs::write(&undo_snapshot_file, INVALID_UNDO_SNAPSHOT_JSON)
        .expect("invalid snapshot should be written");

    let error = read_undo_snapshot(&undo_snapshot_file).expect_err("invalid snapshot should fail");

    assert!(error.to_string().contains("failed to parse the undo snapshot"));

    drop(temporary_directory);
}

#[test]
fn undo_snapshot_json_uses_camel_case_field_names() {
    let TestContext { temporary_directory, undo_snapshot_file } = get_test_context();

    write_undo_snapshot(&undo_snapshot_file, &get_mock_undo_snapshot())
        .expect("undo snapshot should be written");

    let undo_snapshot_contents =
        fs::read_to_string(&undo_snapshot_file).expect("undo snapshot should be readable");

    assert!(undo_snapshot_contents.contains("\"createdFile\""));

    assert!(undo_snapshot_contents.contains("\"workFileContent\""));

    drop(temporary_directory);
}

#[test]
fn remove_undo_snapshot_deletes_a_stored_snapshot() {
    let TestContext { temporary_directory, undo_snapshot_file } = get_test_context();

    write_undo_snapshot(&undo_snapshot_file, &get_mock_undo_snapshot())
        .expect("undo snapshot should be written");

    remove_undo_snapshot(&undo_snapshot_file).expect("undo snapshot should be removed");

    assert!(!undo_snapshot_file.exists());

    drop(temporary_directory);
}

#[test]
fn remove_undo_snapshot_succeeds_when_the_snapshot_is_already_gone() {
    let TestContext { temporary_directory, undo_snapshot_file } = get_test_context();

    remove_undo_snapshot(&undo_snapshot_file).expect("missing snapshot should be tolerated");

    drop(temporary_directory);
}
