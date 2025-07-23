use std::{
    env::{self},
    fs::{self},
    io::Write, usize,
};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{self, style::Color},
    layout::{Constraint, Layout, Position},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
};
use ropey::RopeBuilder;

#[derive(Default)]
struct AppState {
    running: bool,
    file: (String, bool),
    editor_content: RopeBuilder,
    current_char: usize,
    cursor_row: usize,
    cursor_column: usize,
    max_col: usize,
}

fn main() -> Result<()> {
    let arg = env::args().nth(1);
    let mut _is_file = false;
    match arg {
        Some(_) => _is_file = true,
        None => _is_file = false,
    }
    let mut editor_content = RopeBuilder::new();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = AppState::run(
        &mut AppState {
            running: true,
            file: (
                if _is_file == true {
                    arg.clone().unwrap()
                } else {
                    "".to_string()
                },
                _is_file,
            ),
            editor_content: if _is_file == true {
                let file_content = fs::read_to_string(arg.unwrap());
                editor_content.append(file_content.unwrap().as_str());
                editor_content
            } else {
                editor_content
            },
            current_char: 0,
            cursor_row: 0,
            cursor_column: 0,
            max_col: 0,
        },
        terminal,
    );
    ratatui::restore();

    result
}

impl AppState {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running == true {
            terminal.draw(|frame| self.view(frame))?;
            if let false = AppState::key_events(self) {
                break;
            }
        }
        Ok(())
    }

    pub fn key_events(&mut self) -> bool {
        if let Event::Key(key) = event::read().unwrap() {
            if let KeyEventKind::Press = key.kind {
                match key.code {
                    KeyCode::Char(c) => {
                        self.editor_content.append(c.to_string().as_str());
                        self.cursor_row += 1;
                        self.current_char += 1;
                        self.max_col += 1;
                    }
                    KeyCode::Enter => {
                        self.editor_content.append("\n");
                        self.cursor_row = 0;
                        self.cursor_column += 1;
                        self.current_char += 1;
                        self.max_col = 0;
                    }
                    KeyCode::Backspace => {
                        if self.current_char != 0 {
                            let mut text = self.editor_content.clone().finish().to_string();
                            text.remove(self.current_char - 1);
                            self.editor_content = RopeBuilder::new();
                            self.editor_content.append(text.as_str());
                            if self.cursor_row == 0 && self.cursor_column != 0 {
                                self.cursor_column -= 1;
                                self.cursor_row += 1;
                                self.current_char -= 1;
                                match text.lines().nth(self.cursor_column) {
                                    Some(line) => self.cursor_row = line.chars().count(),
                                    None => self.cursor_row = 0,
                                }
                            } else {
                                self.cursor_row -= 1;
                                self.current_char -= 1;
                            }
                        }
                    }
                    KeyCode::Left => {
                        if self.current_char != 0 && self.cursor_row != 0 {
                            self.cursor_row -= 1;
                            self.current_char -= 1;
                        } else if self.current_char != 0 && self.cursor_row == 0 {
                            self.cursor_column -= 1;
                            self.cursor_row += 1;
                            self.current_char -= 1;
                            let text = self.editor_content.clone().finish().to_string();
                            match text.lines().nth(self.cursor_column) {
                                Some(line) => self.cursor_row = line.chars().count(),
                                None => self.cursor_row = 0,
                            }
                        }
                    }
                    KeyCode::Right => {
                        let text = self.editor_content.clone().finish().to_string();
                        if let Some(line) = text.lines().nth(self.cursor_column) {
                            if line.len() == self.cursor_row {
                                match text.lines().nth(self.cursor_column + 1) {
                                    Some(_) => {
                                        self.current_char += 1;
                                        self.cursor_column += 1;
                                        self.cursor_row = 0;
                                    }
                                    None => {}
                                }
                            } else {
                                self.cursor_row += 1;
                                self.current_char += 1;
                            }
                        } else {
                        }
                    }
                    /*
                    KeyCode::Up => {
                        self.cursor_column -= 1;
                    }
                    KeyCode::Down => {
                        self.cursor_column += 1;
                    }
                    */
                    KeyCode::Home => {
                        self.cursor_row = 0;
                        self.cursor_column = 0;
                    }
                    KeyCode::Insert => match self.file.1 {
                        true => {
                            let mut f = std::fs::File::create(&self.file.0).unwrap();
                            write!(f, "{}", self.editor_content.clone().finish().to_string())
                                .unwrap();
                        }
                        false => {}
                    },
                    KeyCode::Esc => {
                        return false;
                    }
                    _ => {}
                }
            }
        }
        return true;
    }

    pub fn view(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Percentage(95), Constraint::Percentage(5)])
            .split(frame.area());

        let mut status_bar_content = " ".to_owned();
        status_bar_content.push_str(self.current_char.to_string().as_str());
        status_bar_content.push_str(" | ");
        status_bar_content.push_str(self.cursor_row.to_string().as_str());
        status_bar_content.push(':');
        status_bar_content.push_str(self.cursor_column.to_string().as_str());
        let status_bar = Line::raw(status_bar_content)
            .bg(Color::White)
            .fg(Color::Black);

        frame.render_widget(
            Paragraph::new(self.editor_content.clone().finish().to_string()),
            layout[0],
        );
        frame.render_widget(status_bar, layout[1]);
        frame.set_cursor_position(Position {
            x: self.cursor_row as u16,
            y: self.cursor_column as u16,
        });
    }
}
