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
        misc::{down_line_one, move_left_one, move_right_one, to_col, up_line_one},
        model::Mode,
        scrolling::{self, scroll_down},
        view::{check_end_of_view, print_content},
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
                print_content(main_struct, false).unwrap();
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
        KeyCode::Enter => match check_end_of_view(main_struct) {
            true => {
                scroll_down(main_struct);
                main_struct.buffer.append("\n");
                down_line_one();
            }
            false => {
                main_struct.buffer.append("\n");
                down_line_one();
            }
        },
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
                print_content(main_struct, true).unwrap();
            }
        }
        KeyCode::Esc => main_struct.mode = Mode::Ovr,
        KeyCode::PageDown => {
            if (main_struct.scroll_offset as usize)
                < main_struct.buffer.clone().finish().lines().count() - 1
            {
                scrolling::scroll_down(main_struct);
                print_content(main_struct, true).unwrap();
            }
        }
        KeyCode::PageUp => {
            if main_struct.scroll_offset != 0 {
                scrolling::scroll_up(main_struct);
                print_content(main_struct, true).unwrap();
            }
        }
        _ => {}
    }
    true
}
