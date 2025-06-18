use pulldown_cmark::{Parser, Event, Tag};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

pub fn render_markdown(md_text: &str) {
    let parser = Parser::new(md_text);

    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading(level) => {
                    execute!(
                        stdout,
                        SetForegroundColor(match level {
                            1 => Color::Blue,
                            2 => Color::Cyan,
                            3 => Color::Green,
                            _ => Color::White,
                        })
                    )
                    .unwrap();
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                Tag::Heading(_) => {
                    execute!(stdout, ResetColor).unwrap();
                    println!();
                }
                _ => {}
            },
            Event::Text(text) => {
                print!("{}", text);
                stdout.flush().unwrap();
            }
            Event::Code(text) => {
                execute!(stdout, SetForegroundColor(Color::Yellow)).unwrap();
                print!("{}", text);
                execute!(stdout, ResetColor).unwrap();
                stdout.flush().unwrap();
            }
            Event::SoftBreak | Event::HardBreak => {
                println!();
            }
            _ => {}
        }
    }
}
