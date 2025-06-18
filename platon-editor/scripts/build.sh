#!/usr/bin/env bash

set -e

# === Couleurs pour affichage ===
GREEN='\033[0;32m'
CYAN='\033[1;36m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${CYAN}🔧 Construction complète de l'éditeur Platon${NC}"

cd "$(dirname "$0")/.."

# Nettoyage optionnel
if [[ "$1" == "--clean" ]]; then
    echo -e "${CYAN}🧹 Nettoyage du projet...${NC}"
    cargo clean
fi

echo -e "${CYAN}📦 Compilation du cœur du projet...${NC}"
cargo build --release

# Compilation des modules lang/ si Cargo.toml présent
echo -e "${CYAN}🌐 Compilation des langages (modules dans src/lang)...${NC}"
for dir in src/lang/*; do
    if [ -d "$dir" ] && [ -f "$dir/Cargo.toml" ]; then
        lang=$(basename "$dir")
        echo -e "➡️  ${GREEN}Compilation du module $lang${NC}"
        (cd "$dir" && cargo build --release)
    fi
done

# Vérifie si le binaire est généré
BIN=target/release/platon
if [ -f "$BIN" ]; then
    echo -e "${GREEN}✅ Compilation réussie.${NC}"
    echo -e "${CYAN}📏 Taille du binaire :$(du -h "$BIN" | cut -f1)${NC}"
else
    echo -e "${RED}❌ Erreur : le binaire Platon n’a pas été généré correctement.${NC}"
    exit 1
fi

# Option : copier dans un dossier /bin local
mkdir -p ~/.local/bin && cp "$BIN" ~/.local/bin/platon

# Lancement automatique
# echo -e "${CYAN}🚀 Lancement de l’éditeur Platon...${NC}"
# ./target/release/platon
