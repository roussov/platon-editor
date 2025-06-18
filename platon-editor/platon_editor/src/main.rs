mod buffer;
mod commands;
mod editor;
mod errors;
mod file;
mod input;
mod lang;
mod syntax;
mod terminal;
mod theme;
mod markdown_render;
mod tui;

use std::{fs, io, path::PathBuf};
use serde::{Serialize, Deserialize};
use tui::Tui;
use tui::Theme;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Modifier, Style, Color},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use pulldown_cmark::{Parser, Event as MdEvent, Tag, CodeBlockKind};

/// Structure simple pour gérer la session utilisateur
#[derive(Serialize, Deserialize, Default)]
struct Session {
    last_opened_file: Option<String>,
    // ajouter d'autres champs nécessaires à la session
}

fn load_session() -> io::Result<Session> {
    let path = PathBuf::from("session.json");
    if path.exists() {
        let data = fs::read_to_string(path)?;
        let sess: Session = serde_json::from_str(&data).unwrap_or_default();
        Ok(sess)
    } else {
        Ok(Session::default())
    }
}

fn save_session(sess: &Session) -> io::Result<()> {
    let data = serde_json::to_string_pretty(sess).unwrap();
    fs::write("session.json", data)?;
    Ok(())
}
/// Theme simple pour gérer styles cohérents
struct Theme {
    title_style: Style,
    border_style: Style,
    text_style: Style,
}

impl Theme {
    fn new() -> Self {
        Self {
            title_style: Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            border_style: Style::default().fg(Color::Blue),
            text_style: Style::default().fg(Color::White),
        }
    }
}

/// Fonction avancée pour parser le markdown en Text stylisé pour TUI
fn parse_markdown_to_text_advanced(md: &str) -> Text<'_> {
    let parser = Parser::new(md);
    let mut text = Text::default();

    let mut list_stack: Vec<(bool, usize)> = Vec::new();
    let mut current_style = Style::default();

    for event in parser {
        match event {
            MdEvent::Start(tag) => match tag {
                Tag::Heading(level, _, _) => {
                    if !text.lines.is_empty() {
                        text.lines.push(Spans::from(Span::raw("")));
                    }
                    current_style = Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED);
                    let prefix = "#".repeat(level as usize) + " ";
                    text.lines.push(Spans::from(Span::styled(prefix, current_style)));
                }
                Tag::Strong => {
                    current_style = current_style.add_modifier(Modifier::BOLD);
                }
                Tag::Emphasis => {
                    current_style = current_style.add_modifier(Modifier::ITALIC);
                }
                Tag::CodeBlock(kind) => {
                    text.lines.push(Spans::from(Span::styled(
                        match kind {
                            CodeBlockKind::Indented => "```".to_string(),
                            CodeBlockKind::Fenced(lang) => format!("```{}", lang),
                        },
                        Style::default().add_modifier(Modifier::BOLD),
                    )));
                }
                Tag::List(ordered) => {
                    list_stack.push((ordered, 0));
                }
                Tag::Item => {
                    if let Some((ordered, count)) = list_stack.last_mut() {
                        if *ordered {
                            *count += 1;
                        }
                    }
                }
                Tag::BlockQuote => {
                    text.lines.push(Spans::from(Span::styled(
                        "> ",
                        Style::default().add_modifier(Modifier::ITALIC),
                    )));
                }
                _ => {}
            },
            MdEvent::End(tag) => match tag {
                Tag::Heading(_, _, _) => {
                    text.lines.push(Spans::from(Span::raw("")));
                    current_style = Style::default();
                }
                Tag::Strong => {
                    current_style = current_style.remove_modifier(Modifier::BOLD);
                }
                Tag::Emphasis => {
                    current_style = current_style.remove_modifier(Modifier::ITALIC);
                }
                Tag::CodeBlock(_) => {
                    text.lines.push(Spans::from(Span::styled(
                        "```",
                        Style::default().add_modifier(Modifier::BOLD),
                    )));
                }
                Tag::List(_) => {
                    list_stack.pop();
                }
                _ => {}
            },
            MdEvent::Text(t) => {
                if let Some((ordered, count)) = list_stack.last() {
                    let prefix = if *ordered {
                        format!("{}.", count)
                    } else {
                        "•".to_string()
                    };
                    text.lines.push(Spans::from(vec![
                        Span::styled(format!("{} ", prefix), Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(t.into_string(), current_style),
                    ]));
                } else {
                    text.lines.push(Spans::from(Span::styled(t.into_string(), current_style)));
                }
            }
            MdEvent::Code(t) => {
                text.lines.push(Spans::from(Span::styled(
                    t.into_string(),
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .bg(Color::Gray),
                )));
            }
            MdEvent::SoftBreak | MdEvent::HardBreak => {
                text.lines.push(Spans::from(Span::raw("")));
            }
            _ => {}
        }
    }

    text
}

fn main() -> Result<(), io::Error> {
    // Charger session utilisateur
    let mut session = load_session().unwrap_or_default();
    let mut tui = Tui::new()?;
    

    // Exemple : on pourrait ouvrir le dernier fichier édité, ici on charge la doc sinon
    let md = if let Some(file) = &session.last_opened_file {
        fs::read_to_string(file).unwrap_or_else(|_| "# Fichier introuvable".to_string())
    } else {
        fs::read_to_string("docs/commands.md").unwrap_or_else(|_| "# Documentation introuvable".to_string())
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let text = parse_markdown_to_text_advanced(&md);

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Documentation Platon").borders(Borders::ALL);
            let paragraph = Paragraph::new(text.clone())
                .block(block)
                .wrap(tui::widgets::Wrap { trim: true });
            f.render_widget(paragraph, size);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
                // Exemple : raccourci pour sauvegarder session
                if key.code == KeyCode::Char('s') {
                    // sauvegarde exemple
                    session.last_opened_file = Some("docs/commands.md".to_string());
                    save_session(&session).ok();
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
