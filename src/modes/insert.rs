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
                    terminal::Clear(ClearType::All),
                    cursor::MoveToNextLine(1)
                )
                .unwrap();
                print_content(main_struct, false).unwrap();
            }
        },
        KeyCode::Backspace => {
            if main_struct.cursor_pos != (0, 0) {
                let mut text = main_struct.buffer.clone().finish();
                let mut full_clear = false;
                if let Some(c) = text.get_char(main_struct.cursor_char - 1)
                    && c == '\n'
                {
                    full_clear = true;
                }
                if let Ok(_) = text.try_remove(main_struct.cursor_char - 1..main_struct.cursor_char)
                {
                    main_struct.buffer = RopeBuilder::new();
                    main_struct.buffer.append(text.to_string().as_str());
                    execute!(stdout(), cursor::Hide).unwrap();
                    if main_struct.cursor_pos.0 != 0 {
                        execute!(stdout(), cursor::MoveLeft(1)).unwrap();
                    } else {
                        execute!(stdout(), cursor::MoveToPreviousLine(1)).unwrap();
                        if text
                            .line(cursor::position().unwrap().1 as usize + main_struct.scroll_offset as usize - 1)
                            .len_chars()
                            != 0
                        {
                            queue!(
                                stdout(),
                                cursor::MoveToColumn(
                                    text.line(
                                        cursor::position().unwrap().1 as usize
                                            + main_struct.scroll_offset as usize
                                    )
                                    .len_chars() as u16
                                        - 1
                                )
                            )
                            .expect("Failed to move cursor previous line, and/or to column!");
                        }
                    }
                    main_struct.target_col = main_struct.cursor_pos.0.into();
                    if full_clear == true {
                        execute!(stdout(), terminal::Clear(ClearType::FromCursorDown)).unwrap();
                    }
                    if let Some(l) = text.get_line(
                        cursor::position().unwrap().1 as usize + main_struct.scroll_offset as usize,
                    ) && l.len_chars() == 0
                        && l.chars().last() == Some('\n')
                    {
                        execute!(stdout(), cursor::MoveToColumn(0)).unwrap();
                    }
                    execute!(stdout(), cursor::Show).unwrap();
                    print_content(main_struct, true).unwrap();
                }
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
