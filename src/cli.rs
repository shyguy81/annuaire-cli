use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "annuaire-cli")]
#[command(version)]
#[command(about = "CLI pour l'Annuaire de Contacts")]
pub struct Cli {
    #[arg(global = true, long, default_value = "https://annuaire-api.demo.docker.dev")]
    pub api_url: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Commandes d'interaction avec l'API REST
    Cli {
        #[command(subcommand)]
        command: CliCommands,
    },
    /// Interface MCP en mode stdio
    Mcp,
}

#[derive(Subcommand)]
pub enum CliCommands {
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
pub struct CreateArgs {
    #[arg(long)]
    pub nom: String,
    #[arg(long)]
    pub email: String,
    #[arg(long)]
    pub telephone: Option<String>,
    #[arg(long)]
    pub adresse: Option<String>,
    #[arg(long)]
    pub organisation: Option<String>,
    #[arg(long, value_delimiter = ',')]
    pub tags: Vec<String>,
}

#[derive(Args)]
pub struct UpdateArgs {
    #[arg(long)]
    pub nom: Option<String>,
    #[arg(long)]
    pub email: Option<String>,
    #[arg(long)]
    pub telephone: Option<String>,
    #[arg(long)]
    pub adresse: Option<String>,
    #[arg(long)]
    pub organisation: Option<String>,
    #[arg(long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,
}
