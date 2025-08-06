use std::io::stdout;

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    terminal::{self, ClearType},
};
use ropey::RopeBuilder;

use crate::{
    AugeliteState,
    util::{
        cursor_movement,
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
                execute!(stdout(), cursor::MoveRight(1)).unwrap();
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
                let text = main_struct.buffer.clone().finish();
                let mut text = text.to_string();
                text.insert(main_struct.cursor_char, '\n');
                main_struct.buffer = RopeBuilder::new();
                main_struct.buffer.append(text.as_str());
                scroll_down(main_struct);
                queue!(
                    stdout(),
                    terminal::Clear(ClearType::All),
                    cursor::MoveToNextLine(1)
                )
                .unwrap();
                print_content(main_struct, false).unwrap();
            }
            false => {
                let text = main_struct.buffer.clone().finish();
                let mut text = text.to_string();
                text.insert(main_struct.cursor_char, '\n');
                main_struct.buffer = RopeBuilder::new();
                main_struct.buffer.append(text.as_str());
                queue!(
                    stdout(),
                    terminal::Clear(ClearType::FromCursorDown),
                    cursor::MoveToNextLine(1)
                )
                .unwrap();
                print_content(main_struct, false).unwrap();
            }
        },
        KeyCode::Backspace => {
            if main_struct.cursor_pos != (0, 0) {
                let mut text = main_struct.buffer.clone().finish();
                let current_col = main_struct.cursor_pos.0 as usize;
                if let Some(c) = text.get_char(main_struct.cursor_char - 1)
                    && c != '\n'
                {
                    if let Ok(_) =
                        text.try_remove(main_struct.cursor_char - 1..main_struct.cursor_char)
                    {
                        main_struct.buffer = RopeBuilder::new();
                        main_struct.buffer.append(text.to_string().as_str());
                        if current_col != 0 {
                            cursor_movement::cursor_left(main_struct);
                        } else {
                            cursor_movement::cursor_up_to_end_of_line(main_struct);
                        }
                    }
                } else {
                    if let Ok(_) =
                        text.try_remove(main_struct.cursor_char - 1..main_struct.cursor_char)
                    {
                        main_struct.buffer = RopeBuilder::new();
                        main_struct.buffer.append(text.to_string().as_str());
                        let line_len = main_struct
                            .buffer
                            .clone()
                            .finish()
                            .line(
                                cursor::position().unwrap().1 as usize
                                    + main_struct.scroll_offset as usize
                                    - 1,
                            )
                            .len_chars();
                        queue!(
                            stdout(),
                            cursor::MoveUp(1),
                            cursor::MoveToColumn(line_len.try_into().unwrap()),
                        )
                        .unwrap();
                    }
                }
                execute!(stdout(), terminal::Clear(ClearType::FromCursorDown)).unwrap();
                print_content(main_struct, true).unwrap();
            }
        }
        KeyCode::Esc => {
            main_struct.mode = Mode::Ovr;
            execute!(stdout(), crossterm::cursor::SetCursorStyle::SteadyBlock).unwrap();
        }
        KeyCode::PageDown => scrolling::scroll_down(main_struct),
        KeyCode::PageUp => scrolling::scroll_up(main_struct),
        _ => {}
    }
    true
}
