use std::process::{Command, Stdio};

/// Liste des mots-clés PHP pour autocomplétion.
const KEYWORDS: &[&str] = &[
    "abstract", "and", "array", "as", "break", "callable", "case", "catch",
    "class", "clone", "const", "continue", "declare", "default", "do", "echo",
    "else", "elseif", "enddeclare", "endfor", "endforeach", "endif", "endswitch",
    "endwhile", "extends", "final", "finally", "for", "foreach", "function",
    "global", "goto", "if", "implements", "include", "include_once", "instanceof",
    "insteadof", "interface", "isset", "list", "namespace", "new", "or", "print",
    "private", "protected", "public", "require", "require_once", "return", "static",
    "switch", "throw", "trait", "try", "unset", "use", "var", "while", "xor",
    "yield"
];

/// Snippet PHP simple (exemple Hello World)
pub const PHP_SNIPPET: &str = r#"<?php
// Exemple basique PHP
echo "Hello, World!";
?>"#;

/// Retourne la liste de suggestions selon un préfixe.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    KEYWORDS.iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Retourne la documentation pour un mot-clé PHP, si disponible.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    match keyword {
        "echo" => Some("echo — Output one or more strings"),
        "function" => Some("function — Define a function"),
        "class" => Some("class — Define a class"),
        "if" => Some("if — Conditional statement"),
        "foreach" => Some("foreach — Loop over iterable elements"),
        "require" => Some("require — Include and evaluate a file"),
        "include" => Some("include — Include and evaluate a file"),
        "namespace" => Some("namespace — Declare namespace"),
        "try" => Some("try — Start a try block for exception handling"),
        "catch" => Some("catch — Catch exceptions"),
        _ => None,
    }
}

/// Interprète un code PHP en l'exécutant avec la commande `php` du système.
/// Retourne stdout ou stderr en cas d'erreur.
pub fn interpret(code: &str) -> Result<String, String> {
    // On lance `php` en mode exécution à partir d'une pipe stdin
    let mut process = match Command::new("php")
        .arg("-r")
        .arg(code)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() {
            Ok(proc) => proc,
            Err(e) => return Err(format!("Erreur lancement PHP: {}", e)),
        };

    let output = process.wait_with_output().map_err(|e| e.to_string())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| e.to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
