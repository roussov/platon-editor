//! Module Fortran pour l'éditeur Platon
//! Autocomplétion, documentation, snippets et interprétation très simplifiée.

use std::collections::HashMap;

/// Mots-clés Fortran pour l'autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "program", "end", "integer", "real", "doubleprecision", "complex", "character",
        "logical", "dimension", "parameter", "data", "print", "read", "write",
        "if", "then", "else", "elseif", "do", "continue", "stop", "call", "function",
        "subroutine", "return", "module", "use", "implicit", "none",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation simple pour mots-clés Fortran.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs: HashMap<&str, &str> = [
        ("program", "Déclare un programme principal."),
        ("end", "Fin d’une structure ou programme."),
        ("integer", "Type entier."),
        ("real", "Type réel (float)."),
        ("doubleprecision", "Type réel double précision."),
        ("complex", "Type complexe."),
        ("character", "Type caractère."),
        ("logical", "Type logique (booléen)."),
        ("dimension", "Déclare un tableau."),
        ("parameter", "Constante symbolique."),
        ("data", "Initialisation de données."),
        ("print", "Affiche une sortie formatée."),
        ("read", "Lit une entrée."),
        ("write", "Écrit sur un flux."),
        ("if", "Conditionnelle."),
        ("then", "Début du bloc conditionnel."),
        ("else", "Alternative."),
        ("elseif", "Sinon si."),
        ("do", "Boucle."),
        ("continue", "Instruction no-op."),
        ("stop", "Arrête l’exécution."),
        ("call", "Appelle une procédure."),
        ("function", "Déclare une fonction."),
        ("subroutine", "Déclare une sous-routine."),
        ("return", "Retourne d’une fonction ou procédure."),
        ("module", "Déclare un module."),
        ("use", "Importe un module."),
        ("implicit", "Contrôle l’implicite des types."),
        ("none", "Désactive l’implicite."),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(keyword).copied()
}

/// Snippet Fortran basique : Hello World
pub const FORTRAN_SNIPPET: &str = r#"program hello
    print *, "Hello, world!"
end program hello
"#;

/// Interpréteur très basique qui extrait la sortie de `print *`
/// Ici on détecte les lignes `print *, "..."` ou `print *, '...'`.
pub fn interpret(code: &str) -> Result<String, String> {
    let mut output = String::new();

    for line in code.lines() {
        let line = line.trim();
        if line.to_lowercase().starts_with("print *") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    let text = &line[start + 1..start + 1 + end];
                    output.push_str(text);
                    output.push('\n');
                } else if let Some(start_squote) = line.find('\'') {
                    if let Some(end_squote) = line[start_squote + 1..].find('\'') {
                        let text = &line[start_squote + 1..start_squote + 1 + end_squote];
                        output.push_str(text);
                        output.push('\n');
                    }
                }
            }
        }
    }

    if output.is_empty() {
        Err("Aucune sortie détectée ou format print * non supporté.".to_string())
    } else {
        Ok(output)
    }
}
