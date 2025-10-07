use anyhow::Result;

use crate::api::types::{CommentHit, Issue, Match};
use crate::api::LinearClient;

pub struct SearchOptions {
    pub search_term: String,
    pub search_descriptions: bool,
    pub search_comments: bool,
}

impl SearchOptions {
    pub fn new(search_term: String, search_descriptions: bool, search_comments: bool) -> Self {
        Self {
            search_term,
            search_descriptions,
            search_comments,
        }
    }
}

pub async fn search_issues(
    client: &LinearClient,
    issues: Vec<Issue>,
    options: &SearchOptions,
) -> Result<Vec<Match>> {
    let mut matches = Vec::new();
    let search_term_lower = options.search_term.to_lowercase();

    for issue in issues {
        if client.request_count() >= client.max_requests() {
            break;
        }

        let in_title = issue.title.to_lowercase().contains(&search_term_lower);
        let in_desc = options.search_descriptions
            && issue
                .description
                .as_ref()
                .map(|d| d.to_lowercase().contains(&search_term_lower))
                .unwrap_or(false);

        let mut comment_hits = Vec::new();

        if options.search_comments && !in_title && !in_desc {
            if let Ok(comments) = client.fetch_comments(&issue.id).await {
                for comment in comments {
                    if comment.body.to_lowercase().contains(&search_term_lower) {
                        comment_hits.push(CommentHit {
                            commenter: comment
                                .user
                                .map(|u| u.name)
                                .unwrap_or_else(|| "Unknown".to_string()),
                            created_at: comment.created_at,
                            url: comment.url,
                        });
                    }
                }
            }
        }

        if in_title || in_desc || !comment_hits.is_empty() {
            let team_str = issue
                .team
                .as_ref()
                .map(|t| format!("[{}] {}", t.key, t.name))
                .unwrap_or_else(|| "Unknown".to_string());

            matches.push(Match {
                team: team_str,
                id: issue.identifier,
                title: issue.title,
                url: issue.url,
                in_title,
                in_desc,
                comments_matched: comment_hits,
            });
        }
    }

    Ok(matches)
}
