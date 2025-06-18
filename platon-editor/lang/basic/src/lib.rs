//! Module Basic pour l'éditeur Platon
//! Autocomplétion, documentation, snippets, interprétation simple.

use std::collections::HashMap;

/// Liste des mots-clés BASIC pour autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "PRINT", "LET", "IF", "THEN", "ELSE", "FOR", "TO", "STEP", "NEXT",
        "GOTO", "GOSUB", "RETURN", "END", "INPUT", "REM",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(&prefix.to_uppercase()))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation des mots-clés BASIC.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs: HashMap<&str, &str> = [
        ("PRINT", "Affiche du texte ou des variables à l'écran."),
        ("LET", "Assigne une valeur à une variable."),
        ("IF", "Instruction conditionnelle."),
        ("THEN", "Bloc exécuté si condition vraie."),
        ("ELSE", "Bloc alternatif."),
        ("FOR", "Début d'une boucle for."),
        ("TO", "Définir la limite supérieure d'une boucle."),
        ("STEP", "Définir l'incrément dans une boucle."),
        ("NEXT", "Fin d'une boucle for."),
        ("GOTO", "Saut inconditionnel vers une ligne."),
        ("GOSUB", "Appel d'une sous-routine."),
        ("RETURN", "Retour d'une sous-routine."),
        ("END", "Fin du programme."),
        ("INPUT", "Lire une entrée utilisateur."),
        ("REM", "Commentaire."),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(&keyword.to_uppercase()[..]).copied()
}

/// Snippet d'exemple BASIC
pub const BASIC_SNIPPET: &str = r#"10 REM Exemple simple en BASIC
20 INPUT "Entrez un nombre : ", N
30 IF N < 10 THEN
40     PRINT "Petit nombre"
50 ELSE
60     PRINT "Grand nombre"
70 END IF
80 END
"#;

/// Interprétation basique d’un sous-ensemble de BASIC.
/// Limité à quelques instructions simples.
pub fn interpret(code: &str) -> Result<String, String> {
    // Pour simplifier, on fait un interpréteur très basique, ligne par ligne
    // Supporte uniquement PRINT et INPUT simulé, LET simple, IF condition simple

    let mut variables = std::collections::HashMap::new();
    let mut output = String::new();

    for line in code.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("REM") {
            continue;
        }

        // Parse ligne : on supprime le numéro de ligne s’il est présent
        let content = if let Some(pos) = line.find(' ') {
            &line[pos + 1..]
        } else {
            line
        };

        if content.to_uppercase().starts_with("PRINT") {
            let to_print = content[5..].trim();
            // Support print variable ou texte brut entre guillemets
            if to_print.starts_with("\"") && to_print.ends_with("\"") {
                output.push_str(&to_print[1..to_print.len() -1]);
                output.push('\n');
            } else {
                let val = variables.get(to_print).unwrap_or(&"".to_string());
                output.push_str(val);
                output.push('\n');
            }
        } else if content.to_uppercase().starts_with("LET") {
            // Ex: LET X=10
            if let Some(eq_pos) = content.find('=') {
                let var = content[3..eq_pos].trim();
                let val = content[eq_pos+1..].trim();
                variables.insert(var.to_string(), val.to_string());
            }
        } else if content.to_uppercase().starts_with("INPUT") {
            // Pour l'exemple, on simule l'entrée par une valeur fixe
            let parts: Vec<&str> = content[5..].trim().split(',').collect();
            if parts.len() == 1 {
                variables.insert(parts[0].trim().to_string(), "42".to_string());
            }
        } else if content.to_uppercase().starts_with("END") {
            break;
        }
        // Pour simplifier, pas de support IF/NEXT/GOTO dans cette version basique
    }

    Ok(output)
}
