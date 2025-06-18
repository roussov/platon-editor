use tui::style::{Color, Style};

#[derive(Clone, Debug)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub selection: Color,
    pub line_number: Color,
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
    // Ajouter d’autres couleurs/thèmes selon besoin
}

impl Theme {
    pub fn light() -> Self {
        Theme {
            background: Color::White,
            foreground: Color::Black,
            cursor: Color::Blue,
            selection: Color::Rgb(200, 200, 255),
            line_number: Color::DarkGray,
            status_bar_bg: Color::Rgb(220, 220, 220),
            status_bar_fg: Color::Black,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Green,
        }
    }

    pub fn dark() -> Self {
        Theme {
            background: Color::Black,
            foreground: Color::White,
            cursor: Color::LightBlue,
            selection: Color::Rgb(50, 50, 100),
            line_number: Color::Gray,
            status_bar_bg: Color::Rgb(30, 30, 30),
            status_bar_fg: Color::White,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::LightGreen,
        }
    }

    /// Exemple de style pour le texte normal
    pub fn normal_style(&self) -> Style {
        Style::default().fg(self.foreground).bg(self.background)
    }

    /// Style pour la barre de statut
    pub fn status_bar_style(&self) -> Style {
        Style::default().fg(self.status_bar_fg).bg(self.status_bar_bg)
    }

    /// Style pour la ligne de numéro
    pub fn line_number_style(&self) -> Style {
        Style::default().fg(self.line_number).bg(self.background)
    }

    /// Style pour la sélection
    pub fn selection_style(&self) -> Style {
        Style::default().bg(self.selection)
    }

    /// Style pour le curseur
    pub fn cursor_style(&self) -> Style {
        Style::default().fg(self.cursor).bg(self.background)
    }
}
