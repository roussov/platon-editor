use std::fmt;
use std::io;

#[derive(Debug)]
pub enum PlatonError {
    Io(io::Error),
    FileNotFound(String),
    PermissionDenied(String),
    ParseError(String),
    UnsupportedLanguage(String),
    InterpreterError(String),
    InvalidCommand(String),
    ConfigError(String),
    BufferOverflow,
    Internal(String),
}

impl fmt::Display for PlatonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatonError::Io(e) => write!(f, "Erreur IO : {}", e),
            PlatonError::FileNotFound(p) => write!(f, "Fichier introuvable : {}", p),
            PlatonError::PermissionDenied(p) => write!(f, "Permission refusée : {}", p),
            PlatonError::ParseError(e) => write!(f, "Erreur de parsing : {}", e),
            PlatonError::UnsupportedLanguage(lang) => {
                write!(f, "Langage non supporté : {}", lang)
            }
            PlatonError::InterpreterError(e) => write!(f, "Erreur de l'interpréteur : {}", e),
            PlatonError::InvalidCommand(cmd) => write!(f, "Commande invalide : {}", cmd),
            PlatonError::ConfigError(e) => write!(f, "Erreur de configuration : {}", e),
            PlatonError::BufferOverflow => write!(f, "Dépassement de tampon détecté"),
            PlatonError::Internal(e) => write!(f, "Erreur interne : {}", e),
        }
    }
}

impl std::error::Error for PlatonError {}

impl From<io::Error> for PlatonError {
    fn from(err: io::Error) -> Self {
        PlatonError::Io(err)
    }
}
