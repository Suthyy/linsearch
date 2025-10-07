pub mod queries;
pub mod types;

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};

use types::{
    Comment, GraphQLRequest, GraphQLResponse, Issue, IssueData, IssuesData, Team, TeamsData,
};

const LINEAR_API_URL: &str = "https://api.linear.app/graphql";
const MAX_REQUESTS: usize = 1500;

static REQUEST_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct LinearClient {
    client: Client,
    api_key: String,
}

impl LinearClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub fn request_count(&self) -> usize {
        REQUEST_COUNT.load(Ordering::SeqCst)
    }

    pub fn max_requests(&self) -> usize {
        MAX_REQUESTS
    }

    async fn execute_query<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<Option<T>> {
        let count = REQUEST_COUNT.fetch_add(1, Ordering::SeqCst) + 1;

        if count > MAX_REQUESTS {
            anyhow::bail!("Rate limit reached: {} requests", MAX_REQUESTS);
        }

        let request = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = self
            .client
            .post(LINEAR_API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", &self.api_key)
            .json(&request)
            .send()
            .await?;

        let gql_response: GraphQLResponse<T> = response.json().await?;

        if let Some(errors) = gql_response.errors {
            eprintln!("⚠ GraphQL errors: {:?}", errors);
            return Ok(None);
        }

        Ok(gql_response.data)
    }

    pub async fn fetch_teams(&self) -> Result<Vec<Team>> {
        let mut teams = Vec::new();
        let mut after: Option<String> = None;

        loop {
            let variables = json!({ "after": after });
            let data: Option<TeamsData> = self
                .execute_query(queries::TEAMS_QUERY, variables)
                .await?;

            if let Some(data) = data {
                teams.extend(data.teams.nodes);
                if !data.teams.page_info.has_next_page {
                    break;
                }
                after = data.teams.page_info.end_cursor;
            } else {
                break;
            }
        }

        Ok(teams)
    }

    pub async fn fetch_issues(&self, team_id: &str) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        let mut after: Option<String> = None;

        loop {
            if REQUEST_COUNT.load(Ordering::SeqCst) >= MAX_REQUESTS {
                break;
            }

            let variables = json!({ "teamId": team_id, "after": after });
            let data: Option<IssuesData> = self
                .execute_query(queries::ISSUES_QUERY, variables)
                .await?;

            if let Some(data) = data {
                issues.extend(data.issues.nodes);
                if !data.issues.page_info.has_next_page {
                    break;
                }
                after = data.issues.page_info.end_cursor;
            } else {
                break;
            }
        }

        Ok(issues)
    }

    pub async fn fetch_comments(&self, issue_id: &str) -> Result<Vec<Comment>> {
        let mut comments = Vec::new();
        let mut after: Option<String> = None;

        loop {
            if REQUEST_COUNT.load(Ordering::SeqCst) >= MAX_REQUESTS {
                break;
            }

            let variables = json!({ "issueId": issue_id, "after": after });
            let data: Option<IssueData> = self
                .execute_query(queries::ISSUE_COMMENTS_QUERY, variables)
                .await?;

            if let Some(data) = data {
                if let Some(issue) = data.issue {
                    comments.extend(issue.comments.nodes);
                    if !issue.comments.page_info.has_next_page {
                        break;
                    }
                    after = issue.comments.page_info.end_cursor;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(comments)
    }
}
