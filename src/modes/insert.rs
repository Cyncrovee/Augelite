use std::io::stdout;

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
};
use ropey::RopeBuilder;

use crate::{
    AugeliteState,
    util::{
        cursor_movement,
        misc::{move_left_one, move_right_one, down_line_one, print_content, to_col, up_line_one},
        model::Mode,
    },
};

pub fn insert_input(key: KeyEvent, main_struct: &mut AugeliteState) -> bool {
    match key.code {
        KeyCode::Char(c) => {
            if c == 'q' && key.modifiers == KeyModifiers::CONTROL {
                execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
                return false;
            } else {
                let text = main_struct.buffer.clone().finish();
                let mut text = text.to_string();
                text.insert(main_struct.cursor_char, c);
                main_struct.buffer = RopeBuilder::new();
                main_struct.buffer.append(text.as_str());
                print_content(main_struct.buffer.clone().finish(), false).unwrap();
                move_right_one();
                main_struct.target_col = cursor::position().unwrap().0.into();
            }
        }
        KeyCode::Left => {
            cursor_movement::cursor_left(main_struct);
        }
        KeyCode::Right => {
            cursor_movement::cursor_right(main_struct);
        }
        KeyCode::Up => {
            cursor_movement::cursor_up(main_struct);
        }
        KeyCode::Down => {
            cursor_movement::cursor_down(main_struct);
        }
        KeyCode::Enter => {
            main_struct.buffer.append("\n");
            down_line_one();
        }
        KeyCode::Backspace => {
            if main_struct.cursor_pos != (0, 0) {
                let mut text = main_struct.buffer.clone().finish();
                text.remove(main_struct.cursor_char - 1..main_struct.cursor_char);
                main_struct.buffer = RopeBuilder::new();
                main_struct.buffer.append(text.to_string().as_str());
                if cursor::position().unwrap().0 != 0 {
                    move_left_one();
                } else {
                    up_line_one();
                    to_col(
                        text.line(cursor::position().unwrap().1.into())
                            .len_chars()
                            .try_into()
                            .unwrap(),
                    );
                }
                main_struct.target_col = main_struct.cursor_pos.0.into();
                print_content(main_struct.buffer.clone().finish(), true).unwrap();
            }
        }
        KeyCode::Esc => main_struct.mode = Mode::Ovr,
        _ => {}
    }
    true
}
