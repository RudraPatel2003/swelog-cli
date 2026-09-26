use std::path::PathBuf;

use config::setup::swelog_paths::SwelogPaths;
use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const WORK_FILE_CONTENT: &str = "# Today's Work\n\n## Log\n- Reviewed auth PR\n";

const RESET_WORK_FILE_CONTENT: &str = "# Today's Work\n\n## Log\n";

const DAILY_LOG_CONTENT: &str = "# Daily Log - 05-23-2026\n";

const DAILY_LOG_FILE_NAME: &str = "05-23-2026.md";

struct TestContext {
    temporary_directory: TempDir,
    config: SwelogConfig,
}

impl TestContext {
    fn swelog_paths(&self) -> SwelogPaths {
        SwelogPaths::new(&self.config)
    }

    fn work_file(&self) -> PathBuf {
        self.swelog_paths().work_file
    }

    fn daily_log_file(&self) -> PathBuf {
        self.swelog_paths().daily_log_directory.join(DAILY_LOG_FILE_NAME)
    }

    fn write_swelog_files(&self) {
        let swelog_paths = self.swelog_paths();

        fs::create_dir_all(&swelog_paths.daily_log_directory)
            .expect("daily log directory should be created");

        fs::write(&swelog_paths.work_file, RESET_WORK_FILE_CONTENT)
            .expect("work file should be written");

        fs::write(self.daily_log_file(), DAILY_LOG_CONTENT)
            .expect("daily log file should be written");
    }
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let config = SwelogConfig {
        obsidian_vault_path: temporary_directory.path().to_path_buf(),
        ..SwelogConfig::get_default_config()
    };

    TestContext { temporary_directory, config }
}

#[test]
fn restore_undo_snapshot_writes_the_work_file_content_back() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let undo_snapshot = UndoSnapshot {
        created_file: Some(test_context.daily_log_file()),
        work_file_content: String::from(WORK_FILE_CONTENT),
    };

    restore_undo_snapshot(&test_context.config, &undo_snapshot)
        .expect("undo snapshot should be restored");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_content, WORK_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn restore_undo_snapshot_deletes_the_created_file() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let undo_snapshot = UndoSnapshot {
        created_file: Some(test_context.daily_log_file()),
        work_file_content: String::from(WORK_FILE_CONTENT),
    };

    restore_undo_snapshot(&test_context.config, &undo_snapshot)
        .expect("undo snapshot should be restored");

    assert!(!test_context.daily_log_file().exists());

    drop(test_context.temporary_directory);
}

#[test]
fn restore_undo_snapshot_restores_the_work_file_when_the_created_file_is_already_deleted() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::remove_file(test_context.daily_log_file()).expect("daily log file should be deleted");

    let undo_snapshot = UndoSnapshot {
        created_file: Some(test_context.daily_log_file()),
        work_file_content: String::from(WORK_FILE_CONTENT),
    };

    restore_undo_snapshot(&test_context.config, &undo_snapshot)
        .expect("undo snapshot should be restored");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_content, WORK_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn restore_undo_snapshot_deletes_nothing_when_no_file_was_created() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    let undo_snapshot =
        UndoSnapshot { created_file: None, work_file_content: String::from(WORK_FILE_CONTENT) };

    restore_undo_snapshot(&test_context.config, &undo_snapshot)
        .expect("undo snapshot should be restored");

    assert!(test_context.daily_log_file().is_file());

    drop(test_context.temporary_directory);
}
