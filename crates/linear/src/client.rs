pub mod structs;

use chrono::{
    Local,
    NaiveDate,
};
use credentials::store::CredentialStore;
use miette::Result;
use reqwest::Client;
use rmcp::{
    RoleClient,
    ServiceExt,
    model::{
        CallToolRequestParams,
        JsonObject,
    },
    service::RunningService,
    transport::{
        AuthClient,
        StreamableHttpClientTransport,
        streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use serde_json::Value;
use url::Url;

use crate::{
    activity::{
        get_updated_after_filter,
        is_issue_active_or_finished_today,
        was_issue_worked_on,
    },
    client::structs::{
        LinearIssue,
        LinearIssuePage,
    },
    errors::LinearMcpRequestFailed,
    oauth::{
        clear_linear_authorization,
        get_authorization_manager,
    },
    response::parse_issue_page,
};

pub const DEFAULT_LINEAR_MCP_URL: &str = "https://mcp.linear.app/mcp/readonly";

const PAGE_SIZE: u64 = 50;

pub struct LinearClient {
    mcp_url: Url,
    credential_store: CredentialStore,
}

impl LinearClient {
    #[must_use]
    pub const fn new(mcp_url: Url, credential_store: CredentialStore) -> Self {
        Self { mcp_url, credential_store }
    }

    pub async fn get_current_active_assigned_issues(
        &self,
        username: &str,
        today: &NaiveDate,
    ) -> Result<Vec<LinearIssue>> {
        let client = self.connect_to_linear_mcp().await?;

        let mut issues = fetch_all_issues(&client, username, None).await?;

        issues.retain(|issue| is_issue_active_or_finished_today(issue, *today, &Local));

        disconnect_from_linear_mcp(client).await?;

        Ok(issues)
    }

    pub async fn get_assigned_issues_on_date(
        &self,
        username: &str,
        date: &NaiveDate,
    ) -> Result<Vec<LinearIssue>> {
        let updated_after = get_updated_after_filter(*date)?;

        let client = self.connect_to_linear_mcp().await?;

        let mut issues = fetch_all_issues(&client, username, Some(&updated_after)).await?;

        issues.retain(|issue| was_issue_worked_on(issue, *date, &Local));

        disconnect_from_linear_mcp(client).await?;

        Ok(issues)
    }

    async fn connect_to_linear_mcp(&self) -> Result<RunningService<RoleClient, ()>> {
        let mut reauthorization_attempted = false;

        loop {
            let authorization_manager =
                get_authorization_manager(self.mcp_url.as_str(), &self.credential_store).await?;

            let authorized_client = AuthClient::new(Client::new(), authorization_manager);

            let transport = StreamableHttpClientTransport::with_client(
                authorized_client,
                StreamableHttpClientTransportConfig::with_uri(self.mcp_url.as_str()),
            );

            match ().serve(transport).await {
                Ok(client) => return Ok(client),

                Err(error) if error.is_authorization_required() && !reauthorization_attempted => {
                    clear_linear_authorization(&self.credential_store)?;

                    reauthorization_attempted = true;
                }

                Err(error) => {
                    let linear_mcp_request_failed_error =
                        LinearMcpRequestFailed { message: error.to_string() };

                    return Err(linear_mcp_request_failed_error.into());
                }
            }
        }
    }
}

async fn disconnect_from_linear_mcp(client: RunningService<RoleClient, ()>) -> Result<()> {
    client.cancel().await.map_err(|error| LinearMcpRequestFailed { message: error.to_string() })?;

    Ok(())
}

async fn fetch_all_issues(
    client: &RunningService<RoleClient, ()>,
    username: &str,
    updated_after: Option<&str>,
) -> Result<Vec<LinearIssue>> {
    let mut issues = Vec::new();

    let mut cursor: Option<String> = None;

    loop {
        let mut page = fetch_issue_page(client, username, cursor.as_deref(), updated_after).await?;

        issues.append(&mut page.issues);

        let Some(next_cursor) = page.take_next_cursor() else {
            return Ok(issues);
        };

        if cursor.as_deref() == Some(next_cursor.as_str()) {
            let linear_mcp_request_failed_error = LinearMcpRequestFailed {
                message: "Linear MCP returned a repeated pagination cursor".to_string(),
            };

            return Err(linear_mcp_request_failed_error.into());
        }

        cursor = Some(next_cursor);
    }
}

async fn fetch_issue_page(
    client: &RunningService<RoleClient, ()>,
    username: &str,
    cursor: Option<&str>,
    updated_after: Option<&str>,
) -> Result<LinearIssuePage> {
    let arguments = list_issues_arguments(username, cursor, updated_after);

    let result = client
        .call_tool(CallToolRequestParams::new("list_issues").with_arguments(arguments))
        .await
        .map_err(|error| LinearMcpRequestFailed { message: error.to_string() })?;

    parse_issue_page(result)
}

fn list_issues_arguments(
    username: &str,
    cursor: Option<&str>,
    updated_after: Option<&str>,
) -> JsonObject {
    let mut arguments = JsonObject::new();

    let issue_fields_argument = get_issue_fields_argument();

    arguments.insert("assignee".to_string(), Value::String(username.to_string()));

    arguments.insert("includeArchived".to_string(), Value::Bool(false));

    arguments.insert("limit".to_string(), Value::Number(PAGE_SIZE.into()));

    arguments.insert("fields".to_string(), issue_fields_argument);

    if let Some(cursor) = cursor {
        arguments.insert("cursor".to_string(), Value::String(cursor.to_string()));
    }

    if let Some(updated_after) = updated_after {
        arguments.insert("updatedAt".to_string(), Value::String(updated_after.to_string()));
    }

    arguments
}

/// Only fetch what is needed
const ISSUE_FIELDS: [&str; 10] = [
    "id",
    "title",
    "url",
    "status",
    "statusType",
    "createdAt",
    "startedAt",
    "completedAt",
    "canceledAt",
    "updatedAt",
];

fn get_issue_fields_argument() -> Value {
    Value::Array(
        ISSUE_FIELDS.iter().map(|field| Value::String((*field).to_string())).collect::<Vec<_>>(),
    )
}
