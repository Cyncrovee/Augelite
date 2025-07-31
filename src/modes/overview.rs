use std::io::stdout;

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
};

use crate::{
    AugeliteState,
    util::{
        cursor_movement::{self, cursor_max_col},
        misc::to_col,
        model::Mode,
        scrolling,
    },
};

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
            'I' => {
                to_col(0);
                main_struct.mode = Mode::Ins;
            }
            'A' => {
                cursor_max_col(main_struct);
                main_struct.mode = Mode::Ins;
            }
            'w' => cursor_movement::cursor_word(main_struct),
            'b' => cursor_movement::cursor_back(main_struct),
            'p' => {
                if key.modifiers == KeyModifiers::CONTROL {
                    if main_struct.scroll_offset != 0 {
                        scrolling::scroll_up(main_struct);
                    }
                }
            }
            'n' => {
                if key.modifiers == KeyModifiers::CONTROL {
                    scrolling::scroll_down(main_struct);
                }
            }
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
        KeyCode::PageDown => scrolling::scroll_down(main_struct),
        KeyCode::PageUp => scrolling::scroll_up(main_struct),
        _ => {}
    }
    true
}
