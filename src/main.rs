use std::io::stdout;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, terminal::{self, ClearType},
};
use ropey::RopeBuilder;
use util::{
    check_target_col, move_down, move_left, move_right, move_up, new_line, print_content, to_col,
    to_row, up_line,
};

mod util;

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
                                let text = self.buffer.clone().finish();
                                if self.cursor_pos.0 == 0 && self.cursor_pos.1 != 0 {
                                    if let Some(line) =
                                        text.lines().nth(self.cursor_pos.1 as usize - 1)
                                    {
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
                                let text = self.buffer.clone().finish();
                                if text.lines().nth(self.cursor_pos.1 as usize + 1).is_some()
                                    && text
                                        .line(self.cursor_pos.1 as usize)
                                        .char(self.cursor_pos.0 as usize)
                                        == '\n'
                                {
                                    will_move_right = false;
                                    new_line();
                                }
                                if text.line(self.cursor_pos.1 as usize).len_chars()
                                    == self.cursor_pos.0 as usize
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
                                let text = self.buffer.clone().finish();
                                if text.lines().nth(self.cursor_pos.1 as usize + 1).is_some() {
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

fn statusline(main_struct: &AugeliteState) -> std::io::Result<()> {
    execute!(stdout(), cursor::SavePosition)?;
    to_col(1);
    to_row(terminal::size().unwrap().1 - 1);
    execute!(stdout(), terminal::Clear(ClearType::CurrentLine))?;
    match main_struct.mode {
        Mode::Ovr => print!("OVERVIEW"),
        Mode::Ins => print!("INSERT"),
    }
    print!(" | ");
    print!("{}", main_struct.cursor_pos.1 as usize);
    print!(":");
    print!("{}", main_struct.cursor_pos.0 as usize);
    execute!(stdout(), cursor::RestorePosition)?;
    Ok(())
}
