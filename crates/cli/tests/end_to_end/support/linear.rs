use credentials::credential::Credential;
use httpmock::{
    Method::POST,
    Mock,
    MockServer,
};
use serde_json::json;

use crate::support::sandbox::SwelogSandbox;

pub const LINEAR_USERNAME: &str = "rudra";

pub const LINEAR_ACCESS_TOKEN: &str = "lin_oauth_end_to_end";

const MCP_PATH: &str = "/mcp/readonly";

const INITIALIZE_REQUEST_ID: u64 = 0;

const LIST_ISSUES_REQUEST_ID: u64 = 1;

pub const LINEAR_SECTION_FOR_THE_DAY: &str = "## Linear

### In Progress

- [SWE-42](https://linear.app/example/issue/SWE-42) Ship end-to-end tests

### Done

- [SWE-41](https://linear.app/example/issue/SWE-41) Fix work file formatting";

pub const LINEAR_SECTION_FOR_ACTIVE_ISSUES: &str = "## Linear

### In Progress

- [SWE-42](https://linear.app/example/issue/SWE-42) Ship end-to-end tests

### Todo

- [SWE-40](https://linear.app/example/issue/SWE-40) Document the config flag";

pub struct LinearMocks<'server> {
    pub initialize: Mock<'server>,
    pub initialized: Mock<'server>,
    pub list_issues: Mock<'server>,
}

pub fn linear_mcp_url(server: &MockServer) -> String {
    format!("{}{MCP_PATH}", server.base_url())
}

/// Stores an OAuth token in the shape rmcp keeps, so the CLI treats Linear as
/// already authorized and never opens a browser.
pub fn store_linear_authorization(sandbox: &SwelogSandbox) {
    let stored_credentials = json!({
        "client_id": "swelog-end-to-end",
        "token_response": {
            "access_token": LINEAR_ACCESS_TOKEN,
            "token_type": "bearer",
            "expires_in": 3600
        },
        "granted_scopes": ["read"],
        "token_received_at": null,
        "issuer": null
    });

    sandbox.store_credential(Credential::Linear, &stored_credentials.to_string());
}

pub fn mock_linear_mcp(server: &MockServer) -> LinearMocks<'_> {
    LinearMocks {
        initialize: mock_initialize(server),
        initialized: mock_initialized_notification(server),
        list_issues: mock_list_issues(server),
    }
}

fn mock_initialize(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(POST)
            .path(MCP_PATH)
            .header("authorization", format!("Bearer {LINEAR_ACCESS_TOKEN}"))
            .json_body_includes(r#"{ "method": "initialize" }"#);

        then.status(200).header("content-type", "application/json").json_body(json!({
            "jsonrpc": "2.0",
            "id": INITIALIZE_REQUEST_ID,
            "result": {
                "protocolVersion": "2025-06-18",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "fake-linear", "version": "0.0.0" }
            }
        }));
    })
}

fn mock_initialized_notification(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(POST)
            .path(MCP_PATH)
            .json_body_includes(r#"{ "method": "notifications/initialized" }"#);

        then.status(202);
    })
}

fn mock_list_issues(server: &MockServer) -> Mock<'_> {
    let issue_page = json!({
        "issues": [
            {
                "id": "SWE-42",
                "title": "Ship  end-to-end tests",
                "url": "https://linear.app/example/issue/SWE-42",
                "status": "In Progress",
                "statusType": "started",
                "createdAt": "2026-07-01T10:00:00Z",
                "startedAt": "2026-07-04T09:00:00Z",
                "updatedAt": "2026-07-04T15:30:00Z"
            },
            {
                "id": "SWE-41",
                "title": "Fix work file formatting",
                "url": "https://linear.app/example/issue/SWE-41",
                "status": "Done",
                "statusType": "completed",
                "createdAt": "2026-06-30T10:00:00Z",
                "completedAt": "2026-07-04T12:00:00Z",
                "updatedAt": "2026-07-04T12:00:00Z"
            },
            {
                "id": "SWE-40",
                "title": "Document the config flag",
                "url": "https://linear.app/example/issue/SWE-40",
                "status": "Todo",
                "statusType": "unstarted",
                "createdAt": "2026-07-02T10:00:00Z",
                "updatedAt": "2026-07-02T10:00:00Z"
            }
        ],
        "hasNextPage": false
    });

    server.mock(|when, then| {
        when.method(POST)
            .path(MCP_PATH)
            .header("authorization", format!("Bearer {LINEAR_ACCESS_TOKEN}"))
            .json_body_includes(format!(
                r#"{{ "method": "tools/call", "params": {{ "name": "list_issues", "arguments": {{ "assignee": "{LINEAR_USERNAME}" }} }} }}"#
            ));

        then.status(200).header("content-type", "application/json").json_body(json!({
            "jsonrpc": "2.0",
            "id": LIST_ISSUES_REQUEST_ID,
            "result": {
                "content": [{ "type": "text", "text": issue_page.to_string() }],
                "structuredContent": issue_page
            }
        }));
    })
}
