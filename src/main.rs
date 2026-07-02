mod cli;
mod handlers;
mod mcp;

use clap::Parser;
use shared::generated::ApiClient;

use cli::{Cli, Commands};
use handlers::handle_cli;
use mcp::run_mcp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = ApiClient::new(&cli.api_url);

    match cli.command {
        Commands::Cli { command } => handle_cli(command, &client).await?,
        Commands::Mcp => run_mcp().await?,
    }

    Ok(())
}
