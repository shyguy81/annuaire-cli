use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use shared::generated::{
    ApiClient, ContactCreate, ContactUpdate, InteractionCreate, RelationshipActionCreate,
    RelationshipActionUpdate, RelationshipProfileCreate, RelationshipProfileUpdate,
};

use crate::cli::CliCommands;

fn parse_enum<T: DeserializeOwned>(field: &str, value: &str) -> Result<T> {
    serde_json::from_value(serde_json::Value::String(value.to_string()))
        .with_context(|| format!("valeur invalide pour {field}: \"{value}\""))
}

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

        CliCommands::InteractionList { contact_id, skip, limit, interaction_type, since } => {
            let interactions = client
                .list_interactions(&contact_id, Some(skip), Some(limit), interaction_type, since)
                .await?;
            println!("{}", serde_json::to_string_pretty(&interactions)?);
        }
        CliCommands::InteractionCreate { contact_id, interaction_type, notes, interaction_date } => {
            let body = InteractionCreate {
                interaction_type: parse_enum("interaction_type", &interaction_type)?,
                notes,
                interaction_date,
            };
            let interaction = client.create_interaction(&contact_id, &body).await?;
            println!("{}", serde_json::to_string_pretty(&interaction)?);
        }

        CliCommands::RelationshipProfileGet { contact_id } => {
            let profile = client.get_relationship_profile(&contact_id).await?;
            println!("{}", serde_json::to_string_pretty(&profile)?);
        }
        CliCommands::RelationshipProfileCreate {
            contact_id,
            relationship_type,
            proximity_level,
            business_potential,
            trust_level,
        } => {
            let body = RelationshipProfileCreate {
                relationship_type: parse_enum("relationship_type", &relationship_type)?,
                proximity_level: parse_enum("proximity_level", &proximity_level)?,
                business_potential: parse_enum("business_potential", &business_potential)?,
                trust_level,
            };
            let profile = client.create_relationship_profile(&contact_id, &body).await?;
            println!("{}", serde_json::to_string_pretty(&profile)?);
        }
        CliCommands::RelationshipProfileUpdate {
            contact_id,
            relationship_type,
            proximity_level,
            business_potential,
            trust_level,
        } => {
            let body = RelationshipProfileUpdate {
                relationship_type: relationship_type
                    .map(|v| parse_enum("relationship_type", &v))
                    .transpose()?,
                proximity_level: proximity_level
                    .map(|v| parse_enum("proximity_level", &v))
                    .transpose()?,
                business_potential: business_potential
                    .map(|v| parse_enum("business_potential", &v))
                    .transpose()?,
                trust_level,
            };
            let profile = client.update_relationship_profile(&contact_id, &body).await?;
            println!("{}", serde_json::to_string_pretty(&profile)?);
        }

        CliCommands::RelationshipActionCreate { contact_id, action_type, priority, status, due_date } => {
            let body = RelationshipActionCreate {
                action_type: parse_enum("action_type", &action_type)?,
                priority: parse_enum("priority", &priority)?,
                status: parse_enum("status", &status)?,
                due_date,
            };
            let action = client.create_relationship_action(&contact_id, &body).await?;
            println!("{}", serde_json::to_string_pretty(&action)?);
        }
        CliCommands::RelationshipActionList { skip, limit, status, priority, contact_id } => {
            let actions = client
                .list_relationship_actions(Some(skip), Some(limit), status, priority, contact_id)
                .await?;
            println!("{}", serde_json::to_string_pretty(&actions)?);
        }
        CliCommands::RelationshipActionListDue { skip, limit } => {
            let actions = client.list_due_relationship_actions(Some(skip), Some(limit)).await?;
            println!("{}", serde_json::to_string_pretty(&actions)?);
        }
        CliCommands::RelationshipActionUpdate {
            action_id,
            action_type,
            priority,
            status,
            due_date,
            completed_at,
        } => {
            let body = RelationshipActionUpdate {
                action_type: action_type.map(|v| parse_enum("action_type", &v)).transpose()?,
                priority: priority.map(|v| parse_enum("priority", &v)).transpose()?,
                status: status.map(|v| parse_enum("status", &v)).transpose()?,
                due_date,
                completed_at,
            };
            let action = client.update_relationship_action(&action_id, &body).await?;
            println!("{}", serde_json::to_string_pretty(&action)?);
        }
        CliCommands::RelationshipActionComplete { action_id } => {
            let action = client.complete_relationship_action(&action_id).await?;
            println!("{}", serde_json::to_string_pretty(&action)?);
        }

        CliCommands::Dashboard => {
            let dashboard = client.get_dashboard().await?;
            println!("{}", serde_json::to_string_pretty(&dashboard)?);
        }
    }
    Ok(())
}
