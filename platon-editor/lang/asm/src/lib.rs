//! Module d'autocomplétion, documentation, snippets et interprétation basique pour Assembleur (ASM).

use std::collections::HashMap;

/// Retourne une liste de suggestions de mots-clés ou instructions ASM pour l'autocomplétion.
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let keywords = vec![
        "mov", "add", "sub", "mul", "div", "jmp", "je", "jne", "jg", "jl", "jge", "jle",
        "push", "pop", "call", "ret", "nop", "cmp", "inc", "dec", "xor", "and", "or",
        "sal", "sar", "shl", "shr",
    ];

    keywords
        .into_iter()
        .filter(|kw| kw.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Retourne la documentation associée à un mot-clé ASM donné.
pub fn doc_for_keyword(keyword: &str) -> Option<&'static str> {
    let docs: HashMap<&str, &str> = [
        ("mov", "Déplace une valeur d'une source vers une destination."),
        ("add", "Additionne deux opérandes."),
        ("sub", "Soustrait deux opérandes."),
        ("mul", "Multiplie deux opérandes."),
        ("div", "Divise deux opérandes."),
        ("jmp", "Saut inconditionnel vers une étiquette."),
        ("je", "Saut si égal (ZF=1)."),
        ("jne", "Saut si différent (ZF=0)."),
        ("call", "Appelle une fonction/procédure."),
        ("ret", "Retourne d'une fonction/procédure."),
        ("push", "Empile une valeur sur la pile."),
        ("pop", "Dépile une valeur de la pile."),
        ("nop", "Aucune opération (no-op)."),
        ("cmp", "Compare deux opérandes."),
        ("inc", "Incrémente une valeur."),
        ("dec", "Décrémente une valeur."),
        ("xor", "Opération OU exclusif."),
        ("and", "Opération ET binaire."),
        ("or", "Opération OU binaire."),
        ("sal", "Décalage logique à gauche."),
        ("sar", "Décalage arithmétique à droite."),
        ("shl", "Décalage logique à gauche."),
        ("shr", "Décalage logique à droite."),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(keyword).copied()
}

/// Exemple de snippet assembleur
pub const ASM_SNIPPET: &str = r#"; Exemple de fonction simple en assembleur
section .text
global _start

_start:
    mov eax, 1      ; syscall: sys_exit
    mov ebx, 0      ; status 0
    int 0x80        ; appel système
"#;


/// Interprétation basique : actuellement non supportée pour ASM.
/// Renvoie une erreur.
pub fn interpret(_code: &str) -> Result<String, String> {
    Err("Interprétation directe du code assembleur non supportée.".to_string())
}
