use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Debug)]
pub enum InputEvent {
    InsertChar(char),
    Backspace,
    Delete,
    Enter,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Save,
    Quit,
    Command(String),
    None,
}

#[derive(Default, Debug)]
pub struct EditorInput {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub buffer: VecDeque<String>,
    pub command_mode: bool,
    pub command_buffer: String,
}

impl EditorInput {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            buffer: VecDeque::from([String::new()]),
            command_mode: false,
            command_buffer: String::new(),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some(line) = self.buffer.get_mut(self.cursor_y) {
            line.insert(self.cursor_x, c);
            self.cursor_x += 1;
        }
    }

    pub fn backspace(&mut self) {
        if let Some(line) = self.buffer.get_mut(self.cursor_y) {
            if self.cursor_x > 0 {
                line.remove(self.cursor_x - 1);
                self.cursor_x -= 1;
            } else if self.cursor_y > 0 {
                let prev_len = self.buffer[self.cursor_y - 1].len();
                let removed = self.buffer.remove(self.cursor_y).unwrap();
                self.cursor_y -= 1;
                self.cursor_x = prev_len;
                self.buffer[self.cursor_y].push_str(&removed);
            }
        }
    }

    pub fn enter(&mut self) {
        if let Some(line) = self.buffer.get_mut(self.cursor_y) {
            let new_line = line.split_off(self.cursor_x);
            self.buffer.insert(self.cursor_y + 1, new_line);
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }

    pub fn move_cursor(&mut self, direction: InputEvent) {
        match direction {
            InputEvent::MoveLeft => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            InputEvent::MoveRight => {
                if let Some(line) = self.buffer.get(self.cursor_y) {
                    if self.cursor_x < line.len() {
                        self.cursor_x += 1;
                    }
                }
            }
            InputEvent::MoveUp => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.cursor_x.min(self.buffer[self.cursor_y].len());
                }
            }
            InputEvent::MoveDown => {
                if self.cursor_y + 1 < self.buffer.len() {
                    self.cursor_y += 1;
                    self.cursor_x = self.cursor_x.min(self.buffer[self.cursor_y].len());
                }
            }
            _ => {}
        }
    }

    pub fn get_current_line(&self) -> Option<&String> {
        self.buffer.get(self.cursor_y)
    }

    pub fn as_text(&self) -> String {
        self.buffer.iter().cloned().collect::<Vec<String>>().join("\n")
    }

    /// Écoute une touche et la traduit en événement interne
    pub fn poll_input(timeout_ms: u64) -> Option<InputEvent> {
        if event::poll(Duration::from_millis(timeout_ms)).unwrap_or(false) {
            if let Event::Key(KeyEvent { code, modifiers }) = event::read().unwrap() {
                return match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => Some(InputEvent::Quit),
                    (KeyCode::Char('s'), KeyModifiers::CONTROL) => Some(InputEvent::Save),
                    (KeyCode::Enter, _) => Some(InputEvent::Enter),
                    (KeyCode::Backspace, _) => Some(InputEvent::Backspace),
                    (KeyCode::Delete, _) => Some(InputEvent::Delete),
                    (KeyCode::Left, _) => Some(InputEvent::MoveLeft),
                    (KeyCode::Right, _) => Some(InputEvent::MoveRight),
                    (KeyCode::Up, _) => Some(InputEvent::MoveUp),
                    (KeyCode::Down, _) => Some(InputEvent::MoveDown),
                    (KeyCode::Char(':'), _) => Some(InputEvent::Command(String::new())), // Début mode commande
                    (KeyCode::Char(c), _) => Some(InputEvent::InsertChar(c)),
                    _ => Some(InputEvent::None),
                };
            }
        }
        None
    }
}
