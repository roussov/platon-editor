use std::process::{Command, Stdio};

const KEYWORDS: &[&str] = &[
    "let", "var", "func", "struct", "class", "enum", "protocol", "extension",
    "if", "else", "switch", "case", "default", "for", "in", "while", "repeat", "do",
    "return", "guard", "defer", "throw", "throws", "try", "catch", "import",
    "public", "private", "internal", "fileprivate", "open", "static", "lazy",
    "final", "override", "self", "super", "init",
];

pub const SWIFT_SNIPPET: &str = r#"import Foundation

print("Hello, Platon!")
"#;

/// Autocomplétion pour Swift
pub fn autocomplete(prefix: &str) -> Vec<String> {
    KEYWORDS
        .iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation pour mots-clés Swift
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    match keyword {
        "let" => Some("let — Déclare une constante."),
        "var" => Some("var — Déclare une variable."),
        "func" => Some("func — Déclare une fonction."),
        "struct" => Some("struct — Définit une structure."),
        "class" => Some("class — Définit une classe."),
        "enum" => Some("enum — Définit une énumération."),
        "protocol" => Some("protocol — Définit une interface de comportement."),
        "if" => Some("if — Conditionnelle."),
        "guard" => Some("guard — Conditionnelle précoce."),
        _ => None,
    }
}

/// Interprétation de code Swift via `swift`
pub fn interpret(code: &str) -> Result<String, String> {
    let mut process = match Command::new("swift")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(proc) => proc,
        Err(e) => return Err(format!("Erreur swift : {}", e)),
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
