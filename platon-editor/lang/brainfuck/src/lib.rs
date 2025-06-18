//! Module Brainfuck pour l'éditeur Platon
//! Autocomplétion, documentation, snippets, interprétation simple et visuelle.

use std::collections::HashMap;

/// Autocomplétion Brainfuck (les 8 commandes)
pub fn autocomplete(prefix: &str) -> Vec<String> {
    let commands = vec!["+", "-", "<", ">", "[", "]", ".", ","];
    commands
        .into_iter()
        .filter(|cmd| cmd.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// Documentation pour chaque commande Brainfuck.
pub fn doc_for_command(command: &str) -> Option<&'static str> {
    let docs: HashMap<&str, &str> = [
        ("+", "Incrémente la cellule mémoire sous le pointeur."),
        ("-", "Décrémente la cellule mémoire sous le pointeur."),
        (">", "Déplace le pointeur mémoire vers la droite."),
        ("<", "Déplace le pointeur mémoire vers la gauche."),
        ("[", "Début d'une boucle (tant que la cellule sous le pointeur n'est pas zéro)."),
        ("]", "Fin d'une boucle."),
        (".", "Sortie du caractère correspondant à la cellule sous le pointeur."),
        (",", "Entrée d'un caractère dans la cellule sous le pointeur."),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(command).copied()
}

/// Snippet Brainfuck exemple (Hello World)
pub const BRAINFUCK_SNIPPET: &str = r#"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."#;

/// Interpréteur Brainfuck simple
/// Supporte l'entrée et la sortie via les chaînes.
pub fn interpret(code: &str) -> Result<String, String> {
    let code_chars: Vec<char> = code.chars().filter(|c| "+-<>[],.".contains(*c)).collect();
    let mut memory = vec![0u8; 30000];
    let mut ptr = 0usize;
    let mut pc = 0usize; // programme counter
    let mut output = String::new();

    // Préparer un mapping de sauts pour [ et ]
    let mut bracket_map = std::collections::HashMap::new();
    let mut stack = Vec::new();

    for (i, &c) in code_chars.iter().enumerate() {
        if c == '[' {
            stack.push(i);
        } else if c == ']' {
            if let Some(open_pos) = stack.pop() {
                bracket_map.insert(open_pos, i);
                bracket_map.insert(i, open_pos);
            } else {
                return Err("Erreur : crochet ] sans correspondant [".to_string());
            }
        }
    }
    if !stack.is_empty() {
        return Err("Erreur : crochet [ sans correspondant ]".to_string());
    }

    while pc < code_chars.len() {
        match code_chars[pc] {
            '+' => {
                memory[ptr] = memory[ptr].wrapping_add(1);
            }
            '-' => {
                memory[ptr] = memory[ptr].wrapping_sub(1);
            }
            '>' => {
                ptr += 1;
                if ptr >= memory.len() {
                    return Err("Erreur : pointeur mémoire dépasse la limite droite".to_string());
                }
            }
            '<' => {
                if ptr == 0 {
                    return Err("Erreur : pointeur mémoire dépasse la limite gauche".to_string());
                }
                ptr -= 1;
            }
            '[' => {
                if memory[ptr] == 0 {
                    // sauter à la commande après le ]
                    pc = *bracket_map.get(&pc).ok_or("Erreur dans la correspondance des crochets")?;
                }
            }
            ']' => {
                if memory[ptr] != 0 {
                    // retourner à la commande après le [
                    pc = *bracket_map.get(&pc).ok_or("Erreur dans la correspondance des crochets")?;
                }
            }
            '.' => {
                output.push(memory[ptr] as char);
            }
            ',' => {
                // Entrée non supportée (ou on peut simuler)
                // Pour l'exemple on met 0
                memory[ptr] = 0;
            }
            _ => {}
        }
        pc += 1;
    }

    Ok(output)
}
