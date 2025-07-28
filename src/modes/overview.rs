use std::io::stdout;

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
};

use crate::{util::{cursor_movement::{self, cursor_max_col}, misc::to_col, model::Mode}, AugeliteState};

pub fn overview_input(key: KeyEvent, main_struct: &mut AugeliteState) -> bool {
    match key.code {
        KeyCode::Char(c) => match c {
            '0' => to_col(0),
            ')' => cursor_max_col(main_struct),
            'i' => main_struct.mode = Mode::Ins,
            'h' => cursor_movement::cursor_left(main_struct),
            'j' => cursor_movement::cursor_down(main_struct),
            'k' => cursor_movement::cursor_up(main_struct),
            'l' => cursor_movement::cursor_right(main_struct),
            'q' => {
                if key.modifiers == KeyModifiers::CONTROL {
                    execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
                    return false;
                }
            }
            _ => {}
        },
        KeyCode::Left => cursor_movement::cursor_left(main_struct),
        KeyCode::Right => cursor_movement::cursor_right(main_struct),
        KeyCode::Up => cursor_movement::cursor_up(main_struct),
        KeyCode::Down => cursor_movement::cursor_down(main_struct),
        _ => {}
    }
    true
}
