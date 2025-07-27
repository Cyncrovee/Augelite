use std::io::stdout;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
};
use ropey::RopeBuilder;
use util::{
    check_target_col, move_down, move_left, move_right, move_up, new_line, print_content, to_col,
    to_row, up_line,
};

mod util;

struct AugeliteState {
    buffer: RopeBuilder,
    target_col: usize, // file_path: String,
}

fn main() -> std::io::Result<()> {
    execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    // execute!(stdout(), crossterm::terminal::DisableLineWrap).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(stdout(), crossterm::cursor::Show).unwrap();
    AugeliteState::run(&mut AugeliteState {
        buffer: RopeBuilder::new(),
        target_col: 0,
    });

    Ok(())
}

impl AugeliteState {
    fn run(&mut self) {
        to_col(0);
        to_row(0);
        loop {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            let cursor_pos = cursor::position().unwrap();
                            if c == 'q' && key.modifiers == KeyModifiers::CONTROL {
                                execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)
                                    .unwrap();
                                break;
                            } else {
                                let text = self.buffer.clone().finish();
                                let char = text.line_to_char(cursor_pos.1 as usize)
                                    + cursor_pos.0 as usize;
                                let mut text = text.to_string();
                                text.insert(char, c);
                                self.buffer = RopeBuilder::new();
                                self.buffer.append(text.as_str());
                                print_content(self.buffer.clone().finish(), false).unwrap();
                                move_right();
                                self.target_col = cursor::position().unwrap().0.into();
                            }
                        }
                        KeyCode::Left => {
                            let cursor_pos = cursor::position().unwrap();
                            let text = self.buffer.clone().finish();
                            if cursor_pos.0 == 0 && cursor_pos.1 != 0 {
                                if let Some(line) = text.lines().nth(cursor_pos.1 as usize - 1) {
                                    up_line();
                                    to_col((line.len_chars()) as u16 - 1);
                                }
                            } else {
                                move_left();
                            }
                            self.target_col = cursor::position().unwrap().0.into();
                        }
                        KeyCode::Right => {
                            let mut will_move_right = true;
                            let cursor_pos = cursor::position().unwrap();
                            let text = self.buffer.clone().finish();
                            if text.lines().nth(cursor_pos.1 as usize + 1).is_some()
                                && text.line(cursor_pos.1 as usize).char(cursor_pos.0 as usize)
                                    == '\n'
                            {
                                will_move_right = false;
                                new_line();
                            }
                            if text.line(cursor_pos.1 as usize).len_chars() == cursor_pos.0 as usize
                            {
                                will_move_right = false;
                            }
                            if will_move_right {
                                move_right();
                            }
                            self.target_col = cursor::position().unwrap().0.into();
                        }
                        KeyCode::Up => {
                            move_up();
                            to_col(
                                self.buffer
                                    .clone()
                                    .finish()
                                    .line(cursor::position().unwrap().1.into())
                                    .len_chars()
                                    .try_into()
                                    .unwrap(),
                            );
                            move_left();
                            if self.target_col != 0
                                && check_target_col(
                                    self.buffer.clone().finish(),
                                    cursor::position().unwrap().1.into(),
                                    self.target_col,
                                )
                            {
                                to_col(self.target_col as u16);
                            }
                        }
                        KeyCode::Down => {
                            let cursor_pos = cursor::position().unwrap();
                            let text = self.buffer.clone().finish();
                            if text.lines().nth(cursor_pos.1 as usize + 1).is_some() {
                                move_down();
                                to_col(
                                    self.buffer
                                        .clone()
                                        .finish()
                                        .line(cursor::position().unwrap().1.into())
                                        .len_chars()
                                        .try_into()
                                        .unwrap(),
                                );
                                move_left();
                                if self.target_col != 0
                                    && check_target_col(
                                        self.buffer.clone().finish(),
                                        (cursor::position().unwrap().1).into(),
                                        self.target_col,
                                    )
                                {
                                    to_col(self.target_col as u16);
                                }
                            }
                        }
                        KeyCode::Enter => {
                            self.buffer.append("\n");
                            new_line();
                        }
                        KeyCode::Backspace => {
                            let cursor_pos = cursor::position().unwrap();
                            if cursor_pos != (0, 0) {
                                let mut text = self.buffer.clone().finish();
                                let char = text.line_to_char(cursor_pos.1 as usize)
                                    + cursor_pos.0 as usize
                                    - 1;
                                text.remove(char..char + 1);
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
                                print_content(self.buffer.clone().finish(), true).unwrap();
                            }
                            self.target_col = cursor_pos.0.into();
                        }
                        KeyCode::Esc => {
                            execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
