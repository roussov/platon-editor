//! Module Go pour l'éditeur Platon
//! Autocomplétion, documentation, snippets et interprétation.

use std::process::{Command, Stdio};
use std::io::Write;

/// Mots-clés Go pour l'autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "package", "import", "func", "var", "const", "type",
        "struct", "interface", "map", "chan", "go",
        "defer", "return", "if", "else", "for", "range",
        "switch", "case", "default", "break", "continue",
        "select", "fallthrough", "goto", "nil", "true", "false",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation simple pour mots-clés Go.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs = [
        ("package", "Déclare le paquet du fichier source."),
        ("import", "Importe un ou plusieurs paquets."),
        ("func", "Déclare une fonction."),
        ("var", "Déclare une ou plusieurs variables."),
        ("const", "Déclare une constante."),
        ("type", "Déclare un nouveau type."),
        ("struct", "Déclare une structure."),
        ("interface", "Déclare une interface."),
        ("map", "Déclare une table de hachage."),
        ("chan", "Déclare un canal de communication."),
        ("go", "Lance une goroutine."),
        ("defer", "Diffère l'exécution d'une fonction."),
        ("return", "Retourne une valeur."),
        ("if", "Condition."),
        ("else", "Alternative conditionnelle."),
        ("for", "Boucle."),
        ("range", "Itère sur une collection."),
        ("switch", "Sélecteur multi-cas."),
        ("case", "Cas dans un switch."),
        ("default", "Cas par défaut."),
        ("break", "Sort d'une boucle ou switch."),
        ("continue", "Passe à l'itération suivante."),
        ("select", "Multiplexe sur des canaux."),
        ("fallthrough", "Passe au cas suivant dans switch."),
        ("goto", "Effectue un saut."),
        ("nil", "Valeur nulle pour pointeurs et interfaces."),
        ("true", "Valeur booléenne vraie."),
        ("false", "Valeur booléenne fausse."),
    ];

    for (k, v) in docs.iter() {
        if *k == keyword {
            return Some(*v);
        }
    }
    None
}

/// Snippet Go minimal : Hello World
pub const GO_SNIPPET: &str = r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, world!")
}
"#;

/// Interpréteur Go basique qui exécute du code via `go run`.
/// Nécessite que la commande `go` soit installée et dans le PATH.
pub fn interpret(code: &str) -> Result<String, String> {
    use std::fs::File;
    use std::io::Write;
    use std::env::temp_dir;
    use std::process::Output;

    // Création d'un fichier temporaire
    let mut path = temp_dir();
    path.push("platon_temp.go");

    // Écriture du code dans le fichier temporaire
    match File::create(&path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(code.as_bytes()) {
                return Err(format!("Erreur écriture fichier temporaire : {}", e));
            }
        }
        Err(e) => {
            return Err(format!("Erreur création fichier temporaire : {}", e));
        }
    }

    // Exécution de la commande `go run`
    let output: Output = match Command::new("go")
        .arg("run")
        .arg(&path)
        .output() {
            Ok(output) => output,
            Err(e) => return Err(format!("Erreur exécution go run : {}", e)),
        };

    // Nettoyage du fichier temporaire (ignorer erreur)
    let _ = std::fs::remove_file(&path);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Erreur exécution go run : {}", stderr))
    }
}
