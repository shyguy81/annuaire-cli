use shared::generated::{ApiClient, ContactCreate, ContactUpdate};

use crate::cli::CliCommands;

pub async fn handle_cli(command: CliCommands, client: &ApiClient) -> anyhow::Result<()> {
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
