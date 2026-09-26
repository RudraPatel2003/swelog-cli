use std::path::PathBuf;

use tempfile::{
    TempDir,
    tempdir,
};

use super::*;
use crate::errors::SwelogFileNotFound;

const WORK_FILE_CONTENT: &str = "# Today's Work\n\n## Log\n- Reviewed auth PR\n";

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

    fn write_work_file(&self) {
        let swelog_paths = self.swelog_paths();

        fs::create_dir_all(&swelog_paths.swelog_directory)
            .expect("swelog directory should be created");

        fs::write(&swelog_paths.work_file, WORK_FILE_CONTENT).expect("work file should be written");
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
fn read_work_file_returns_the_work_file_content() {
    let test_context = get_test_context();

    test_context.write_work_file();

    let work_file_content =
        read_work_file(&test_context.swelog_paths()).expect("work file should be read");

    assert_eq!(work_file_content, WORK_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn read_work_file_fails_when_the_work_file_is_missing() {
    let test_context = get_test_context();

    let error =
        read_work_file(&test_context.swelog_paths()).expect_err("missing work file should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.work_file());

    drop(test_context.temporary_directory);
}
