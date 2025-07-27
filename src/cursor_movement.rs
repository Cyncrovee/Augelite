use crossterm::cursor;

use crate::{util::{check_target_col, move_down, move_left, move_right, move_up, new_line, to_col, up_line}, AugeliteState};

pub fn cursor_left(main_struct: &mut AugeliteState) {
    let text = main_struct.buffer.clone().finish();
    if main_struct.cursor_pos.0 == 0 && main_struct.cursor_pos.1 != 0 {
        if let Some(line) =
            text.lines().nth(main_struct.cursor_pos.1 as usize - 1)
        {
            up_line();
            to_col((line.len_chars()) as u16 - 1);
        }
    } else {
        move_left();
    }
    main_struct.target_col = cursor::position().unwrap().0.into();
}

pub fn cursor_right(main_struct: &mut AugeliteState) {
    let mut will_move_right = true;
    let text = main_struct.buffer.clone().finish();
    if text.lines().nth(main_struct.cursor_pos.1 as usize + 1).is_some()
        && text
            .line(main_struct.cursor_pos.1 as usize)
            .char(main_struct.cursor_pos.0 as usize)
            == '\n'
    {
        will_move_right = false;
        new_line();
    }
    if text.line(main_struct.cursor_pos.1 as usize).len_chars()
        == main_struct.cursor_pos.0 as usize
    {
        will_move_right = false;
    }
    if will_move_right {
        move_right();
    }
    main_struct.target_col = cursor::position().unwrap().0.into();
}

pub fn cursor_up(main_struct: &mut AugeliteState) {
    move_up();
    to_col(
        main_struct.buffer
            .clone()
            .finish()
            .line(cursor::position().unwrap().1.into())
            .len_chars()
            .try_into()
            .unwrap(),
    );
    move_left();
    if main_struct.target_col != 0
        && check_target_col(
            main_struct.buffer.clone().finish(),
            cursor::position().unwrap().1.into(),
            main_struct.target_col,
        )
    {
        to_col(main_struct.target_col as u16);
    }
}

pub fn cursor_down(main_struct: &mut AugeliteState) {
    let text = main_struct.buffer.clone().finish();
    if text.lines().nth(main_struct.cursor_pos.1 as usize + 1).is_some() {
        move_down();
        to_col(
            main_struct.buffer
                .clone()
                .finish()
                .line(cursor::position().unwrap().1.into())
                .len_chars()
                .try_into()
                .unwrap(),
        );
        move_left();
        if main_struct.target_col != 0
            && check_target_col(
                main_struct.buffer.clone().finish(),
                (cursor::position().unwrap().1).into(),
                main_struct.target_col,
            )
        {
            to_col(main_struct.target_col as u16);
        }
    }
}
