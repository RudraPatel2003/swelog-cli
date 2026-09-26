use config::setup::default_files::{
    DEFAULT_WORK_FILE_CONTENT,
    DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS,
};

use super::*;
use crate::sections::format_section;

const DOCUMENTED_CONTEXT_FILE_CONTENT: &str = r"# Engineer Context

## Role and Team

Senior backend engineer on the Payments Platform team.

## Systems Owned

Checkout APIs, billing event pipeline, payout reconciliation jobs.

## Current Priorities

Improve billing reliability, reduce support escalations, and make deployments
safer.
";

const OBSIDIAN_NOTE_CONTENT: &str = r"---
tags: [payments, on-call]
---

# Today's Work

- [ ] Follow up on [[Checkout Retries]]
- [x] Review [[Billing Pipeline|the billing pipeline]] #review

![[architecture.png]]

> [!note] Incident
> Payout job paged twice.

~~Cancelled standup~~
";

#[test]
fn format_markdown_leaves_the_default_work_file_unchanged() {
    let formatted_markdown = format_markdown(DEFAULT_WORK_FILE_CONTENT);

    assert_eq!(formatted_markdown, DEFAULT_WORK_FILE_CONTENT);
}

#[test]
fn format_markdown_leaves_the_default_work_file_without_comments_unchanged() {
    let formatted_markdown = format_markdown(DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS);

    assert_eq!(formatted_markdown, DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS);
}

#[test]
fn format_markdown_leaves_the_documented_context_file_unchanged() {
    let formatted_markdown = format_markdown(DOCUMENTED_CONTEXT_FILE_CONTENT);

    assert_eq!(formatted_markdown, DOCUMENTED_CONTEXT_FILE_CONTENT);
}

#[test]
fn format_markdown_leaves_a_formatted_section_unchanged() {
    let section = format!("{}\n", format_section("Linear", "### Todo\n\n- Ship it"));

    let formatted_markdown = format_markdown(&section);

    assert_eq!(formatted_markdown, section);
}

#[test]
fn format_markdown_leaves_obsidian_syntax_unchanged() {
    let formatted_markdown = format_markdown(OBSIDIAN_NOTE_CONTENT);

    assert_eq!(formatted_markdown, OBSIDIAN_NOTE_CONTENT);
}

#[test]
fn format_markdown_normalizes_headings_bullets_and_blank_lines() {
    let markdown = r"# Today's Work
## Priorities
* Ship the *retry* fix



## Log
* Paired on billing
";

    let formatted_markdown = format_markdown(markdown);

    let expected_markdown = r"# Today's Work

## Priorities

- Ship the _retry_ fix

## Log

- Paired on billing
";

    assert_eq!(formatted_markdown, expected_markdown);
}

#[test]
fn format_markdown_leaves_code_blocks_unchanged() {
    let markdown = r"# Notes

```rust
fn  main(){}
```
";

    let formatted_markdown = format_markdown(markdown);

    assert_eq!(formatted_markdown, markdown);
}
