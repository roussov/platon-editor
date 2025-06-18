use crossterm::style::{Attribute, Color, ContentStyle, Stylize};

/// Styles de base utilisés dans tout l’éditeur.
pub struct Style;

impl Style {
    pub fn title() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Cyan),
            background_color: None,
            attributes: Attribute::Bold.into(),
        }
    }

    pub fn status_info() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Green),
            background_color: None,
            attributes: Attribute::Underlined.into(),
        }
    }

    pub fn status_warning() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Yellow),
            background_color: None,
            attributes: Attribute::Bold.into(),
        }
    }

    pub fn status_error() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Red),
            background_color: None,
            attributes: Attribute::Bold.into(),
        }
    }

    pub fn line_number() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::DarkGrey),
            background_color: None,
            attributes: Attribute::Dim.into(),
        }
    }

    pub fn line_highlight() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Black),
            background_color: Some(Color::Grey),
            attributes: Attribute::Bold.into(),
        }
    }

    pub fn cursor() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::White),
            background_color: Some(Color::Blue),
            attributes: Attribute::Bold.into(),
        }
    }

    pub fn code_default() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::White),
            background_color: None,
            attributes: Attribute::NoItalic.into(),
        }
    }

    pub fn keyword() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Magenta),
            background_color: None,
            attributes: Attribute::Bold.into(),
        }
    }

    pub fn comment() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::DarkGrey),
            background_color: None,
            attributes: Attribute::Italic.into(),
        }
    }

    pub fn string_literal() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Yellow),
            background_color: None,
            attributes: Attribute::NoBold.into(),
        }
    }

    pub fn function_name() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Blue),
            background_color: None,
            attributes: Attribute::Bold.into(),
        }
    }
}
