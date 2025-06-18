//! Module Hook pour l'éditeur Platon
//! Permet d'enregistrer et d'exécuter des hooks personnalisés.

use std::collections::HashMap;

type HookFn = Box<dyn Fn(&str) -> String + Send + Sync>;

/// Gestionnaire de hooks
pub struct HookManager {
    hooks: HashMap<String, Vec<HookFn>>,
}

impl HookManager {
    /// Crée un nouveau gestionnaire de hooks.
    pub fn new() -> Self {
        HookManager {
            hooks: HashMap::new(),
        }
    }

    /// Enregistre un hook sur un événement donné.
    pub fn register_hook<F>(&mut self, event: &str, callback: F)
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        self.hooks
            .entry(event.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    /// Exécute tous les hooks liés à un événement avec une donnée en entrée.
    /// Renvoie la concaténation des résultats.
    pub fn run_hooks(&self, event: &str, data: &str) -> Option<String> {
        self.hooks.get(event).map(|callbacks| {
            callbacks.iter().map(|f| f(data)).collect::<Vec<_>>().join("\n")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hooks() {
        let mut manager = HookManager::new();

        manager.register_hook("on_save", |data| format!("Save hook 1: {}", data));
        manager.register_hook("on_save", |data| format!("Save hook 2: {}", data));
        manager.register_hook("on_exit", |data| format!("Exit hook: {}", data));

        let result = manager.run_hooks("on_save", "code.rs");
        assert!(result.is_some());
        let output = result.unwrap();
        assert!(output.contains("Save hook 1: code.rs"));
        assert!(output.contains("Save hook 2: code.rs"));

        let exit_result = manager.run_hooks("on_exit", "bye");
        assert_eq!(exit_result.unwrap(), "Exit hook: bye");

        let none_result = manager.run_hooks("nonexistent", "data");
        assert!(none_result.is_none());
    }
}
