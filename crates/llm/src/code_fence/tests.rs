use super::*;

const DAILY_LOG_CONTENT: &str = "# Daily Log\n\n- Reviewed auth PR";

#[test]
fn strip_markdown_code_fence_removes_a_markdown_fence_around_the_response() {
    let response = format!("```md\n{DAILY_LOG_CONTENT}\n```");

    assert_eq!(strip_markdown_code_fence(&response), DAILY_LOG_CONTENT);
}

#[test]
fn strip_markdown_code_fence_removes_a_fence_with_a_markdown_language_tag() {
    let response = format!("```markdown\n{DAILY_LOG_CONTENT}\n```");

    assert_eq!(strip_markdown_code_fence(&response), DAILY_LOG_CONTENT);
}

#[test]
fn strip_markdown_code_fence_removes_a_fence_without_a_language_tag() {
    let response = format!("```\n{DAILY_LOG_CONTENT}\n```");

    assert_eq!(strip_markdown_code_fence(&response), DAILY_LOG_CONTENT);
}

#[test]
fn strip_markdown_code_fence_removes_a_fence_surrounded_by_whitespace() {
    let response = format!("\n  ```md  \n{DAILY_LOG_CONTENT}\n```\n\n");

    assert_eq!(strip_markdown_code_fence(&response), DAILY_LOG_CONTENT);
}

#[test]
fn strip_markdown_code_fence_keeps_a_response_without_a_fence() {
    assert_eq!(strip_markdown_code_fence(DAILY_LOG_CONTENT), DAILY_LOG_CONTENT);
}

#[test]
fn strip_markdown_code_fence_keeps_a_response_with_only_an_opening_fence() {
    let response = format!("```md\n{DAILY_LOG_CONTENT}");

    assert_eq!(strip_markdown_code_fence(&response), response);
}

#[test]
fn strip_markdown_code_fence_keeps_a_response_with_only_a_closing_fence() {
    let response = format!("{DAILY_LOG_CONTENT}\n```");

    assert_eq!(strip_markdown_code_fence(&response), response);
}

#[test]
fn strip_markdown_code_fence_keeps_a_response_whose_fence_is_not_on_its_own_line() {
    let response = format!("```md\n{DAILY_LOG_CONTENT}```");

    assert_eq!(strip_markdown_code_fence(&response), response);
}

#[test]
fn strip_markdown_code_fence_keeps_a_response_that_only_starts_with_a_fenced_block() {
    let response = format!("```rust\nlet log = 1;\n```\n\n{DAILY_LOG_CONTENT}");

    assert_eq!(strip_markdown_code_fence(&response), response);
}

#[test]
fn strip_markdown_code_fence_keeps_an_empty_response() {
    assert_eq!(strip_markdown_code_fence(""), "");
}
