use std::{fs::read_to_string, io::stdout};

use crossterm::{
    cursor,
    event::{self, Event, KeyEventKind},
    execute,
};
use ropey::RopeBuilder;
use util::{
    model::{AugeliteState, Mode},
    view::{print_content, statusline},
};

mod modes;
mod util;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(stdout(), crossterm::cursor::SetCursorStyle::SteadyBlock).unwrap();
    execute!(stdout(), crossterm::cursor::Show).unwrap();
    AugeliteState::run(&mut AugeliteState {
        buffer: if let Some(arg) = args.iter().nth(1) {
            match read_to_string(arg) {
                Ok(file) => {
                    let mut rope = RopeBuilder::new();
                    rope.append(&file);
                    rope
                }
                Err(_) => {
                    if let Some(arg) = args.iter().nth(1) {
                        let mut rope_builder = RopeBuilder::new();
                        rope_builder.append(arg);
                        rope_builder
                    } else {
                        RopeBuilder::new()
                    }
                }
            }
        } else {
            RopeBuilder::new()
        },
        cursor_pos: (0, 0),
        cursor_char: 0,
        target_col: 0,
        scroll_offset: 0,
        mode: Mode::Ovr,
        file_path: if let Some(arg) = args.iter().nth(1) {
            Some(arg.to_string())
        } else {
            None
        },
    });

    Ok(())
}

impl AugeliteState {
    fn run(&mut self) {
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        if let Some(_) = self.file_path {
            print_content(self, false).unwrap();
        }
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
                        .line_to_char(self.cursor_pos.1 as usize + self.scroll_offset as usize)
                        + self.cursor_pos.0 as usize;
                    statusline(self).unwrap();
                }
            }
        }
    }
}
