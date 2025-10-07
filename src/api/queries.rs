pub const TEAMS_QUERY: &str = r#"
    query Teams($after: String) {
        teams(first: 50, after: $after) {
            nodes { id key name }
            pageInfo { hasNextPage endCursor }
        }
    }
"#;

pub const ISSUES_QUERY: &str = r#"
    query Issues($teamId: ID!, $after: String) {
        issues(first: 100, after: $after, orderBy: updatedAt, filter: { team: { id: { eq: $teamId } } }) {
            nodes { id identifier title url description team { key name } }
            pageInfo { hasNextPage endCursor }
        }
    }
"#;

pub const ISSUE_COMMENTS_QUERY: &str = r#"
    query IssueComments($issueId: String!, $after: String) {
        issue(id: $issueId) {
            id
            comments(first: 100, after: $after) {
                nodes { id body user { name } createdAt url }
                pageInfo { hasNextPage endCursor }
            }
        }
    }
"#;
