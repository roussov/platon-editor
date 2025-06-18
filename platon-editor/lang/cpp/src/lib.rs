//! Module C++ pour l'éditeur Platon
//! Autocomplétion, documentation, snippets et interprétation très simplifiée.

use std::collections::HashMap;

/// Mots-clés C++ pour l'autocomplétion
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "auto", "bool", "break", "case", "catch", "char", "class", "const", "constexpr",
        "continue", "default", "delete", "do", "double", "else", "enum", "explicit",
        "export", "extern", "false", "float", "for", "friend", "goto", "if", "inline",
        "int", "long", "mutable", "namespace", "new", "noexcept", "nullptr", "operator",
        "private", "protected", "public", "register", "return", "short", "signed",
        "sizeof", "static", "struct", "switch", "template", "this", "throw", "true",
        "try", "typedef", "typeid", "typename", "union", "unsigned", "using", "virtual",
        "void", "volatile", "while",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation simple pour mots-clés C++
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs: HashMap<&str, &str> = [
        ("auto", "Déduit automatiquement le type."),
        ("bool", "Type booléen."),
        ("break", "Sort d'une boucle ou switch."),
        ("case", "Cas dans un switch."),
        ("catch", "Capture une exception."),
        ("char", "Type caractère."),
        ("class", "Déclaration d'une classe."),
        ("const", "Déclare une constante."),
        ("constexpr", "Expression constante évaluée à la compilation."),
        ("continue", "Passe à l'itération suivante."),
        ("default", "Cas par défaut dans un switch."),
        ("delete", "Libère la mémoire allouée dynamiquement."),
        ("do", "Boucle do-while."),
        ("double", "Type flottant double précision."),
        ("else", "Alternative dans une condition."),
        ("enum", "Type énuméré."),
        ("explicit", "Constructeur explicite."),
        ("export", "Spécifie qu'une entité peut être exportée."),
        ("extern", "Lien externe."),
        ("false", "Valeur booléenne fausse."),
         ("float", "Type flottant."),
        ("for", "Boucle for."),
        ("friend", "Déclare une fonction ou classe amie."),
        ("goto", "Saut inconditionnel."),
        ("if", "Conditionnelle."),
        ("inline", "Suggestion d'inlining."),
        ("int", "Type entier."),
        ("long", "Type entier long."),
        ("mutable", "Permet la modification dans un objet const."),
        ("namespace", "Espace de noms."),
        ("new", "Allocation dynamique."),
        ("noexcept", "Spécifie qu'une fonction ne lance pas d'exception."),
        ("nullptr", "Pointeur nul."),
        ("operator", "Déclaration d'un opérateur."),
        ("private", "Accès privé."),
        ("protected", "Accès protégé."),
        ("public", "Accès public."),
        ("register", "Stockage en registre."),
        ("return", "Retourne d'une fonction."),
        ("short", "Type entier court."),
        ("signed", "Type signé."),
        ("sizeof", "Taille d'un type ou variable."),
        ("static", "Stockage statique."),
        ("struct", "Déclaration d'une structure."),
        ("switch", "Structure conditionnelle multiple."),
        ("template", "Modèle générique."),
        ("this", "Pointeur vers l'objet courant."),
        ("throw", "Lance une exception."),
        ("true", "Valeur booléenne vraie."),
        ("try", "Bloc d'essai pour exceptions."),
        ("typedef", "Alias de type."),
        ("typeid", "Information sur un type."),
        ("typename", "Spécifie un nom de type dans un template."),
        ("union", "Déclaration d'une union."),
        ("unsigned", "Type non signé."),
        ("using", "Directive d'import."),
        ("virtual", "Méthode virtuelle."),
        ("void", "Type vide."),
        ("volatile", "Variable volatile."),
        ("while", "Boucle while."),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(keyword).copied()
}

/// Snippet C++ basique : Hello World
pub const CPP_SNIPPET: &str = r#"\
#include <iostream>

int main() {
    std::cout << "Hello, world!" << std::endl;
    return 0;
}
"#;

/// Interpréteur très simplifié qui extrait la sortie de std::cout << "..." << ...
/// Il ne gère que des chaînes littérales simples.
pub fn interpret(code: &str) -> Result<String, String> {
    let mut output = String::new();

    for line in code.lines() {
        let line = line.trim();
        if line.starts_with("std::cout") {
            // Recherche la chaîne entre guillemets
            let start = line.find('"');
            let end = line.rfind('"');
            if let (Some(s), Some(e)) = (start, end) {
                if e > s {
                    let mut text = &line[s+1..e];
                    // Remplace \n par saut de ligne
                    let text = text.replace("\\n", "\n");
                    output.push_str(&text);
                }
            }
        }
    }

    if output.is_empty() {
        Err("Aucune sortie détectée ou format std::cout non supporté.".into())
    } else {
        Ok(output)
    }
}
