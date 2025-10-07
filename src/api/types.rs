use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct TeamsData {
    pub teams: Connection<Team>,
}

#[derive(Debug, Deserialize)]
pub struct IssuesData {
    pub issues: Connection<Issue>,
}

#[derive(Debug, Deserialize)]
pub struct IssueData {
    pub issue: Option<IssueWithComments>,
}

#[derive(Debug, Deserialize)]
pub struct Connection<T> {
    pub nodes: Vec<T>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Debug, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Team {
    pub id: String,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub team: Option<TeamInfo>,
}

#[derive(Debug, Deserialize)]
pub struct IssueWithComments {
    pub id: String,
    pub comments: Connection<Comment>,
}

#[derive(Debug, Deserialize)]
pub struct TeamInfo {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub user: Option<User>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
}

#[derive(Debug)]
pub struct Match {
    pub team: String,
    pub id: String,
    pub title: String,
    pub url: String,
    pub in_title: bool,
    pub in_desc: bool,
    pub comments_matched: Vec<CommentHit>,
}

#[derive(Debug)]
pub struct CommentHit {
    pub commenter: String,
    pub created_at: String,
    pub url: String,
}
