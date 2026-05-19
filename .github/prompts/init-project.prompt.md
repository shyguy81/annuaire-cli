---
name: init-project
description: Créer une cli Rust avec arguments pour interagir avec une API REST, en utilisant reqwest, tokio, serde, et anyhow.
agent: agent
---

Crée une CLI Rust (nommée `annuaire-cli`).

# context globale

Cette cli propose 2 arbres de commandes :
 - 'annuaire-cli cli' pour les commandes d'interaction avec l'API
 - 'annuaire-cli mcp' pour l'interface mcp en mode stdio

# Definition openapi

L'url est https://annuaire-api.demo.docker.dev/openapi.json

Il convient de télécharger le fichier openapi.json et de générer les clients et modèles à partir de ce fichier.

# Génération du client et des modèles

Le générateur du client api est un outil en Rust et sert à la fois pour l'arbre cli autant que pour l'arbre mcp.

dossier du générateur : tools/generate-client
le client généré est mis à disposition dans shared/src/generated

le script /scripts/generate-client.sh doit être mis à jour pour télécharger le fichier openapi.json et générer les clients et modèles à partir de ce fichier.

# Commandes cli

L'arbre cli doit proposer les commandes suivantes :
- 'annuaire-cli cli list' pour lister les contacts
- 'annuaire-cli cli get <id>' pour récupérer un contact par son id
- 'annuaire-cli cli create <contact>' pour créer un contact
- 'annuaire-cli cli update <id> <contact>' pour mettre à jour un contact
- 'annuaire-cli cli delete <id>' pour supprimer un contact  

Utiliser le client généré pour implémenter ces commandes.

# Versionning 

Un fichier VERSION doit être créé à la racine du projet et doit contenir la version de la cli. Cette version doit être utilisée dans le Cargo.toml et dans les commandes de build et de publication.

Utiliser le script /scripts/version_manager.sh pour gérer les versions. 

Ce script doit être mis à jour pour incrémenter la version dans le fichier VERSION et dans le Cargo.toml.

- 'annuaire-cli --version' pour afficher la version de la cli  

# Déploiement 

Le déploiement se fait via le script deploy_locally.sh qui doit être mis à jour pour installer la cli annuaire-cli. 

Dossier de déploiement : ~/.local/bin

Ce script doit être idempotent et doit permettre de réinstaller la cli en cas de besoin.