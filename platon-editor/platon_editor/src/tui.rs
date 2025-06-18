use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::{io, time::Duration};

/// Structure simple pour gérer un thème d'interface
pub struct Theme {
    pub title_style: Style,
    pub border_style: Style,
    pub text_style: Style,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            title_style: Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            border_style: Style::default().fg(Color::Blue),
            text_style: Style::default().fg(Color::White),
        }
    }
}

/// Structure qui encapsule la terminal TUI
pub struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    theme: Theme,
}

impl Tui {
    /// Initialise et entre en mode raw + alternate screen
    pub fn new() -> Result<Self, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            theme: Theme::new(),
        })
    }

    /// Nettoie et sort proprement du mode terminal raw
    pub fn exit(&mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    /// Render le texte markdown stylisé dans le terminal
    pub fn render_text(&mut self, text: Text<'_>) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .title(Span::styled("Documentation Platon", self.theme.title_style))
                .borders(Borders::ALL)
                .border_style(self.theme.border_style);

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(self.theme.text_style)
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, size);
        })?;

        Ok(())
    }

    /// Attend les événements clavier (timeout ms en millisecondes)
    /// Retourne Some(KeyCode) si une touche a été pressée, sinon None si timeout
    pub fn poll_event(&self, timeout_ms: u64) -> io::Result<Option<KeyCode>> {
        if event::poll(Duration::from_millis(timeout_ms))? {
            if let Event::Key(key) = event::read()? {
                return Ok(Some(key.code));
            }
        }
        Ok(None)
    }
}
