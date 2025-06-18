/// Module d’optimisation du code source par langage.
///
/// Les fonctions prennent une chaîne de caractères avec le code source
/// et retournent une chaîne optimisée.

pub mod optimize {
    /// Optimise du code C (exemple simple).
    pub fn optimize_c(code: &str) -> String {
        // Enlever commentaires //... et /* ... */
        let mut output = String::new();
        let mut in_block_comment = false;
        let mut in_line_comment = false;

        let mut chars = code.chars().peekable();

        while let Some(c) = chars.next() {
            if in_block_comment {
                if c == '*' {
                    if let Some(&'/') = chars.peek() {
                        chars.next();
                        in_block_comment = false;
                    }
                }
                continue;
            } else if in_line_comment {
                if c == '\n' {
                    in_line_comment = false;
                    output.push(c);
                }
                continue;
            } else if c == '/' {
                if let Some(&next_c) = chars.peek() {
                    if next_c == '/' {
                        in_line_comment = true;
                        chars.next();
                        continue;
                    } else if next_c == '*' {
                        in_block_comment = true;
                        chars.next();
                        continue;
                    }
                }
            }
            output.push(c);
        }

        // Supprimer lignes vides
        let output: String = output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n");

        output
    }

    /// Optimise du code JavaScript (exemple simple).
    pub fn optimize_javascript(code: &str) -> String {
        // Suppression des commentaires // et /* ... */
        // et suppression des lignes vides

        // Réutilisation de la fonction optimize_c car la logique est similaire
        optimize_c(code)
    }

    /// Optimise du code assembleur (ASM) simple.
    pub fn optimize_asm(code: &str) -> String {
        // Suppression des lignes vides et espaces superflus
        code
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with(';')) // ; est commentaire ASM
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Fonction publique pour optimiser selon langage.
    pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
        match lang.to_lowercase().as_str() {
            "c" => Ok(optimize_c(code)),
            "javascript" | "js" => Ok(optimize_javascript(code)),
            "asm" | "assembly" => Ok(optimize_asm(code)),
            _ => Err(format!("Optimisation non supportée pour le langage '{}'", lang)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::optimize::*;

    #[test]
    fn test_optimize_c_removes_comments_and_blank_lines() {
        let code = r#"
        // Commentaire ligne
        int main() {
            /* commentaire bloc */
            return 0; // fin
        }

        "#;
        let optimized = optimize_c(code);
        assert!(!optimized.contains("//"));
        assert!(!optimized.contains("/*"));
        assert!(!optimized.contains("*/"));
        assert!(optimized.contains("int main()"));
        assert!(optimized.contains("return 0;"));
    }

    #[test]
    fn test_optimize_asm_removes_blank_and_comment_lines() {
        let code = r#"
            ; commentaire asm
            mov eax, 1

            nop

        "#;
        let optimized = optimize_asm(code);
        assert!(!optimized.contains("; commentaire"));
        assert!(optimized.contains("mov eax, 1"));
        assert!(optimized.contains("nop"));
    }
}
