use crossterm::{
    cursor,
    event::{self, Event, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    terminal::enable_raw_mode,
    terminal::disable_raw_mode,
};
use std::io::{self, Write};

pub struct Terminal {
    stdout: io::Stdout,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            stdout: io::stdout(),
        })
    }

    pub fn enter_alternate_screen(&mut self) -> io::Result<()> {
        execute!(self.stdout, EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen(&mut self) -> io::Result<()> {
        execute!(self.stdout, LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn enable_raw_mode(&self) -> io::Result<()> {
        enable_raw_mode()
    }

    pub fn disable_raw_mode(&self) -> io::Result<()> {
        disable_raw_mode()
    }

    /// Lit un événement clavier et retourne un KeyEvent
    pub fn read_input(&self) -> io::Result<KeyEvent> {
        loop {
            if let Event::Key(key_event) = event::read()? {
                return Ok(key_event);
            }
        }
    }

    /// Efface l'écran et remet le curseur en haut à gauche
    pub fn clear_screen(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )
    }

    /// Écrit une chaîne dans le terminal
    pub fn write_str(&mut self, s: &str) -> io::Result<()> {
        write!(self.stdout, "{}", s)?;
        self.stdout.flush()
    }
}
