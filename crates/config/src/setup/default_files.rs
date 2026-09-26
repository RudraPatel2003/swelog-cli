pub const DEFAULT_WORK_FILE_CONTENT: &str = "# Today's Work

## Priorities

<!-- What you plan to focus on today. -->

## Log

<!-- Quick capture. Use short bullets; include systems, outcomes, reviews, debugging, meetings, or support work when useful. -->
";

pub const DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS: &str = "# Today's Work

## Priorities

## Log
";

#[must_use]
pub fn is_default_work_file_content(work_file_content: &str) -> bool {
    work_file_content == DEFAULT_WORK_FILE_CONTENT
        || work_file_content == DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS
}

#[cfg(test)]
#[path = "default_files_tests.rs"]
mod tests;
