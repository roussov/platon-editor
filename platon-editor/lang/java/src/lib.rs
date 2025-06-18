//! Module Java pour l'éditeur Platon
//! Autocomplétion, documentation, snippets et interprétation via `javac` et `java`.

use std::process::{Command, Stdio};
use std::io::Write;

/// Mots-clés Java pour l'autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "abstract", "assert", "boolean", "break", "byte", "case", "catch",
        "char", "class", "const", "continue", "default", "do", "double",
        "else", "enum", "extends", "final", "finally", "float", "for",
        "goto", "if", "implements", "import", "instanceof", "int",
        "interface", "long", "native", "new", "package", "private",
        "protected", "public", "return", "short", "static", "strictfp",
        "super", "switch", "synchronized", "this", "throw", "throws",
        "transient", "try", "void", "volatile", "while",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation simple pour mots-clés Java.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs = [
        ("abstract", "Déclare une classe ou méthode abstraite."),
        ("assert", "Fait une assertion."),
        ("boolean", "Type booléen."),
        ("break", "Sort d'une boucle ou switch."),
        ("byte", "Type entier 8 bits."),
        ("case", "Cas dans switch."),
        ("catch", "Capture une exception."),
        ("char", "Type caractère."),
        ("class", "Déclare une classe."),
        ("const", "Mot réservé non utilisé."),
        ("continue", "Passe à l'itération suivante."),
        ("default", "Cas par défaut dans switch."),
        ("do", "Boucle do-while."),
        ("double", "Type réel 64 bits."),
        ("else", "Alternative conditionnelle."),
        ("enum", "Déclare une énumération."),
        ("extends", "Indique l'héritage."),
        ("final", "Déclare une constante ou méthode finale."),
        ("finally", "Bloc d'exécution après try-catch."),
        ("float", "Type réel 32 bits."),
        ("for", "Boucle for."),
        ("goto", "Mot réservé non utilisé."),
        ("if", "Conditionnelle."),
        ("implements", "Implémente une interface."),
        ("import", "Importe un paquet."),
        ("instanceof", "Test de type."),
        ("int", "Type entier 32 bits."),
        ("interface", "Déclare une interface."),
        ("long", "Type entier 64 bits."),
        ("native", "Méthode native."),
        ("new", "Création d'objet."),
        ("package", "Déclare un paquet."),
        ("private", "Visibilité privée."),
        ("protected", "Visibilité protégée."),
        ("public", "Visibilité publique."),
        ("return", "Retourne une valeur."),
        ("short", "Type entier 16 bits."),
        ("static", "Membre statique."),
        ("strictfp", "Précision flottante stricte."),
        ("super", "Référence à la classe parente."),
        ("switch", "Sélecteur multi-cas."),
        ("synchronized", "Synchronisation."),
        ("this", "Référence à l'objet courant."),
        ("throw", "Lance une exception."),
        ("throws", "Déclare une exception possible."),
        ("transient", "Non sérialisable."),
        ("try", "Bloc d'exception."),
        ("void", "Type retour vide."),
        ("volatile", "Variable volatile."),
        ("while", "Boucle while."),
    ];

    for (k, v) in docs.iter() {
        if *k == keyword {
            return Some(*v);
        }
    }
    None
}

/// Snippet Java minimal : Hello World
pub const JAVA_SNIPPET: &str = r#"public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
"#;

/// Interpréteur Java basique : compile et exécute via `javac` puis `java`.
/// Nécessite que `javac` et `java` soient installés et dans le PATH.
pub fn interpret(code: &str) -> Result<String, String> {
    use std::fs::{File, remove_file};
    use std::io::Write;
    use std::env::temp_dir;
    use std::path::PathBuf;
    use std::process::Output;

    let tmp_dir = temp_dir();
    let source_path = tmp_dir.join("HelloWorld.java");

    // Écrire le code dans HelloWorld.java
    match File::create(&source_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(code.as_bytes()) {
                return Err(format!("Erreur écriture fichier source : {}", e));
            }
        }
        Err(e) => return Err(format!("Erreur création fichier source : {}", e)),
    }

    // Compiler avec javac
    let compile_output = Command::new("javac")
        .arg(&source_path)
        .output()
        .map_err(|e| format!("Erreur lancement javac : {}", e))?;

    if !compile_output.status.success() {
        let err = String::from_utf8_lossy(&compile_output.stderr);
        let err_msg = if err.is_empty() {
            "Erreur inconnue à la compilation".to_string()
        } else {
            err.to_string()
        };
        // Nettoyer fichier source
        let _ = remove_file(&source_path);
        return Err(format!("Erreur compilation : {}", err_msg));
    }

    // Exécuter la classe compilée
    let run_output = Command::new("java")
        .current_dir(&tmp_dir)
        .arg("HelloWorld")
        .output()
        .map_err(|e| format!("Erreur lancement java : {}", e))?;

    // Nettoyer fichiers générés
    let _ = remove_file(&source_path);
    let _ = remove_file(tmp_dir.join("HelloWorld.class"));

    if run_output.status.success() {
        Ok(String::from_utf8_lossy(&run_output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&run_output.stderr).to_string())
    }
}
