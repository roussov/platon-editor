use std::process::{Command, Stdio};

const KEYWORDS: &[&str] = &[
    "def", "class", "module", "if", "elsif", "else", "unless", "case", "when", "while",
    "until", "for", "in", "do", "end", "begin", "rescue", "ensure", "yield", "return",
    "self", "nil", "true", "false", "and", "or", "not", "alias", "super", "puts", "gets",
];

pub const RUBY_SNIPPET: &str = r#"def hello
  puts "Hello, Platon!"
end

hello"#;

/// Suggestions automatiques pour Ruby
pub fn autocomplete(prefix: &str) -> Vec<String> {
    KEYWORDS
        .iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation de base pour certains mots-clés Ruby
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    match keyword {
        "def" => Some("def — Définit une méthode."),
        "class" => Some("class — Définit une classe."),
        "if" => Some("if — Structure conditionnelle."),
        "puts" => Some("puts — Affiche une chaîne de caractères."),
        "gets" => Some("gets — Lit une entrée de l'utilisateur."),
        "yield" => Some("yield — Invoque un bloc passé à une méthode."),
        _ => None,
    }
}

/// Interprète du code Ruby via l'exécutable ruby
pub fn interpret(code: &str) -> Result<String, String> {
    let mut process = match Command::new("ruby")
        .arg("-e")
        .arg(code)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(proc) => proc,
        Err(e) => return Err(format!("Erreur d'exécution Ruby : {}", e)),
    };

    let output = process.wait_with_output().map_err(|e| e.to_string())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| e.to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
