use std::io::{self, Write};

use crate::api::types::{Match, Team};

pub fn prompt(message: &str) -> io::Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn display_teams(teams: &[Team]) {
    println!("\nAvailable teams:");
    for (idx, team) in teams.iter().enumerate() {
        println!(
            "{}. [{}] {} (ID: {})",
            idx + 1,
            team.key,
            team.name,
            team.id
        );
    }
}

pub fn select_team(teams: &[Team], choice: &str) -> Option<String> {
    if let Ok(num) = choice.parse::<usize>() {
        if num > 0 && num <= teams.len() {
            return Some(teams[num - 1].id.clone());
        }
    } else {
        return teams
            .iter()
            .find(|t| t.id == choice || t.key.eq_ignore_ascii_case(choice))
            .map(|t| t.id.clone());
    }
    None
}

pub fn display_search_info(search_term: &str, max_requests: usize, descriptions: bool, comments: bool) {
    println!("\n🔍 Searching for: '{}'", search_term);
    println!("📊 Max requests allowed: {}", max_requests);
    println!(
        "📝 Searching in: {}{}",
        if descriptions { "descriptions " } else { "" },
        if comments { "comments " } else { "" }
    );
    println!();
}

pub fn display_results(matches: &[Match], search_term: &str, request_count: usize, max_requests: usize) {
    println!(
        "\n📈 Total API requests used: {}/{}\n",
        request_count, max_requests
    );

    if matches.is_empty() {
        println!("No issues found containing '{}'.", search_term);
        return;
    }

    println!(
        "Found {} issue(s) containing '{}':\n",
        matches.len(),
        search_term
    );

    for m in matches {
        println!("{}  {}  {}", m.team, m.id, m.title);
        println!("→ {}", m.url);

        let mut flags = Vec::new();
        if m.in_title {
            flags.push("title");
        }
        if m.in_desc {
            flags.push("description");
        }
        if !flags.is_empty() {
            println!("   ✓ matched in: {}", flags.join(", "));
        }

        if !m.comments_matched.is_empty() {
            println!("   ✓ {} comment(s) matched:", m.comments_matched.len());
            for c in &m.comments_matched {
                println!("     - {} @ {} → {}", c.commenter, c.created_at, c.url);
            }
        }
        println!();
    }
}

pub fn display_rate_limit_warning(max_requests: usize) {
    println!(
        "\n⚠️  Maximum requests reached ({} requests)",
        max_requests
    );
    println!("Results may be incomplete.\n");
}
