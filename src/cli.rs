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

    /// Lister les interactions d'un contact
    InteractionList {
        contact_id: String,
        #[arg(long, default_value = "0")]
        skip: i64,
        #[arg(long, default_value = "100")]
        limit: i64,
        /// Filtre par type: call, email, meeting, message, other
        #[arg(long = "type")]
        interaction_type: Option<String>,
        #[arg(long)]
        since: Option<String>,
    },
    /// Créer une interaction pour un contact
    InteractionCreate {
        contact_id: String,
        /// Type: call, email, meeting, message, other
        #[arg(long)]
        interaction_type: String,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        interaction_date: Option<String>,
    },

    /// Récupérer le profil relationnel d'un contact
    RelationshipProfileGet { contact_id: String },
    /// Créer le profil relationnel d'un contact
    RelationshipProfileCreate {
        contact_id: String,
        /// spouse, family, business, mentor, friend, acquaintance
        #[arg(long)]
        relationship_type: String,
        /// cold, warm, active, close
        #[arg(long)]
        proximity_level: String,
        /// low, medium, high
        #[arg(long)]
        business_potential: String,
        #[arg(long)]
        trust_level: i64,
    },
    /// Mettre à jour le profil relationnel d'un contact
    RelationshipProfileUpdate {
        contact_id: String,
        #[arg(long)]
        relationship_type: Option<String>,
        #[arg(long)]
        proximity_level: Option<String>,
        #[arg(long)]
        business_potential: Option<String>,
        #[arg(long)]
        trust_level: Option<i64>,
    },

    /// Créer une action relationnelle pour un contact
    RelationshipActionCreate {
        contact_id: String,
        /// followup, relance, candidature, email, call, meeting
        #[arg(long)]
        action_type: String,
        /// low, medium, high
        #[arg(long)]
        priority: String,
        /// todo, in_progress, completed, cancelled
        #[arg(long)]
        status: String,
        #[arg(long)]
        due_date: Option<String>,
    },
    /// Lister les actions relationnelles
    RelationshipActionList {
        #[arg(long, default_value = "0")]
        skip: i64,
        #[arg(long, default_value = "100")]
        limit: i64,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long)]
        contact_id: Option<String>,
    },
    /// Lister les actions relationnelles arrivant à échéance
    RelationshipActionListDue {
        #[arg(long, default_value = "0")]
        skip: i64,
        #[arg(long, default_value = "100")]
        limit: i64,
    },
    /// Mettre à jour une action relationnelle
    RelationshipActionUpdate {
        action_id: String,
        #[arg(long)]
        action_type: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        due_date: Option<String>,
        #[arg(long)]
        completed_at: Option<String>,
    },
    /// Marquer une action relationnelle comme terminée
    RelationshipActionComplete { action_id: String },

    /// Afficher le tableau de bord
    Dashboard,
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
