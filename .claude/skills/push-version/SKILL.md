---
name: push-version
description: Automate version bump and git tagging for annuaire-cli (Rust)
type: skill
---

# Version Bump & Release Skill

Automatise la montée de version et création de tags git pour `annuaire-cli`.

## Usage

```bash
scripts/version_manager.sh show           # Version actuelle
scripts/version_manager.sh patch           # Bump patch (bugfix)
scripts/version_manager.sh minor           # Bump minor (feature)
scripts/version_manager.sh major           # Bump major (breaking)
scripts/version_manager.sh set 1.2.3       # Version spécifique
scripts/version_manager.sh sync            # Resynchronise Cargo.toml/main.rs sur VERSION
scripts/version_manager.sh tag create      # Crée + push le tag git
```

## Workflow

### Pré-requis

- Aucun changement non-committé (`git status` doit être clean)
- Tous les commits doivent être pushés (aucun commit en attente de push)

### Étapes

1. **Vérification** : `git status` clean + pas de commits en attente de push
2. **Bump version** : `scripts/version_manager.sh <major|minor|patch>` — met à jour `VERSION` et synchronise automatiquement `Cargo.toml`, `shared/Cargo.toml`, `tools/generate-client/Cargo.toml`
3. **Redéploiement local (optionnel)** : `./scripts/deploy_locally.sh` — rebuild + réinstalle le binaire, vérifie que la version installée correspond à `VERSION` (voir skill `deploy-local`)
4. **Git commit** : `git commit -m "chore: bump version to X.Y.Z"`
5. **Git push** : `git push origin main`
6. **Tag + push** : `scripts/version_manager.sh tag create` (crée `vX.Y.Z` et push sur origin)

Pas de workflow GitHub Actions configuré dans ce repo (pas de `.github/workflows/*.yml`) — le tag ne déclenche rien automatiquement.

## Fichiers modifiés

- `VERSION` : source de vérité de la version courante
- `Cargo.toml`, `shared/Cargo.toml`, `tools/generate-client/Cargo.toml` : `version = "X.Y.Z"` — mis à jour automatiquement par `write_version` (bump/set/sync)

`src/main.rs` n'a pas de version en dur : `#[command(version)]` (clap) et le endpoint santé lisent `env!("CARGO_PKG_VERSION")` depuis `Cargo.toml` — aucune synchro nécessaire.

## Rollback

Si erreur avant push:

```bash
git reset --soft HEAD~1
git checkout VERSION Cargo.toml shared/Cargo.toml tools/generate-client/Cargo.toml
```

Si tag déjà créé mais pas encore pushé:

```bash
git tag -d vX.Y.Z
```

Si déjà pushé:

```bash
git push origin :refs/tags/vX.Y.Z
git tag -d vX.Y.Z
```

## Vérification

```bash
git log --oneline -5
git tag -l | tail -5
annuaire-cli --version
```

## Troubleshooting

### Uncommitted changes

```bash
git status
git add <fichiers>
git commit -m "..."
```

### Unpushed commits

```bash
git log --oneline -5
git push origin main
```

### Version binaire ≠ VERSION après bump

Le binaire installé (`~/.local/bin/annuaire-cli`) n'est pas rebuild automatiquement.
Relancer `./scripts/deploy_locally.sh` (échoue explicitement si divergence).

## Related

- `scripts/version_manager.sh` : script de version (show/major/minor/patch/set/sync/tag create)
- `scripts/deploy_locally.sh` : build release + install + vérification de version — skill `deploy-local`
- `VERSION` : fichier source de vérité
