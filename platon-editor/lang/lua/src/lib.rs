//! Module Lua pour l'éditeur Platon
//! Autocomplétion, documentation, snippets et interprétation via l'exécutable lua.

use std::process::{Command, Output};
use std::io::Write;

/// Mots-clés Lua pour l'autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "and", "break", "do", "else", "elseif", "end", "false", "for",
        "function", "goto", "if", "in", "local", "nil", "not", "or",
        "repeat", "return", "then", "true", "until", "while",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(String::from)
        .collect()
}

/// Documentation simple pour mots-clés Lua.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs = [
        ("and", "Opérateur logique ET."),
        ("break", "Interrompt une boucle."),
        ("do", "Débute un bloc."),
        ("else", "Alternative conditionnelle."),
        ("elseif", "Alternative conditionnelle multiple."),
        ("end", "Fin d'un bloc."),
        ("false", "Valeur booléenne fausse."),
        ("for", "Boucle for."),
        ("function", "Déclare une fonction."),
        ("goto", "Saut à une étiquette."),
        ("if", "Conditionnelle."),
        ("in", "Pour itérer dans une boucle for."),
        ("local", "Déclare une variable locale."),
        ("nil", "Valeur nulle."),
        ("not", "Opérateur logique NON."),
        ("or", "Opérateur logique OU."),
        ("repeat", "Boucle repeat-until."),
        ("return", "Retourne une valeur."),
        ("then", "Débute un bloc conditionnel."),
        ("true", "Valeur booléenne vraie."),
        ("until", "Condition de fin pour repeat."),
        ("while", "Boucle while."),
    ];

    for (k, v) in docs.iter() {
        if *k == keyword {
            return Some(*v);
        }
    }
    None
}

/// Snippet Lua minimal : Hello World
pub const LUA_SNIPPET: &str = r#"print("Hello, world!")"#;

/// Interpréteur Lua basique : exécute via `lua`.
/// Nécessite que l'exécutable `lua` soit dans le PATH.
pub fn interpret(code: &str) -> Result<String, String> {
    use std::fs::File;
    use std::io::Write;
    use std::env::temp_dir;
    use std::process::Output;

    // Créer un fichier temporaire
    let mut path = temp_dir();
    path.push("platon_temp.lua");

    match File::create(&path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(code.as_bytes()) {
                return Err(format!("Erreur écriture fichier temporaire : {}", e));
            }
        }
        Err(e) => return Err(format!("Erreur création fichier temporaire : {}", e)),
    }

    // Exécuter lua sur ce fichier
    let output: Output = match Command::new("lua")
        .arg(&path)
        .output() {
            Ok(out) => out,
            Err(e) => return Err(format!("Erreur exécution lua : {}", e)),
        };

    // Nettoyer fichier temporaire (ignorer erreur)
    let _ = std::fs::remove_file(&path);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Erreur exécution lua : {}", stderr))
    }
}
