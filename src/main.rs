use std::io::stdout;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
};
use ropey::RopeBuilder;
use util::{
    check_target_col, move_down, move_left, move_right, move_up, new_line, print_content, statusline, to_col, to_row, up_line
};

mod util;
mod cursor_movement;

enum Mode {
    Ovr,
    Ins,
}

struct AugeliteState {
    buffer: RopeBuilder,
    cursor_pos: (u16, u16),
    cursor_char: usize,
    target_col: usize,
    mode: Mode,
}

fn main() -> std::io::Result<()> {
    execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    // execute!(stdout(), crossterm::terminal::DisableLineWrap).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(stdout(), crossterm::cursor::Show).unwrap();
    AugeliteState::run(&mut AugeliteState {
        buffer: RopeBuilder::new(),
        cursor_pos: (0, 0),
        cursor_char: 0,
        target_col: 0,
        mode: Mode::Ovr,
    });

    Ok(())
}

impl AugeliteState {
    fn run(&mut self) {
        to_col(0);
        to_row(0);
        statusline(self).unwrap();
        loop {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match self.mode {
                        Mode::Ovr => {
                            if let KeyCode::Char(c) = key.code {
                                match c {
                                    'i' => self.mode = Mode::Ins,
                                    'q' => {
                                        if key.modifiers == KeyModifiers::CONTROL {
                                            execute!(
                                                stdout(),
                                                crossterm::terminal::LeaveAlternateScreen
                                            )
                                            .unwrap();
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Mode::Ins => match key.code {
                            KeyCode::Char(c) => {
                                if c == 'q' && key.modifiers == KeyModifiers::CONTROL {
                                    execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)
                                        .unwrap();
                                    break;
                                } else {
                                    let text = self.buffer.clone().finish();
                                    let mut text = text.to_string();
                                    text.insert(self.cursor_char, c);
                                    self.buffer = RopeBuilder::new();
                                    self.buffer.append(text.as_str());
                                    print_content(self.buffer.clone().finish(), false).unwrap();
                                    move_right();
                                    self.target_col = cursor::position().unwrap().0.into();
                                }
                            }
                            KeyCode::Left => {
                                cursor_movement::cursor_left(self);
                            }
                            KeyCode::Right => {
                                cursor_movement::cursor_right(self);
                            }
                            KeyCode::Up => {
                                cursor_movement::cursor_up(self);
                            }
                            KeyCode::Down => {
                                cursor_movement::cursor_down(self);
                            }
                            KeyCode::Enter => {
                                self.buffer.append("\n");
                                new_line();
                            }
                            KeyCode::Backspace => {
                                if self.cursor_pos != (0, 0) {
                                    let mut text = self.buffer.clone().finish();
                                    text.remove(self.cursor_char - 1..self.cursor_char);
                                    self.buffer = RopeBuilder::new();
                                    self.buffer.append(text.to_string().as_str());
                                    if cursor::position().unwrap().0 != 0 {
                                        move_left();
                                    } else {
                                        up_line();
                                        to_col(
                                            text.line(cursor::position().unwrap().1.into())
                                                .len_chars()
                                                .try_into()
                                                .unwrap(),
                                        );
                                    }
                                    self.target_col = self.cursor_pos.0.into();
                                    print_content(self.buffer.clone().finish(), true).unwrap();
                                }
                            }
                            KeyCode::Esc => self.mode = Mode::Ovr,
                            _ => {}
                        },
                    }
                    self.cursor_pos = cursor::position().unwrap();
                    self.cursor_char = self
                        .buffer
                        .clone()
                        .finish()
                        .line_to_char(self.cursor_pos.1 as usize)
                        + self.cursor_pos.0 as usize;
                    statusline(self).unwrap();
                }
            }
        }
    }
}
