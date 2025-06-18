use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

/// Structure représentant l'état d'une session.
/// Ajouter ici les champs utiles à sauvegarder/restaurer.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Session {
    pub opened_files: Vec<String>,    // chemins des fichiers ouverts
    pub active_file: Option<String>, // fichier actif (curseur)
    pub cursor_positions: Vec<(String, usize)>, // tuple (fichier, position curseur)
    pub theme: Option<String>,        // thème actif
    pub config_path: Option<PathBuf>, // chemin fichier config utilisateur
    // autres champs à rajouter selon besoins...
}

impl Session {
    /// Crée une nouvelle session vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Charge une session depuis un fichier JSON.
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| format!("Erreur ouverture session: {}", e))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Erreur lecture session: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Erreur parse session JSON: {}", e))
    }

    /// Sauvegarde la session dans un fichier JSON.
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Erreur sérialisation session: {}", e))?;
        let mut file = File::create(path).map_err(|e| format!("Erreur création fichier: {}", e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Erreur écriture fichier: {}", e))
    }

    /// Ajoute un fichier à la liste des fichiers ouverts.
    pub fn open_file(&mut self, filename: &str) {
        if !self.opened_files.contains(&filename.to_string()) {
            self.opened_files.push(filename.to_string());
        }
        self.active_file = Some(filename.to_string());
    }

    /// Met à jour la position du curseur pour un fichier donné.
    pub fn set_cursor_position(&mut self, filename: &str, position: usize) {
        // remplace ou ajoute la position
        if let Some(pos) = self
            .cursor_positions
            .iter_mut()
            .find(|(f, _)| f == filename)
        {
            pos.1 = position;
        } else {
            self.cursor_positions.push((filename.to_string(), position));
        }
    }

    /// Récupère la position du curseur pour un fichier donné.
    pub fn get_cursor_position(&self, filename: &str) -> Option<usize> {
        self.cursor_positions
            .iter()
            .find(|(f, _)| f == filename)
            .map(|(_, pos)| *pos)
    }
}

// Fonctions utilitaires (optionnel)

/// Chemin par défaut du fichier de session
pub fn default_session_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("platon_editor")
        .join("session.json")
}

/// Charge la session ou crée une nouvelle si non trouvée.
pub fn load_or_new_session() -> Session {
    let path = default_session_path();
    if path.exists() {
        Session::load_from_file(path.to_str().unwrap()).unwrap_or_default()
    } else {
        Session::new()
    }
}

/// Sauvegarde la session à l’emplacement par défaut.
pub fn save_session(session: &Session) -> Result<(), String> {
    let path = default_session_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Erreur création dossier: {}", e))?;
    }
    session.save_to_file(path.to_str().unwrap())
}
