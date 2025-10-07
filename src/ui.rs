use colored::Colorize;
use inquire::{Confirm, Select};
use std::fs::File;
use std::io::Write;

use crate::api::types::{Match, Team};

pub fn select_team_interactive(teams: &[Team]) -> anyhow::Result<String> {
    let options: Vec<String> = teams
        .iter()
        .map(|t| format!("[{}] {}", t.key.bright_cyan(), t.name))
        .collect();

    let selection = Select::new("👥 Select a team:", options.clone())
        .with_page_size(10)
        .prompt()?;

    // Find the selected team by matching the formatted string
    let selected_index = options.iter().position(|o| o == &selection).unwrap();
    Ok(teams[selected_index].id.clone())
}

pub fn confirm_search_descriptions() -> anyhow::Result<bool> {
    Confirm::new("📝 Search in descriptions?")
        .with_default(true)
        .prompt()
        .map_err(|e| anyhow::anyhow!("Prompt error: {}", e))
}

pub fn confirm_search_comments() -> anyhow::Result<bool> {
    Confirm::new("💬 Search in comments?")
        .with_default(true)
        .prompt()
        .map_err(|e| anyhow::anyhow!("Prompt error: {}", e))
}

pub fn display_search_info(
    search_term: &str,
    max_requests: usize,
    descriptions: bool,
    comments: bool,
) {
    println!();
    println!(
        "{} {}",
        "🔍 Searching for:".bright_blue().bold(),
        search_term.bright_yellow()
    );
    println!(
        "{} {}",
        "⚡ Max requests:".bright_blue(),
        max_requests.to_string().bright_white()
    );

    let mut search_in = Vec::new();
    if descriptions {
        search_in.push("descriptions".bright_green());
    }
    if comments {
        search_in.push("comments".bright_green());
    }

    println!(
        "{} {}",
        "🎯 Searching in:".bright_blue(),
        search_in
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();
}

pub fn display_results(
    matches: &[Match],
    search_term: &str,
    request_count: usize,
    max_requests: usize,
) {
    println!();
    println!(
        "{} {}/{}",
        "📊 Total API requests used:".bright_blue(),
        request_count.to_string().bright_white(),
        max_requests.to_string().bright_white()
    );
    println!();

    if matches.is_empty() {
        println!(
            "{} {}",
            "❌ No issues found containing".yellow(),
            format!("'{}'", search_term).bright_yellow()
        );
        return;
    }

    println!(
        "{} {} {}",
        "✨ Found".green().bold(),
        matches.len().to_string().bright_green().bold(),
        format!("issue(s) containing '{}':", search_term)
            .green()
            .bold()
    );
    println!();

    for m in matches {
        // Issue header - show team and title, hide ID
        println!(
            "{}  {}",
            m.team.bright_cyan(),
            m.title.bright_white().bold()
        );
        println!("🔗 {}", m.url.bright_blue().underline());

        // Match locations
        let mut flags = Vec::new();
        if m.in_title {
            flags.push("title".green());
        }
        if m.in_desc {
            flags.push("description".green());
        }
        if !flags.is_empty() {
            println!(
                "   {} {}",
                "✅".green(),
                format!(
                    "matched in: {}",
                    flags
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .bright_white()
            );
        }

        // Comment matches
        if !m.comments_matched.is_empty() {
            println!(
                "   {} {} {}",
                "💬".bright_white(),
                m.comments_matched.len().to_string().bright_yellow(),
                "comment(s) matched:".bright_white()
            );
            for c in &m.comments_matched {
                println!(
                    "     {} {} {} {} {}",
                    "•".bright_black(),
                    c.commenter.bright_cyan(),
                    "•".bright_black(),
                    c.created_at.bright_black(),
                    format!("🔗 {}", c.url).bright_blue()
                );
            }
        }
        println!();
    }
}

pub fn display_rate_limit_warning(max_requests: usize) {
    println!();
    println!(
        "{} {}",
        "⚠️  Maximum requests reached:".yellow().bold(),
        format!("{} requests", max_requests).bright_yellow()
    );
    println!("{}", "Results may be incomplete.".yellow());
    println!();
}

pub fn display_fetching_teams() {
    println!(
        "{}",
        "⏳ Fetching teams from Linear...".bright_blue().bold()
    );
}

pub fn display_fetching_issues() {
    println!("{}", "🔍 Searching issues...".bright_blue().bold());
}

pub fn save_results_to_file(
    file_path: &str,
    matches: &[Match],
    search_term: &str,
    request_count: usize,
    max_requests: usize,
) -> anyhow::Result<()> {
    let mut file = File::create(file_path)?;

    // Write header
    writeln!(file, "# LinSearch Results\n")?;
    writeln!(file, "**Search term:** `{}`", search_term)?;
    writeln!(file, "**Total results:** {}", matches.len())?;
    writeln!(
        file,
        "**API requests used:** {}/{}\n",
        request_count, max_requests
    )?;
    writeln!(file, "---\n")?;

    if matches.is_empty() {
        writeln!(file, "No issues found containing '{}'.", search_term)?;
        return Ok(());
    }

    // Write each match
    for (idx, m) in matches.iter().enumerate() {
        writeln!(file, "## {}. {}", idx + 1, m.title)?;
        writeln!(file)?;
        writeln!(file, "**Team:** {}", m.team)?;
        writeln!(file, "**URL:** {}", m.url)?;
        writeln!(file)?;

        // Match locations
        let mut flags = Vec::new();
        if m.in_title {
            flags.push("title");
        }
        if m.in_desc {
            flags.push("description");
        }
        if !flags.is_empty() {
            writeln!(file, "**Matched in:** {}", flags.join(", "))?;
        }

        // Comment matches
        if !m.comments_matched.is_empty() {
            writeln!(file)?;
            writeln!(
                file,
                "**💬 {} comment(s) matched:**",
                m.comments_matched.len()
            )?;
            writeln!(file)?;
            for c in &m.comments_matched {
                writeln!(file, "- **{}** ({})", c.commenter, c.created_at)?;
                writeln!(file, "  - {}", c.url)?;
            }
        }

        writeln!(file)?;
        writeln!(file, "---\n")?;
    }

    Ok(())
}

pub fn display_file_saved(file_path: &str, count: usize) {
    println!();
    println!(
        "{} {} {}",
        "✅".green(),
        "Results saved to:".green().bold(),
        file_path.bright_cyan().underline()
    );
    println!(
        "{} {}",
        "📝".bright_white(),
        format!("{} issue(s) written to file", count)
            .bright_white()
            .bold()
    );
    println!();
}
