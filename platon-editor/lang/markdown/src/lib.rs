//! Module Markdown pour l'éditeur Platon
//! Autocomplétion, documentation, snippets et rendu Markdown avec pulldown-cmark.

use pulldown_cmark::{Parser, Options, html, Event, Tag};

/// Éléments Markdown courants pour autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let elements = vec![
        "#", "##", "###", "####", "#####", "######", // titres
        "-", "*", "+", // listes non ordonnées
        "1.", "2.", "3.", // listes ordonnées
        "`", "```", // code inline, bloc code
        ">", // citation
        "[", "]", "(", ")", // liens/images
        "**", "__", // gras
        "*", "_", // italique
        "---", "***", "___", // séparateurs horizontaux
        "![]()", // image syntaxe complète
    ];

    elements
        .into_iter()
        .filter(|el| el.starts_with(prefix))
        .map(String::from)
        .collect()
}

/// Documentation simple pour éléments Markdown.
pub fn doc_for_element(element: &str) -> Option<&'static str> {
    let docs = [
        ("#", "Titre de niveau 1."),
        ("##", "Titre de niveau 2."),
        ("###", "Titre de niveau 3."),
        ("-", "Liste non ordonnée."),
        ("*", "Liste non ordonnée ou italique."),
        ("+", "Liste non ordonnée."),
        ("1.", "Liste ordonnée."),
        ("`", "Code inline."),
        ("```", "Bloc de code."),
        (">", "Citation."),
        ("[", "Début d'un lien."),
        ("]", "Fin d'un lien."),
        ("(", "Début d'une URL."),
        (")", "Fin d'une URL."),
        ("**", "Texte en gras."),
        ("__", "Texte en gras."),
        ("_", "Italique."),
        ("---", "Séparateur horizontal."),
        ("***", "Séparateur horizontal."),
        ("![]()", "Syntaxe image."),
    ];

    for (k, v) in docs.iter() {
        if *k == element {
            return Some(*v);
        }
    }
    None
}

/// Snippet Markdown minimal
pub const MARKDOWN_SNIPPET: &str = r#"# Titre 1

Paragraphe *italique* **gras**.

- Liste à puces
- Item 2

`code inline`

```rust
// Bloc de code
println!("Hello, Markdown!");
