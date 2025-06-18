//! Module Kotlin pour l'éditeur Platon
//! Autocomplétion, documentation, snippets et interprétation via kotlinc/kotlin.

use std::process::{Command, Stdio};
use std::io::Write;

/// Mots-clés Kotlin pour l'autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "as", "break", "class", "continue", "do", "else", "false", "for",
        "fun", "if", "in", "interface", "is", "null", "object", "package",
        "return", "super", "this", "throw", "true", "try", "typealias",
        "val", "var", "when", "while", "by", "catch", "constructor",
        "delegate", "dynamic", "field", "file", "finally", "get", "import",
        "init", "param", "property", "set", "setparam", "where",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(String::from)
        .collect()
}

/// Documentation simple pour mots-clés Kotlin.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs = [
        ("as", "Cast explicite."),
        ("break", "Sort d'une boucle."),
        ("class", "Déclare une classe."),
        ("continue", "Passe à l'itération suivante."),
        ("do", "Boucle do-while."),
        ("else", "Alternative conditionnelle."),
        ("false", "Valeur booléenne false."),
        ("for", "Boucle for."),
        ("fun", "Déclare une fonction."),
        ("if", "Conditionnelle."),
        ("in", "Teste l'appartenance."),
        ("interface", "Déclare une interface."),
        ("is", "Test de type."),
        ("null", "Valeur nulle."),
        ("object", "Déclare un singleton."),
        ("package", "Déclare un paquet."),
        ("return", "Retourne une valeur."),
        ("super", "Référence à la superclasse."),
        ("this", "Référence à l'objet courant."),
        ("throw", "Lance une exception."),
        ("true", "Valeur booléenne true."),
        ("try", "Bloc d'exception."),
        ("typealias", "Alias de type."),
        ("val", "Variable immutable."),
        ("var", "Variable mutable."),
        ("when", "Expression conditionnelle."),
        ("while", "Boucle while."),
        ("by", "Délégué."),
        ("catch", "Capture une exception."),
        ("constructor", "Constructeur."),
        ("delegate", "Délégué."),
        ("dynamic", "Type dynamique."),
        ("field", "Champ."),
        ("file", "Fichier."),
        ("finally", "Bloc finally."),
        ("get", "Accesseur."),
        ("import", "Importe un paquet."),
        ("init", "Bloc d'initialisation."),
        ("param", "Paramètre."),
        ("property", "Propriété."),
        ("set", "Mutateur."),
        ("setparam", "Paramètre du mutateur."),
        ("where", "Clause where."),
    ];

    for (k, v) in docs.iter() {
        if *k == keyword {
            return Some(*v);
        }
    }
    None
}

/// Snippet Kotlin minimal : Hello World
pub const KOTLIN_SNIPPET: &str = r#"fun main() {
    println("Hello, world!")
}
"#;

/// Interpréteur Kotlin basique : compile et exécute via kotlinc puis kotlin.
/// Nécessite que kotlinc et kotlin soient dans le PATH.
pub fn interpret(code: &str) -> Result<String, String> {
    use std::fs::{File, remove_file};
    use std::io::Write;
    use std::env::temp_dir;
    use std::path::PathBuf;
    use std::process::Output;

    let tmp_dir = temp_dir();
    let source_path = tmp_dir.join("Main.kt");

    // Écrire le code dans Main.kt
    match File::create(&source_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(code.as_bytes()) {
                return Err(format!("Erreur écriture fichier source : {}", e));
            }
        }
        Err(e) => return Err(format!("Erreur création fichier source : {}", e)),
    }

    // Compiler avec kotlinc
    let compile_output = Command::new("kotlinc")
        .arg(&source_path)
        .arg("-d")
        .arg(&tmp_dir)
        .output()
        .map_err(|e| format!("Erreur lancement kotlinc : {}", e))?;

    if !compile_output.status.success() {
        let err = String::from_utf8_lossy(&compile_output.stderr);
        let err_msg = if err.is_empty() {
            "Erreur inconnue à la compilation".to_string()
        } else {
            err.to_string()
        };
        let _ = remove_file(&source_path);
        return Err(format!("Erreur compilation : {}", err_msg));
    }

    // Exécuter la classe compilée avec kotlin
    let run_output = Command::new("kotlin")
        .current_dir(&tmp_dir)
        .arg("MainKt")
        .output()
        .map_err(|e| format!("Erreur lancement kotlin : {}", e))?;

    // Nettoyer fichiers générés
    let _ = remove_file(&source_path);
    let _ = remove_file(tmp_dir.join("MainKt.class"));
    let _ = remove_file(tmp_dir.join("MainKt\$main.class"));

    if run_output.status.success() {
        Ok(String::from_utf8_lossy(&run_output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&run_output.stderr).to_string())
    }
}
