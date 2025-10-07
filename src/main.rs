use anyhow::Result;
use clap::Parser;
use linsearch::{search_issues, Args, LinearClient, SearchOptions};

mod ui {
    pub use linsearch::ui::*;
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = Args::parse();

    // Prompt for API key if not provided
    let api_key = match args.api_key {
        Some(key) => key,
        None => {
            return Err(anyhow::anyhow!(
                "API key required. Set LINEAR_API_KEY environment variable or use --api-key"
            ));
        }
    };

    if api_key.is_empty() {
        anyhow::bail!("API key is required");
    }

    let client = LinearClient::new(api_key);

    // Fetch and select team if not provided
    let team_id = match args.team_id {
        Some(id) => id,
        None => {
            ui::display_fetching_teams();
            let teams = client.fetch_teams().await?;

            if teams.is_empty() {
                anyhow::bail!("No teams found");
            }

            ui::select_team_interactive(&teams)?
        }
    };

    // Prompt for search options if not provided
    if !args.descriptions && !args.comments {
        args.descriptions = ui::confirm_search_descriptions()?;
        args.comments = ui::confirm_search_comments()?;
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

    ui::display_fetching_issues();
    let issues = client.fetch_issues(&team_id).await?;

    let search_options =
        SearchOptions::new(args.search_term.clone(), args.descriptions, args.comments);
    let matches = search_issues(&client, issues, &search_options).await?;

    if client.request_count() >= client.max_requests() {
        ui::display_rate_limit_warning(client.max_requests());
    }

    // Output results - file by default, terminal if --terminal flag is set
    if args.terminal {
        ui::display_results(
            &matches,
            &args.search_term,
            client.request_count(),
            client.max_requests(),
        );
    } else {
        ui::save_results_to_file(
            &args.output,
            &matches,
            &args.search_term,
            client.request_count(),
            client.max_requests(),
        )?;
        ui::display_file_saved(&args.output, matches.len());
    }

    Ok(())
}
