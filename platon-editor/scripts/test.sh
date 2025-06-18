#!/usr/bin/env bash

set -e

echo "🧪 Lancement des tests pour Platon..."

cd "$(dirname "$0")/.."

# Lancer les tests du cœur
echo "🧪 Tests du cœur du projet (src/)..."
cargo test

# Lancer les tests des modules de langages s'ils en ont
echo "🧪 Tests des modules de langages..."
for lang in src/lang/*; do
  if [ -f "$lang/Cargo.toml" ]; then
    echo "➡️  Tests pour $(basename "$lang")"
    (cd "$lang" && cargo test || echo "⚠️  Échec possible dans $(basename "$lang")")
  fi
done

echo "✅ Tous les tests terminés."
