#!/usr/bin/env bash

set -e

echo "🚀 Lancement de l’éditeur Platon (mode développement)..."

cd "$(dirname "$0")/.."

cargo run
