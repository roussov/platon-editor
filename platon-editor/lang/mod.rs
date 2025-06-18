// src/lang/mod.rs

pub mod platon;
pub mod python;
pub mod javascript;
pub mod typescript;
pub mod c;
pub mod cpp;
pub mod java;
pub mod go;
pub mod ruby;
pub mod swift;
pub mod kotlin;
pub mod php;
pub mod bash;
pub mod lua;
pub mod scala;
pub mod fortran;
pub mod basic;
pub mod asm;
pub mod brainfuck;
pub mod ook;
pub mod markdown;

use std::collections::HashMap;

/// Retourne la liste de suggestions selon le langage et le préfixe.
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        "rust" => rust::optimize_code(code),
        "python" => python::optimize_code(code),
        "javascript" => javascript::optimize_code(code),
        "typescript" => typescript::optimize_code(code),
        "c" => c::optimize_code(code),
        "cpp" | "c++" => cpp::optimize_code(code),
        "java" => java::optimize_code(code),
        "go" => go::optimize_code(code),
        "ruby" => ruby::optimize_code(code),
        "swift" => swift::optimize_code(code),
        "kotlin" => kotlin::optimize_code(code),
        "php" => php::optimize_code(code),
        "bash" => bash::optimize_code(code),
        "lua" => lua::optimize_code(code),
        "scala" => scala::optimize_code(code),
        "fortran" => fortran::optimize_code(code),
        "basic" => basic::optimize_code(code),
        "asm" | "assembly" => asm::optimize_code(code),
        "brainfuck" => brainfuck::optimize_code(code),
        "ook" => ook::optimize_code(code),
        "markdown" | "md" => markdown::optimize_code(code),
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}

/// Retourne un snippet exemple stylisé si disponible.
pub fn snippet(lang: &str) -> Option<&'static str> {
    match lang.to_lowercase().as_str() {
        "platon" => Some(platon::RUST_SNIPPET),
        "python" => Some(python::PYTHON_SNIPPET),
        "javascript" => Some(javascript::JS_SNIPPET),
        "typescript" => Some(typescript::TS_SNIPPET),
        "c" => Some(c::C_SNIPPET),
        "cpp" | "c++" => Some(cpp::CPP_SNIPPET),
        "java" => Some(java::JAVA_SNIPPET),
        "go" => Some(go::GO_SNIPPET),
        "ruby" => Some(ruby::RUBY_SNIPPET),
        "swift" => Some(swift::SWIFT_SNIPPET),
        "kotlin" => Some(kotlin::KOTLIN_SNIPPET),
        "php" => Some(php::PHP_SNIPPET),
        "bash" => Some(bash::BASH_SNIPPET),
        "lua" => Some(lua::LUA_SNIPPET),
        "scala" => Some(scala::SCALA_SNIPPET),
        "fortran" => Some(fortran::FORTRAN_SNIPPET),
        "basic" => Some(basic::BASIC_SNIPPET),
        "asm" | "assembly" => Some(asm::ASM_SNIPPET),
        "brainfuck" => Some(brainfuck::BRAINFUCK_SNIPPET),
        "ook" => Some(ook::OOK_SNIPPET),
        "markdown" | "md" => Some(markdown::MARKDOWN_SNIPPET),
        _ => None,
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "scala" => scala::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "javascript" | "js" => javascript::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "asm" | "assembly" => asm::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "asm" | "assembly" => asm::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "cpp" | "c++" => cpp::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ... autres langages ...
        "c" => c::optimize_code(code),
        // ... autres langages ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "lua" => lua::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "swift" => swift::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "markdown" | "md" => markdown::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "typescript" | "ts" => typescript::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
pub fn optimize_code(lang: &str, code: &str) -> Result<String, String> {
    match lang.to_lowercase().as_str() {
        // ...
        "typescript" | "ts" => typescript::optimize_code(code),
        // ...
        _ => Err("Optimisation non supportée pour ce langage".to_string()),
    }
}
