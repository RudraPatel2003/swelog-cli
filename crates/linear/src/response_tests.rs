use rmcp::model::{
    CallToolResult,
    ContentBlock,
};

use super::*;
use crate::client::structs::{
    LinearIssue,
    LinearIssueTimestamps,
    LinearStatusType,
};

fn get_mock_structured_result(value: Value) -> CallToolResult {
    CallToolResult::structured(value)
}

#[test]
fn parse_issue_page_reads_linear_mcp_issue_shape() {
    let result = get_mock_structured_result(serde_json::json!({
        "issues": [{
            "id": "ISWF-3270",
            "title": "Remove organization:incidents flag",
            "url": "https://linear.app/getsentry/issue/ISWF-3270/remove-organizationincidents-flag",
            "status": "In Review",
            "statusType": "started",
            "startedAt": "2026-08-11T20:25:10.197Z",
            "completedAt": null,
            "updatedAt": "2026-08-13T18:22:28.096Z"
        }]
    }));

    let page = parse_issue_page(result).expect("Linear MCP issue page should parse");

    let issue = page.issues.first().expect("page should contain one issue");

    let expected_issue = LinearIssue {
        identifier: "ISWF-3270".to_string(),
        title: "Remove organization:incidents flag".to_string(),
        url: "https://linear.app/getsentry/issue/ISWF-3270/remove-organizationincidents-flag"
            .to_string(),
        status_name: "In Review".to_string(),
        status_type: LinearStatusType::Started,
        timestamps: LinearIssueTimestamps {
            started_at: Some("2026-08-11T20:25:10.197Z".parse().expect("timestamp should parse")),
            updated_at: Some("2026-08-13T18:22:28.096Z".parse().expect("timestamp should parse")),
            ..LinearIssueTimestamps::default()
        },
    };

    assert_eq!(*issue, expected_issue);
}

#[test]
fn parse_issue_page_reads_unrecognized_status_type_as_other() {
    let result = get_mock_structured_result(serde_json::json!({
        "issues": [{
            "id": "ENG-1",
            "title": "Investigate the outage",
            "url": "https://linear.app/acme/issue/ENG-1",
            "status": "Triage",
            "statusType": "triage"
        }]
    }));

    let page = parse_issue_page(result).expect("unknown status types should parse");

    let issue = page.issues.first().expect("page should contain one issue");

    assert_eq!(issue.status_type, LinearStatusType::Other);
}

#[test]
fn take_next_cursor_returns_cursor_when_more_pages_remain() {
    let result = get_mock_structured_result(serde_json::json!({
        "issues": [],
        "hasNextPage": true,
        "nextCursor": "next-page"
    }));

    let mut page = parse_issue_page(result).expect("issue page should parse");

    assert_eq!(page.take_next_cursor().as_deref(), Some("next-page"));
}

#[test]
fn take_next_cursor_reads_the_cursor_the_linear_mcp_server_returns() {
    let result = get_mock_structured_result(serde_json::json!({
        "issues": [],
        "hasNextPage": true,
        "cursor": "next-page"
    }));

    let mut page = parse_issue_page(result).expect("issue page should parse");

    assert_eq!(page.take_next_cursor().as_deref(), Some("next-page"));
}

#[test]
fn take_next_cursor_stops_on_the_last_page() {
    let result = get_mock_structured_result(serde_json::json!({
        "issues": [],
        "hasNextPage": false,
        "nextCursor": "stale-cursor"
    }));

    let mut page = parse_issue_page(result).expect("issue page should parse");

    assert_eq!(page.take_next_cursor(), None);
}

#[test]
fn parse_issue_page_reads_the_empty_text_response() {
    let result = CallToolResult::success(vec![ContentBlock::text("No issues found")]);

    let page = parse_issue_page(result).expect("empty text response should parse");

    assert_eq!(page.issues, []);
}

#[test]
fn parse_issue_page_reports_tool_errors() {
    let result = CallToolResult::error(vec![ContentBlock::text("rate limited")]);

    let error = parse_issue_page(result).expect_err("tool errors should fail");

    assert!(error.to_string().contains("rate limited"));
}
