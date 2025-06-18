use std::process::{Command, Stdio};

const KEYWORDS: &[&str] = &[
    "def", "val", "var", "class", "object", "trait", "extends", "with", "if", "else",
    "match", "case", "while", "for", "yield", "do", "try", "catch", "finally", "import",
    "package", "private", "protected", "override", "implicit", "final", "abstract", "lazy",
    "new", "this", "super", "return",
];

pub const SCALA_SNIPPET: &str = r#"object HelloPlaton {
  def main(args: Array[String]): Unit = {
    println("Hello, Platon!")
  }
}"#;

/// Autocomplétion pour Scala
pub fn autocomplete(prefix: &str) -> Vec<String> {
    KEYWORDS
        .iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation de base pour Scala
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    match keyword {
        "def" => Some("def — Déclare une fonction."),
        "val" => Some("val — Variable immuable."),
        "var" => Some("var — Variable mutable."),
        "object" => Some("object — Singleton."),
        "match" => Some("match — Pattern matching."),
        "trait" => Some("trait — Interface comportant des implémentations par défaut."),
        _ => None,
    }
}

/// Interprétation du code Scala via `scala`
pub fn interpret(code: &str) -> Result<String, String> {
    let mut process = match Command::new("scala")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(proc) => proc,
        Err(e) => return Err(format!("Erreur scala : {}", e)),
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
