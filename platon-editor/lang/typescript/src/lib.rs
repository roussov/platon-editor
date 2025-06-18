use std::process::{Command, Stdio};

const KEYWORDS: &[&str] = &[
    "function", "let", "const", "var", "class", "interface", "extends", "implements",
    "enum", "type", "as", "import", "export", "from", "if", "else", "for", "while",
    "switch", "case", "default", "break", "continue", "return", "try", "catch", "finally",
    "throw", "new", "this", "super", "public", "private", "protected", "static", "readonly",
    "async", "await", "typeof", "instanceof", "in", "keyof", "infer", "never", "unknown", "void",
];

pub const TS_SNIPPET: &str = r#"function greet(name: string): void {
  console.log(`Hello, ${name}!`);
}

greet("Platon");
"#;

/// Autocomplétion des mots-clés TypeScript
pub fn autocomplete(prefix: &str) -> Vec<String> {
    KEYWORDS
        .iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation simple pour les mots-clés TypeScript
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    match keyword {
        "function" => Some("function — Déclare une fonction."),
        "let" => Some("let — Déclare une variable à portée de bloc."),
        "const" => Some("const — Déclare une constante."),
        "interface" => Some("interface — Définit un contrat pour un objet."),
        "type" => Some("type — Alias de type."),
        "async" => Some("async — Fonction asynchrone."),
        "await" => Some("await — Attend une promesse."),
        "extends" => Some("extends — Héritage de classe ou interface."),
        _ => None,
    }
}

/// Interprétation TypeScript via `ts-node`
pub fn interpret(code: &str) -> Result<String, String> {
    let mut process = match Command::new("ts-node")
        .arg("-e")
        .arg(code)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(proc) => proc,
        Err(e) => return Err(format!("Erreur ts-node : {}", e)),
    };

    let output = process.wait_with_output().map_err(|e| e.to_string())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| e.to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Optimisation TypeScript — ici un placeholder (minification possible dans le futur)
pub fn optimize(code: &str) -> String {
    // Exemple simple : retirer les commentaires
    code.lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}
