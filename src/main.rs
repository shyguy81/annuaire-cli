use clap::{Args, Parser, Subcommand};
use shared::generated::{ApiClient, ContactCreate, ContactUpdate};

#[derive(Parser)]
#[command(name = "annuaire-cli")]
#[command(version)]
#[command(about = "CLI pour l'Annuaire de Contacts")]
struct Cli {
    #[arg(global = true, long, default_value = "https://annuaire-api.demo.docker.dev")]
    api_url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Commandes d'interaction avec l'API REST
    Cli {
        #[command(subcommand)]
        command: CliCommands,
    },
    /// Interface MCP en mode stdio
    Mcp,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Lister tous les contacts
    List {
        #[arg(long, default_value = "0")]
        skip: i64,
        #[arg(long, default_value = "100")]
        limit: i64,
    },
    /// Récupérer un contact par son ID
    Get { id: String },
    /// Créer un nouveau contact
    Create(CreateArgs),
    /// Mettre à jour un contact
    Update {
        id: String,
        #[command(flatten)]
        fields: UpdateArgs,
    },
    /// Supprimer un contact
    Delete { id: String },
    /// Rechercher des contacts
    Search { query: String },
}

#[derive(Args)]
struct CreateArgs {
    #[arg(long)]
    nom: String,
    #[arg(long)]
    email: String,
    #[arg(long)]
    telephone: Option<String>,
    #[arg(long)]
    adresse: Option<String>,
    #[arg(long)]
    organisation: Option<String>,
    #[arg(long, value_delimiter = ',')]
    tags: Vec<String>,
}

#[derive(Args)]
struct UpdateArgs {
    #[arg(long)]
    nom: Option<String>,
    #[arg(long)]
    email: Option<String>,
    #[arg(long)]
    telephone: Option<String>,
    #[arg(long)]
    adresse: Option<String>,
    #[arg(long)]
    organisation: Option<String>,
    #[arg(long, value_delimiter = ',')]
    tags: Option<Vec<String>>,
}

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

async fn handle_cli(command: CliCommands, client: &ApiClient) -> anyhow::Result<()> {
    match command {
        CliCommands::List { skip, limit } => {
            let contacts = client.list_contacts(Some(skip), Some(limit)).await?;
            println!("{}", serde_json::to_string_pretty(&contacts)?);
        }
        CliCommands::Get { id } => {
            let contact = client.get_contact(&id).await?;
            println!("{}", serde_json::to_string_pretty(&contact)?);
        }
        CliCommands::Create(args) => {
            let body = ContactCreate {
                nom: args.nom,
                email: args.email,
                telephone: args.telephone,
                adresse: args.adresse,
                organisation: args.organisation,
                tags: if args.tags.is_empty() { None } else { Some(args.tags) },
            };
            let contact = client.create_contact(&body).await?;
            println!("{}", serde_json::to_string_pretty(&contact)?);
        }
        CliCommands::Update { id, fields } => {
            let body = ContactUpdate {
                nom: fields.nom,
                email: fields.email,
                telephone: fields.telephone,
                adresse: fields.adresse,
                organisation: fields.organisation,
                tags: fields.tags,
            };
            let contact = client.update_contact(&id, &body).await?;
            println!("{}", serde_json::to_string_pretty(&contact)?);
        }
        CliCommands::Delete { id } => {
            client.delete_contact(&id).await?;
            println!("✅ Contact {} supprimé", id);
        }
        CliCommands::Search { query } => {
            let contacts = client.search_contacts(&query).await?;
            println!("{}", serde_json::to_string_pretty(&contacts)?);
        }
    }
    Ok(())
}

async fn run_mcp() -> anyhow::Result<()> {
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let stdout = io::stdout();

    eprintln!("annuaire-cli MCP server started (stdio mode)");

    let reader = stdin.lock();
    let mut writer = stdout.lock();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let msg: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                let error = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": { "code": -32700, "message": format!("Parse error: {}", e) }
                });
                writeln!(writer, "{}", error)?;
                writer.flush()?;
                continue;
            }
        };

        let id = msg.get("id").cloned().unwrap_or(serde_json::Value::Null);
        let method = msg.get("method").and_then(|m| m.as_str()).unwrap_or("");

        let response = match method {
            "initialize" => serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "annuaire-cli", "version": env!("CARGO_PKG_VERSION") }
                }
            }),
            "tools/list" => {
                let tools = shared::get_mcp_tools();
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "tools": tools }
                })
            }
            "tools/call" => {
                let params = msg.get("params").cloned().unwrap_or(serde_json::Value::Null);
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -32601, "message": format!("Tool not implemented: {}", tool_name) }
                })
            }
            _ => serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32601, "message": format!("Method not found: {}", method) }
            }),
        };

        writeln!(writer, "{}", response)?;
        writer.flush()?;
    }

    Ok(())
}
