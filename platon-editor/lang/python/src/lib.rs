use std::process::{Command, Stdio};

const KEYWORDS: &[&str] = &[
    "and", "as", "assert", "break", "class", "continue", "def", "del", "elif", "else",
    "except", "False", "finally", "for", "from", "global", "if", "import", "in", "is",
    "lambda", "None", "nonlocal", "not", "or", "pass", "raise", "return", "True", "try",
    "while", "with", "yield", "print", "input", "len", "range"
];

pub const PYTHON_SNIPPET: &str = r#"def hello():
    print("Hello, Platon!")

hello()"#;

/// Retourne les suggestions de mots-clés Python correspondant au préfixe.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    KEYWORDS
        .iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Fournit une documentation de base pour un mot-clé donné.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    match keyword {
        "def" => Some("def — Définit une fonction."),
        "class" => Some("class — Définit une classe."),
        "if" => Some("if — Conditionnelle."),
        "for" => Some("for — Boucle sur un itérable."),
        "print" => Some("print — Affiche des valeurs."),
        "import" => Some("import — Importe un module."),
        "input" => Some("input — Lit une entrée utilisateur."),
        "lambda" => Some("lambda — Fonction anonyme."),
        "try" => Some("try — Bloc de capture d'erreur."),
        _ => None,
    }
}

/// Interprète du code Python via l'exécutable python3.
/// Retourne stdout ou stderr.
pub fn interpret(code: &str) -> Result<String, String> {
    let mut process = match Command::new("python3")
        .arg("-c")
        .arg(code)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(proc) => proc,
        Err(e) => return Err(format!("Erreur exécution Python: {}", e)),
    };

    let output = process.wait_with_output().map_err(|e| e.to_string())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| e.to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
