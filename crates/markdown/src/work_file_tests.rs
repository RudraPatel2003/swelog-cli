use std::{
    fs,
    path::PathBuf,
};

use config::{
    errors::SwelogFileNotFound,
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
};
use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const LINEAR_SECTION_CONTENT: &str = "- [ENG-123](https://linear.app/issue/ENG-123) Ship it";

struct TestContext {
    temporary_directory: TempDir,
    config: SwelogConfig,
}

impl TestContext {
    fn swelog_paths(&self) -> SwelogPaths {
        SwelogPaths::new(&self.config)
    }

    fn swelog_directory(&self) -> PathBuf {
        self.swelog_paths().swelog_directory
    }

    fn work_file(&self) -> PathBuf {
        self.swelog_paths().work_file
    }

    fn write_work_file(&self, content: &str) {
        fs::create_dir_all(self.swelog_directory()).expect("swelog directory should be created");

        fs::write(self.work_file(), content).expect("work file should be written");
    }
}

fn make_read_only(file: &Path) {
    let mut permissions = fs::metadata(file).expect("file should have metadata").permissions();

    permissions.set_readonly(true);

    fs::set_permissions(file, permissions).expect("file should be made read-only");
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
fn upsert_work_file_section_writes_the_section_into_the_work_file() {
    let test_context = get_test_context();

    let work_file_content = r"# Today's Work

## Focus
- Ship it

## Log
- Existing log
";

    test_context.write_work_file(work_file_content);

    upsert_work_file_section_from_config(&test_context.config, "Linear", LINEAR_SECTION_CONTENT)
        .expect("section should be upserted");

    let updated_work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    let expected_work_file_content = format!(
        r"# Today's Work

## Focus
- Ship it

## Linear

{LINEAR_SECTION_CONTENT}

## Log
- Existing log
"
    );

    assert_eq!(updated_work_file_content, expected_work_file_content);

    drop(test_context.temporary_directory);
}

#[test]
fn remove_work_file_section_writes_the_updated_work_file() {
    let test_context = get_test_context();

    let work_file_content = r"# Today's Work

## Linear
- Issue

## Log
- Existing log
";

    test_context.write_work_file(work_file_content);

    remove_work_file_section_from_config(&test_context.config, "Linear")
        .expect("section should be removed");

    let updated_work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    let expected_work_file_content = r"# Today's Work

## Log
- Existing log
";

    assert_eq!(updated_work_file_content, expected_work_file_content);

    drop(test_context.temporary_directory);
}

#[test]
fn format_work_file_formats_the_work_file_in_place() {
    let test_context = get_test_context();

    let work_file_content = r"# Today's Work
## Log
* Paired on billing
";

    test_context.write_work_file(work_file_content);

    format_work_file(&test_context.work_file()).expect("work file should be formatted");

    let formatted_work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    let expected_work_file_content = r"# Today's Work

## Log

- Paired on billing
";

    assert_eq!(formatted_work_file_content, expected_work_file_content);

    drop(test_context.temporary_directory);
}

#[test]
fn format_work_file_leaves_a_formatted_work_file_untouched() {
    let test_context = get_test_context();

    let work_file_content = r"# Today's Work

## Log

- Paired on billing
";

    test_context.write_work_file(work_file_content);

    make_read_only(&test_context.work_file());

    format_work_file(&test_context.work_file()).expect("formatted work file should not be written");

    drop(test_context.temporary_directory);
}

#[test]
fn format_work_file_fails_when_work_file_is_missing() {
    let test_context = get_test_context();

    let error =
        format_work_file(&test_context.work_file()).expect_err("missing work file should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.work_file());

    drop(test_context.temporary_directory);
}

#[test]
fn upsert_work_file_section_fails_when_work_file_is_missing() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.swelog_directory())
        .expect("swelog directory should be created");

    let error = upsert_work_file_section_from_config(
        &test_context.config,
        "Linear",
        LINEAR_SECTION_CONTENT,
    )
    .expect_err("missing work file should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.work_file());

    drop(test_context.temporary_directory);
}

#[test]
fn upsert_section_inserts_before_log() {
    let markdown = r"# Today's Work

## Focus
- Ship it

## Log
- Existing log
";

    let new_section_content =
        "### In Progress\n- [ENG-123](https://linear.app/issue/ENG-123) Ship it";

    let updated_markdown = upsert_section(markdown, "Linear", new_section_content);

    let expected_markdown = r"# Today's Work

## Focus
- Ship it

## Linear

### In Progress
- [ENG-123](https://linear.app/issue/ENG-123) Ship it

## Log
- Existing log
";

    assert_eq!(updated_markdown, expected_markdown);
}

#[test]
fn upsert_section_appends_when_work_file_has_no_log_section() {
    let markdown = "# Today's Work\n\n## Focus\n- Ship it\n";

    let updated_markdown = upsert_section(markdown, "Linear", "- New");

    let expected_markdown = r"# Today's Work

## Focus
- Ship it

## Linear

- New
";

    assert_eq!(updated_markdown, expected_markdown);
}

#[test]
fn upsert_section_replaces_existing_section_content() {
    let markdown = r"# Today's Work

## Linear
- Old

## Log
";

    let updated_markdown = upsert_section(markdown, "Linear", "- New");

    let expected_markdown = r"# Today's Work

## Linear

- New

## Log
";

    assert_eq!(updated_markdown, expected_markdown);
}

#[test]
fn upsert_section_leaves_other_sections_untouched() {
    let markdown = r"# Today's Work

## Linear
- Old

## GitHub
- Merged a PR

## Log
- Existing log
";

    let updated_markdown = upsert_section(markdown, "Linear", "- New");

    let expected_markdown = r"# Today's Work

## Linear

- New

## GitHub
- Merged a PR

## Log
- Existing log
";

    assert_eq!(updated_markdown, expected_markdown);
}

#[test]
fn upsert_section_ignores_heading_in_code_block() {
    let markdown = r"# Today's Work

```markdown
## Linear
```

## Log
";

    let updated_markdown = upsert_section(markdown, "Linear", "- New");

    let expected_markdown = r"# Today's Work

```markdown
## Linear
```

## Linear

- New

## Log
";

    assert_eq!(updated_markdown, expected_markdown);
}

#[test]
fn remove_section_removes_only_the_named_section() {
    let markdown = r"# Today's Work

## Linear
- Issue

## Log
- Existing log
";

    let updated_markdown = remove_section(markdown, "Linear");

    let expected_markdown = r"# Today's Work

## Log
- Existing log
";

    assert_eq!(updated_markdown, expected_markdown);
}

#[test]
fn remove_section_leaves_markdown_unchanged_when_section_is_missing() {
    let markdown = r"# Today's Work

## Focus
- Ship it

## Log
";

    let updated_markdown = remove_section(markdown, "Linear");

    assert_eq!(updated_markdown, markdown);
}
