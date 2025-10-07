use anyhow::Result;
use clap::Parser;
use linsearch::{search_issues, Args, LinearClient, SearchOptions};

mod api {
    pub use linsearch::api::*;
}
mod ui {
    pub use linsearch::ui::*;
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = Args::parse();

    // Prompt for API key if not provided
    let api_key = match args.api_key {
        Some(key) => key,
        None => ui::prompt("Enter Linear API key: ")?,
    };

    if api_key.is_empty() {
        anyhow::bail!("API key is required");
    }

    let client = LinearClient::new(api_key);

    // Fetch and select team if not provided
    let team_id = match args.team_id {
        Some(id) => id,
        None => {
            println!("Fetching teams...");
            let teams = client.fetch_teams().await?;

            if teams.is_empty() {
                anyhow::bail!("No teams found");
            }

            ui::display_teams(&teams);

            let choice = ui::prompt("\nEnter team number or team ID: ")?;

            ui::select_team(&teams, &choice)
                .ok_or_else(|| anyhow::anyhow!("Invalid team selection"))?
        }
    };

    // Prompt for search options if not provided
    if !args.descriptions && !args.comments {
        let desc_input = ui::prompt("Search in descriptions? (y/n): ")?;
        args.descriptions =
            desc_input.eq_ignore_ascii_case("y") || desc_input.eq_ignore_ascii_case("yes");

        let comments_input = ui::prompt("Search in comments? (y/n): ")?;
        args.comments =
            comments_input.eq_ignore_ascii_case("y") || comments_input.eq_ignore_ascii_case("yes");
    }

    if !args.descriptions && !args.comments {
        anyhow::bail!("Must search in at least descriptions or comments");
    }

    ui::display_search_info(
        &args.search_term,
        client.max_requests(),
        args.descriptions,
        args.comments,
    );

    let issues = client.fetch_issues(&team_id).await?;

    let search_options = SearchOptions::new(args.search_term.clone(), args.descriptions, args.comments);
    let matches = search_issues(&client, issues, &search_options).await?;

    if client.request_count() >= client.max_requests() {
        ui::display_rate_limit_warning(client.max_requests());
    }

    ui::display_results(&matches, &args.search_term, client.request_count(), client.max_requests());

    Ok(())
}