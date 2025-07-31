use std::io::stdout;

use crossterm::{
    cursor::{self, MoveLeft},
    execute, queue,
};

use crate::AugeliteState;

use super::view::check_target_col;

pub fn cursor_left(main_struct: &mut AugeliteState) {
    let text = main_struct.buffer.clone().finish();
    if main_struct.cursor_pos.0 == 0 && main_struct.cursor_pos.1 != 0 {
        if let Some(line) = text.lines().nth(main_struct.cursor_pos.1 as usize - 1) {
            queue!(
                stdout(),
                cursor::MoveUp(1),
                cursor::MoveToColumn(line.len_chars() as u16 - 1),
            )
            .unwrap();
        }
    } else {
        execute!(stdout(), cursor::MoveLeft(1)).unwrap();
    }
    main_struct.target_col = cursor::position().unwrap().0.into();
}

pub fn cursor_right(main_struct: &mut AugeliteState) {
    let mut will_move_right = true;
    let text = main_struct.buffer.clone().finish();
    if text
        .lines()
        .nth(main_struct.cursor_pos.1 as usize + 1)
        .is_some()
        && text
            .line(main_struct.cursor_pos.1 as usize)
            .char(main_struct.cursor_pos.0 as usize)
            == '\n'
    {
        will_move_right = false;
        execute!(stdout(), cursor::MoveToNextLine(1)).unwrap();
    }
    if text.line(main_struct.cursor_pos.1 as usize).len_chars() == main_struct.cursor_pos.0 as usize
    {
        will_move_right = false;
    }
    if will_move_right {
        execute!(stdout(), cursor::MoveRight(1)).unwrap();
    }
    main_struct.target_col = cursor::position().unwrap().0.into();
}

pub fn cursor_up(main_struct: &mut AugeliteState) {
    if main_struct.cursor_pos.1 != 0 {
        let text = main_struct.buffer.clone().finish();
        queue!(
            stdout(),
            cursor::Hide,
            cursor::MoveUp(1),
            cursor::MoveToColumn(
                text.line(cursor::position().unwrap().1.into())
                    .len_chars()
                    .try_into()
                    .unwrap(),
            ),
            cursor::MoveLeft(1)
        )
        .unwrap();
        if check_target_col(
            text,
            cursor::position().unwrap().1.into(),
            main_struct.target_col,
        ) {
            execute!(
                stdout(),
                cursor::MoveToColumn(main_struct.target_col as u16)
            )
            .unwrap();
        }
        execute!(stdout(), cursor::Show).unwrap();
    }
}

pub fn cursor_down(main_struct: &mut AugeliteState) {
    let text = main_struct.buffer.clone().finish();
    if text
        .lines()
        .nth(main_struct.cursor_pos.1 as usize + 1)
        .is_some()
    {
        queue!(
            stdout(),
            cursor::Hide,
            cursor::MoveToNextLine(1),
            cursor::MoveToColumn(
                text.line(cursor::position().unwrap().1 as usize + main_struct.scroll_offset as usize)
                    .len_chars()
                    .try_into()
                    .unwrap(),
            ),
            cursor::MoveLeft(1)
        )
        .unwrap();
        if check_target_col(
            text,
            (cursor::position().unwrap().1).into(),
            main_struct.target_col,
        ) {
            execute!(
                stdout(),
                cursor::MoveToColumn(main_struct.target_col as u16)
            )
            .unwrap();
        }
        execute!(stdout(), cursor::Show).unwrap();
    }
}

pub fn cursor_word(main_struct: &mut AugeliteState) {
    let text = main_struct.buffer.clone().finish();
    let line = text.line(main_struct.cursor_pos.1 as usize);
    let start = main_struct.cursor_pos.0 as usize;
    let line_slice = line.slice(start..line.len_chars());
    let mut col: u16 = 0;
    let mut will_move_to_col = true;
    for char in line_slice.chars() {
        match char {
            ' ' => {
                col += 1;
                line_slice.slice(0..1);
            }
            '\n' => {
                will_move_to_col = false;
                break;
            }
            _ => break,
        }
    }
    for char in line_slice.chars() {
        match char {
            ' ' => break,
            '\n' => {}
            _ => col += 1,
        }
    }
    if will_move_to_col {
        execute!(
            stdout(),
            cursor::MoveToColumn(main_struct.cursor_pos.0 + col)
        )
        .unwrap();
    } else {
        queue!(stdout(), cursor::MoveDown(1), cursor::MoveToColumn(0)).unwrap();
    }
}

pub fn cursor_back(main_struct: &mut AugeliteState) {
    if main_struct.cursor_pos.0 == 0 {
        cursor_left(main_struct);
    } else {
        let text = main_struct.buffer.clone().finish();
        let line = text.line(main_struct.cursor_pos.1 as usize);
        let end = main_struct.cursor_pos.0 as usize;
        let line = line.slice(0..end);
        let mut col: u16 = 0;
        for char in line.to_string().chars().rev() {
            if char != ' ' {
                col += 1;
            } else {
                break;
            }
        }

        execute!(stdout(), MoveLeft(col)).unwrap();
    }
}

pub fn cursor_max_col(main_struct: &mut AugeliteState) {
    let text = main_struct.buffer.clone().finish();
    queue!(stdout(), cursor::MoveToColumn(text.line(main_struct.cursor_pos.1.into()).len_chars() as u16), cursor::MoveLeft(1)).unwrap();
}
