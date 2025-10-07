use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "linsearch")]
#[command(about = "Search Linear issues for specific terms", long_about = None)]
#[command(version)]
pub struct Args {
    /// Search term to look for
    #[arg(value_name = "SEARCH_TERM")]
    pub search_term: String,

    /// Linear API key
    #[arg(short, long, env = "LINEAR_API_KEY")]
    pub api_key: Option<String>,

    /// Team ID to search within
    #[arg(short, long)]
    pub team_id: Option<String>,

    /// Search in descriptions
    #[arg(short, long)]
    pub descriptions: bool,

    /// Search in comments
    #[arg(short, long)]
    pub comments: bool,

    /// Output file path (defaults to linsearch-results.md)
    #[arg(short, long, default_value = "linsearch-results.md")]
    pub output: String,

    /// Display results in terminal instead of saving to file
    #[arg(long)]
    pub terminal: bool,
}
