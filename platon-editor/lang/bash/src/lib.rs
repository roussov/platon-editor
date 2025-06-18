//! Module Bash pour l'éditeur Platon
//! Autocomplétion, documentation, snippets, interprétation basique.

use std::collections::HashMap;

/// Liste des commandes Bash courantes pour autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "echo", "cd", "ls", "pwd", "cat", "grep", "awk", "sed", "exit", "export",
        "alias", "unalias", "source", "test", "if", "then", "else", "fi", "for",
        "while", "do", "done", "case", "esac", "function", "read", "break", "continue",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation des commandes Bash.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs: HashMap<&str, &str> = [
        ("echo", "Affiche une ligne de texte."),
        ("cd", "Change le répertoire courant."),
        ("ls", "Liste le contenu d'un répertoire."),
        ("pwd", "Affiche le chemin du répertoire courant."),
        ("cat", "Concatène et affiche des fichiers."),
        ("grep", "Recherche une chaîne dans un fichier."),
        ("awk", "Traitement de texte et extraction de données."),
        ("sed", "Éditeur de flux pour modifications de texte."),
        ("exit", "Quitte le shell ou script."),
        ("export", "Définit une variable d'environnement."),
        ("alias", "Crée un alias de commande."),
        ("unalias", "Supprime un alias."),
        ("source", "Exécute un script dans le shell courant."),
        ("test", "Évalue une expression conditionnelle."),
        ("if", "Instruction conditionnelle."),
        ("then", "Début du bloc alors."),
        ("else", "Bloc sinon."),
        ("fi", "Fin de la condition if."),
        ("for", "Boucle for."),
        ("while", "Boucle while."),
        ("do", "Début du bloc de boucle."),
        ("done", "Fin du bloc de boucle."),
        ("case", "Structure de sélection multiple."),
        ("esac", "Fin de la structure case."),
        ("function", "Déclare une fonction."),
        ("read", "Lit une entrée utilisateur."),
        ("break", "Interrompt une boucle."),
        ("continue", "Passe à l'itération suivante."),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(keyword).copied()
}

/// Snippet d'exemple Bash
pub const BASH_SNIPPET: &str = r#"#!/bin/bash
# Script Bash simple

echo "Hello, World!"

for i in {1..5}; do
    echo "Compteur: $i"
done
"#;

/// Interprétation basique du code Bash via l'appel au shell.
/// ATTENTION : Cette fonction exécute du code arbitraire sur la machine locale.
use std::process::Command;

pub fn interpret(code: &str) -> Result<String, String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(code)
        .output()
        .map_err(|e| format!("Erreur lors de l'exécution : {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(stderr)
    }
}
