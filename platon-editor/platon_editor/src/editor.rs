use crate::buffer::Buffer;
use crate::terminal::Terminal;
use crate::theme::Theme;
use std::io;

use tui::{
    backend::Backend,
    Frame,
    layout::Rect,
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};
use crossterm::execute;

pub struct Editor {
    buffer: Buffer,
    cursor_x: usize,
    cursor_y: usize,
    theme: Theme,
}

impl Editor {
    pub fn new(theme: Theme) -> Self {
        Self {
            buffer: Buffer::new(),
            cursor_x: 0,
            cursor_y: 0,
            theme,
        }
    }

    pub fn open_file(&mut self, path: &str) -> io::Result<()> {
        self.buffer.load_file(path)?;
        self.cursor_x = 0;
        self.cursor_y = 0;
        Ok(())
    }

    /// Rendu avec tui
    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        // Convertir les lignes du buffer en Spans stylisés
        let lines: Vec<Spans> = self
            .buffer
            .lines()
            .iter()
            .map(|line| Spans::from(Span::styled(line.clone(), self.theme.normal_style())))
            .collect();

        let block = Block::default()
            .title("Editeur Platon")
            .borders(Borders::ALL)
            .style(self.theme.status_bar_style());

        let paragraph = Paragraph::new(lines).block(block);

        f.render_widget(paragraph, area);

        // Positionner le curseur tui (dans l'aire donnée)
        let x = self.cursor_x.min(area.width as usize - 1) as u16 + area.x;
        let y = self.cursor_y.min(area.height as usize - 1) as u16 + area.y;
        f.set_cursor(x, y);
    }

    /// Ancien rendu terminal bas niveau, si tu veux garder (optionnel)
    #[allow(dead_code)]
    pub fn render_lowlevel(&self, terminal: &mut Terminal) -> io::Result<()> {
        terminal.clear_screen()?;

        for line in self.buffer.lines() {
            terminal.write_str(line)?;
            terminal.write_str("\n")?;
        }

        let _ = execute!(
            terminal.stdout,
            crossterm::cursor::MoveTo(self.cursor_x as u16, self.cursor_y as u16)
        );

        Ok(())
    }

    pub fn insert_char(&mut self, ch: char) {
        self.buffer.insert_char(self.cursor_y, self.cursor_x, ch);
        self.cursor_x += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.buffer.delete_char(self.cursor_y, self.cursor_x - 1);
            self.cursor_x -= 1;
        }
    }

    // Ajouter plus de méthodes: navigation, sauvegarde, etc.
}
