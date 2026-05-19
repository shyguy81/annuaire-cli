# Generate Client Tool

Outil de génération des clients API Rust depuis les specs OpenAPI.

## Architecture

Le processus de génération en **deux étapes** :

```
┌─────────────────────────────────────────┐
│ Étape 1 : Téléchargement des specs     │
│ ────────────────────────────────────    │
│ Récupère les specs depuis le serveur   │
│ API et les sauvegarde localement       │
│ dans `.specs/`                          │
└──────────┬──────────────────────────────┘
           ↓
┌─────────────────────────────────────────┐
│ Étape 2 : Génération des clients       │
│ ────────────────────────────────────    │
│ Lit les specs depuis `.specs/`         │
│ et génère le code Rust                 │
└─────────────────────────────────────────┘
```

## Usage

Depuis le root du workspace :

```bash
# Compilation et génération
cargo run -p generate-client

# Ou via script
./scripts/generate-client.sh
```

## Processus détaillé

### Étape 1 : Téléchargement
1. **Télécharge** la spec OpenAPI depuis le serveur API :
   - `https://atm-api.local.docker.dev/v3/api-docs/public-mcp-api` → `.specs/mcp-api.json`
   - `https://atm-api.local.docker.dev/v3/api-docs/admin-api` → `.specs/admin-api.json`
2. **Valide** le JSON
3. **Sauvegarde** localement pour traçabilité

### Étape 2 : Génération
1. **Lit** les specs depuis `.specs/`
2. **Parse** le JSON
3. **Génère** les clients Rust typés

## Sortie

Génère dans `shared/src/generated/` :
- `mcp_client.rs` — Client API public (MCP)
- `mcp_types.rs` — Types générés depuis la spec MCP
- `admin_client.rs` — Client API admin
- `admin_types.rs` — Types générés depuis la spec admin
- `mod.rs` — Module d'exposition

## Structure des fichiers

```
.specs/                    ← Specs OpenAPI téléchargées (gitignored)
├── mcp-api.json         ← Spec publique brute
└── admin-api.json       ← Spec admin brute

shared/src/generated/     ← Code généré (suivi en Git)
├── mcp_client.rs        ← Client public
├── mcp_types.rs         ← Types publics
├── admin_client.rs      ← Client admin
├── admin_types.rs       ← Types admin
└── mod.rs               ← Exports
```

## Notes importantes

### Fichiers generés (`.specs/`)
- 📂 **Gitignored** : `.specs/` n'est pas commité
- 🔄 **Régénérés automatiquement** : `cargo run -p generate-client` les met à jour
- 📋 **Traçabilité** : Stockent la version brute du serveur API

### Fichiers Rust générés (`shared/src/generated/`)
- 📝 **Commités en Git** : Trace l'évolution de l'API
- ⚠️ **Read-only** : Ne pas modifier manuellement (changements perdus au prochain build)
- 🔄 **Auto-générés** : Régénérés depuis les specs

## Workflow : Ajouter une nouvelle API

1. Ajouter une nouvelle config dans `tools/generate-client/src/main.rs` :
   ```rust
   ClientConfig {
       name: "NewAPI",
       spec_url: "https://...",
       spec_file: ".specs/new-api.json",
       client_file: "new_client.rs",
       types_file: "new_types.rs",
       tools_file: None,
   }
   ```
2. Exécuter : `cargo run -p generate-client`
3. Exporter dans `shared/src/generated/mod.rs`
4. Commiter les fichiers générés

## Dépannage

### Les specs ne se téléchargent pas
```bash
# Vérifier la connectivité au serveur API
curl https://atm-api.local.docker.dev/v3/api-docs/public-mcp-api

# Forcer le re-téléchargement (supprimer .specs/)
rm -rf .specs/
cargo run -p generate-client
```

### Erreur "Erreur lecture spec locale"
- Le dossier `.specs/` n'existe pas → exécuter `cargo run -p generate-client` pour le créer
- Le fichier spec est corrompu → supprimer `.specs/` et réexécuter

### Fichier généré ne se met à pas à jour
```bash
# Vérifier que la spec est bien téléchargée
cat .specs/mcp-api.json | head -20

# Régénérer tout de zéro
rm -rf .specs/
cargo run -p generate-client
```
