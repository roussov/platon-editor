use std::process::{Command, Stdio};

const KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "struct", "enum", "trait", "impl", "for", "match", "if", "else",
    "loop", "while", "break", "continue", "return", "mod", "use", "pub", "const", "static",
    "ref", "as", "in", "crate", "super", "Self", "self", "dyn", "where", "async", "await",
];

pub const RUST_SNIPPET: &str = r#"fn main() {
    println!("Hello, Platon!");
}"#;

/// Autocomplétion Rust
pub fn autocomplete(prefix: &str) -> Vec<String> {
    KEYWORDS
        .iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation de base pour Rust
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    match keyword {
        "fn" => Some("fn — Déclare une fonction."),
        "let" => Some("let — Déclare une variable."),
        "struct" => Some("struct — Définit une structure."),
        "impl" => Some("impl — Implémente des méthodes."),
        "match" => Some("match — Branches conditionnelles puissantes."),
        "trait" => Some("trait — Définit un comportement."),
        "async" => Some("async — Fonction ou bloc asynchrone."),
        _ => None,
    }
}

/// Interprétation Rust (avec rust-script si installé)
pub fn interpret(code: &str) -> Result<String, String> {
    let mut process = match Command::new("rust-script")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(proc) => proc,
        Err(e) => return Err(format!("Erreur rust-script : {}", e)),
    };

    if let Some(mut stdin) = process.stdin.take() {
        use std::io::Write;
        if let Err(e) = write!(stdin, "{}", code) {
            return Err(format!("Échec écriture stdin : {}", e));
        }
    }

    let output = process.wait_with_output().map_err(|e| e.to_string())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| e.to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
