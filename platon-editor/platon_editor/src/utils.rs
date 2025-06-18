use std::path::{Path, PathBuf};

/// Tronque une ligne trop longue pour le terminal.
pub fn truncate_line(line: &str, max_width: usize) -> String {
    if line.chars().count() > max_width {
        let truncated: String = line.chars().take(max_width.saturating_sub(3)).collect();
        format!("{truncated}...")
    } else {
        line.to_string()
    }
}

/// Retourne l’extension d’un fichier, en minuscule.
pub fn file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// Retourne le nom du fichier (sans le chemin).
pub fn file_name(path: &Path) -> Option<String> {
    path.file_name().and_then(|name| name.to_str()).map(|s| s.to_string())
}

/// Vérifie si le chemin est un fichier avec une extension donnée.
pub fn is_file_with_ext(path: &Path, ext: &str) -> bool {
    path.is_file()
        && file_extension(path)
            .map(|e| e == ext.to_lowercase())
            .unwrap_or(false)
}

/// Convertit un booléen en ✅ ou ❌.
pub fn bool_icon(value: bool) -> &'static str {
    if value {
        "✅"
    } else {
        "❌"
    }
}

/// Convertit une valeur booléenne en "oui"/"non"
pub fn bool_str(value: bool) -> &'static str {
    if value {
        "oui"
    } else {
        "non"
    }
}

/// Normalise un chemin de manière portable.
pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    dunce::canonicalize(path).unwrap_or_else(|_| PathBuf::from("."))
}

/// Limite une chaîne à n caractères, avec suffixe si trop longue.
pub fn shorten(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max.saturating_sub(1)])
    } else {
        s.to_string()
    }
}
