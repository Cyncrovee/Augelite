use std::io::stdout;

use crossterm::{
    cursor,
    event::{self, Event, KeyEventKind},
    execute,
};
use ropey::RopeBuilder;
use util::{misc::{statusline, to_col, to_row}, model::{AugeliteState, Mode}};

mod modes;
mod util;

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
                            if !modes::overview::overview_input(key, self) {
                                break;
                            }
                        }
                        Mode::Ins => {
                            if !modes::insert::insert_input(key, self) {
                                break;
                            }
                        }
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
