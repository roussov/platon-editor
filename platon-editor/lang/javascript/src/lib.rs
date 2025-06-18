//! Module JavaScript pour l'éditeur Platon
//! Autocomplétion, documentation, snippets et interprétation via Node.js.

use std::process::{Command, Stdio};
use std::io::Write;

/// Mots-clés JavaScript pour l'autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "break", "case", "catch", "class", "const", "continue", "debugger",
        "default", "delete", "do", "else", "export", "extends", "finally",
        "for", "function", "if", "import", "in", "instanceof", "let",
        "new", "return", "super", "switch", "this", "throw", "try",
        "typeof", "var", "void", "while", "with", "yield",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation simple pour mots-clés JavaScript.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs = [
        ("break", "Interrompt une boucle ou un switch."),
        ("case", "Cas dans une instruction switch."),
        ("catch", "Bloc pour gérer les exceptions."),
        ("class", "Déclare une classe."),
        ("const", "Déclare une constante."),
        ("continue", "Passe à l'itération suivante d'une boucle."),
        ("debugger", "Point d'arrêt pour le débogueur."),
        ("default", "Cas par défaut dans un switch."),
        ("delete", "Supprime une propriété d'un objet."),
        ("do", "Boucle do-while."),
        ("else", "Alternative dans une condition."),
        ("export", "Exporte des fonctions ou variables."),
        ("extends", "Hérite d'une classe."),
        ("finally", "Bloc qui s'exécute après try/catch."),
        ("for", "Boucle for."),
        ("function", "Déclare une fonction."),
        ("if", "Conditionnelle."),
        ("import", "Importe des modules."),
        ("in", "Teste l'appartenance à une propriété."),
        ("instanceof", "Teste l'appartenance à une classe."),
        ("let", "Déclare une variable locale."),
        ("new", "Crée une instance d'un objet."),
        ("return", "Retourne une valeur."),
        ("super", "Appelle le constructeur parent."),
        ("switch", "Sélecteur multi-cas."),
        ("this", "Référence à l'objet courant."),
        ("throw", "Lance une exception."),
        ("try", "Bloc d'essai."),
        ("typeof", "Retourne le type d'une variable."),
        ("var", "Déclare une variable."),
        ("void", "Évalue une expression et retourne undefined."),
        ("while", "Boucle while."),
        ("with", "Étend la portée d'une expression."),
        ("yield", "Pause et reprend une fonction génératrice."),
    ];

    for (k, v) in docs.iter() {
        if *k == keyword {
            return Some(*v);
        }
    }
    None
}

/// Snippet JavaScript minimal : Hello World
pub const JS_SNIPPET: &str = r#"console.log("Hello, world!");"#;

/// Interpréteur JavaScript basique : exécute via `node`.
/// Nécessite que `node` soit installé et dans le PATH.
pub fn interpret(code: &str) -> Result<String, String> {
    use std::fs::File;
    use std::io::Write;
    use std::env::temp_dir;
    use std::process::Output;

    // Créer un fichier temporaire
    let mut path = temp_dir();
    path.push("platon_temp.js");

    match File::create(&path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(code.as_bytes()) {
                return Err(format!("Erreur écriture fichier temporaire : {}", e));
            }
        }
        Err(e) => return Err(format!("Erreur création fichier temporaire : {}", e)),
    }

    // Exécuter node sur ce fichier
    let output: Output = match Command::new("node")
        .arg(&path)
        .output() {
            Ok(out) => out,
            Err(e) => return Err(format!("Erreur exécution node : {}", e)),
        };

    // Nettoyer fichier temporaire (ignorer erreur)
    let _ = std::fs::remove_file(&path);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Erreur exécution node : {}", stderr))
    }
}
