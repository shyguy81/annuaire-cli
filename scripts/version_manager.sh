#!/bin/bash

VERSION_FILE="VERSION"
CARGO_FILES=("Cargo.toml" "shared/Cargo.toml" "tools/generate-client/Cargo.toml")

# Fonctions utilitaires
read_version() {
    if [[ -f "$VERSION_FILE" ]]; then
        cat "$VERSION_FILE" | tr -d '\n\r'
    else
        echo "0.0.1"
        echo "0.0.1" > "$VERSION_FILE"
    fi
}

update_cargo_versions() {
    local new_version="$1"
    local updated_count=0
    
    for cargo_file in "${CARGO_FILES[@]}"; do
        if [[ -f "$cargo_file" ]]; then
            # Utilise sed pour remplacer la version dans les fichiers Cargo.toml
            # Cherche la ligne "version = " et la remplace par la nouvelle version
            sed -i.bak "s/^version = \"[^\"]*\"$/version = \"${new_version}\"/g" "$cargo_file"
            
            # Supprime le fichier de sauvegarde
            rm -f "${cargo_file}.bak"
            
            updated_count=$((updated_count + 1))
            echo "✓ $cargo_file mis à jour avec la version: $new_version"
        fi
    done
    
    if [[ $updated_count -eq 0 ]]; then
        echo "Aucun fichier Cargo.toml trouvé"
    else
        echo "✓ $updated_count fichier(s) Cargo.toml mis à jour"
    fi
}

write_version() {
    echo "$1" > "$VERSION_FILE"
    update_cargo_versions "$1"
    echo "✓ Version mise à jour: $1"
}

validate_version() {
    if [[ $1 =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        return 0
    else
        echo "Erreur: Format de version invalide. Utilisez x.y.z"
        return 1
    fi
}

# Fonctions de versioning
bump_major() {
    local current=$(read_version)
    local major=$(echo $current | cut -d. -f1)
    local new_version="$((major + 1)).0.0"
    write_version "$new_version"
}

bump_minor() {
    local current=$(read_version)
    local major=$(echo $current | cut -d. -f1)
    local minor=$(echo $current | cut -d. -f2)
    local new_version="$major.$((minor + 1)).0"
    write_version "$new_version"
}

bump_patch() {
    local current=$(read_version)
    local major=$(echo $current | cut -d. -f1)
    local minor=$(echo $current | cut -d. -f2)
    local patch=$(echo $current | cut -d. -f3)
    local new_version="$major.$minor.$((patch + 1))"
    write_version "$new_version"
}

show_version() {
    echo "✓ Version actuelle: $(read_version)"
}

show_help() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commandes:"
    echo "  show                 Affiche la version actuelle"
    echo "  major               Incrémente la version majeure (x.0.0)"
    echo "  minor               Incrémente la version mineure (x.y.0)"
    echo "  patch               Incrémente la version patch (x.y.z)"
    echo "  set <version>       Définit une version spécifique"
    echo "  sync                Synchronise tous les Cargo.toml avec la version du fichier VERSION"
    echo "  tag create          Crée et pousse le tag Git correspondant à la version (dep: git)"
    echo "  help                Affiche cette aide"
}

# Crée un tag Git à partir du fichier VERSION et le pousse sur l'origine
create_tag() {
    local current_version
    current_version=$(read_version)
    local tag_name="v${current_version}"

    if ! command -v git >/dev/null 2>&1; then
        echo "Erreur: git n'est pas installé ou non disponible dans le PATH"
        return 1
    fi

    # Optionnel: vérifier qu'on est dans un repo git
    if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        echo "Erreur: le répertoire courant n'est pas un dépôt git"
        return 1
    fi

    # Crée le tag local
    git tag "$tag_name"
    if [[ $? -ne 0 ]]; then
        echo "Erreur: échec de la création du tag $tag_name"
        return 1
    fi

    # Pousse le tag vers origin
    git push origin "$tag_name"
    if [[ $? -ne 0 ]]; then
        echo "Erreur: échec du push du tag $tag_name vers origin"
        return 1
    fi

    echo "Tag créé et poussé: $tag_name"
}

# Script principal
case "$1" in
    "show"|"")
        show_version
        ;;
    "major")
        bump_major
        ;;
    "minor")
        bump_minor
        ;;
    "patch")
        bump_patch
        ;;
    "set")
        if [[ -z "$2" ]]; then
            echo "Erreur: Version requise pour la commande 'set'"
            exit 1
        fi
        if validate_version "$2"; then
            write_version "$2"
        else
            exit 1
        fi
        ;;
    "sync")
        current_version=$(read_version)
        update_cargo_versions "$current_version"
        echo "✓ Synchronisation terminée avec la version: $current_version"
        ;;
    "tag")
        if [[ "$2" == "create" ]]; then
            create_tag
        else
            echo "Usage: $0 tag create"
            exit 1
        fi
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    *)
        echo "Commande inconnue: $1"
        show_help
        exit 1
        ;;
esac
