//! Module C pour l'éditeur Platon
//! Autocomplétion, documentation, snippets et simulation simple d'exécution.

use std::collections::HashMap;

/// Liste des mots-clés C pour l'autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "int", "return", "if", "else", "for", "while", "do", "break", "continue",
        "void", "char", "float", "double", "struct", "typedef", "sizeof", "switch",
        "case", "default", "const", "static", "enum", "extern", "goto",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation simple des mots-clés C.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs: HashMap<&str, &str> = [
        ("int", "Déclare une variable entière."),
        ("return", "Retourne une valeur d'une fonction."),
        ("if", "Conditionnelle."),
        ("else", "Alternative à if."),
        ("for", "Boucle for."),
        ("while", "Boucle while."),
        ("do", "Boucle do-while."),
        ("break", "Sort de la boucle ou switch."),
        ("continue", "Passe à l'itération suivante de la boucle."),
        ("void", "Type vide, sans valeur."),
        ("char", "Type caractère."),
        ("float", "Type flottant."),
        ("double", "Type flottant double précision."),
        ("struct", "Déclaration d'une structure."),
        ("typedef", "Définition d'un nouveau type."),
        ("sizeof", "Retourne la taille d'un type ou variable."),
        ("switch", "Structure conditionnelle multiple."),
        ("case", "Cas dans un switch."),
        ("default", "Cas par défaut dans switch."),
        ("const", "Constante."),
        ("static", "Durée de vie statique."),
        ("enum", "Type énuméré."),
        ("extern", "Lien externe."),
        ("goto", "Saut inconditionnel."),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(keyword).copied()
}

/// Snippet C basique : Hello World
pub const C_SNIPPET: &str = r#"#include <stdio.h>

int main() {
    printf("Hello, world!\n");
    return 0;
}
"#;

/// Simulation d'exécution basique : analyse très simplifiée
///
/// Note : C est un langage compilé, l'interprétation réelle est complexe.
/// Ici on simule juste la reconnaissance de printf avec chaîne constante.
pub fn interpret(code: &str) -> Result<String, String> {
    // Cherche printf("...") et extrait le texte affiché (simple)
    let mut output = String::new();

    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("printf") {
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start+1..].find('"') {
                    let text = &trimmed[start+1..start+1+end];
                    // remplace \n par saut de ligne
                    let text = text.replace("\\n", "\n");
                    output.push_str(&text);
                }
            }
        }
    }

    if output.is_empty() {
        Err("Aucune sortie détectée ou format printf non supporté.".to_string())
    } else {
        Ok(output)
    }
}
