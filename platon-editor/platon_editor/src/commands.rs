use crate::editor::Editor;
use crossterm::event::KeyEvent;
use std::io;

pub struct PlatonCommands {}

impl PlatonCommands {
    pub fn new() -> Self {
        Self {}
    }

    /// Traite une touche et modifie l’éditeur.
    /// Retourne true pour quitter le programme.
    pub fn handle_input(&mut self, key: &KeyEvent, editor: &mut Editor) -> io::Result<bool> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key.code {
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Ctrl+Q pour quitter
                return Ok(true);
            }
            KeyCode::Char(ch) => {
                editor.insert_char(ch);
            }
            KeyCode::Backspace => {
                editor.delete_char();
            }
            KeyCode::Enter => {
                editor.insert_char('\n');
            }
            _ => {}
        }

        Ok(false)
    }
}
