use crossterm::cursor;

use crate::AugeliteState;

use super::misc::{
    check_target_col, down_line_one, move_down_one, move_left_one, move_right_one, move_up_one,
    to_col, up_line_one,
};

pub fn cursor_left(main_struct: &mut AugeliteState) {
    let text = main_struct.buffer.clone().finish();
    if main_struct.cursor_pos.0 == 0 && main_struct.cursor_pos.1 != 0 {
        if let Some(line) = text.lines().nth(main_struct.cursor_pos.1 as usize - 1) {
            up_line_one();
            to_col((line.len_chars()) as u16 - 1);
        }
    } else {
        move_left_one();
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
        down_line_one();
    }
    if text.line(main_struct.cursor_pos.1 as usize).len_chars() == main_struct.cursor_pos.0 as usize
    {
        will_move_right = false;
    }
    if will_move_right {
        move_right_one();
    }
    main_struct.target_col = cursor::position().unwrap().0.into();
}

pub fn cursor_up(main_struct: &mut AugeliteState) {
    move_up_one();
    to_col(
        main_struct
            .buffer
            .clone()
            .finish()
            .line(cursor::position().unwrap().1.into())
            .len_chars()
            .try_into()
            .unwrap(),
    );
    move_left_one();
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
    if text
        .lines()
        .nth(main_struct.cursor_pos.1 as usize + 1)
        .is_some()
    {
        move_down_one();
        to_col(
            main_struct
                .buffer
                .clone()
                .finish()
                .line(cursor::position().unwrap().1.into())
                .len_chars()
                .try_into()
                .unwrap(),
        );
        move_left_one();
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

pub fn cursor_word(main_struct: &mut AugeliteState) {
    let text = main_struct.buffer.clone().finish();
    let line = text.line(main_struct.cursor_pos.1 as usize);
    let start = main_struct.cursor_pos.0 as usize;
    let line_slice = line.slice(start..line.len_chars());
    let mut col: u16 = 0;
    for char in line_slice.chars() {
	if char == ' ' {
	    col += 1;
	    line_slice.slice(0..1);
	} else {
	    break;
	}
    }
    for char in line_slice.chars() {
        if char != ' ' {
            col += 1;
        } else {
	    break;
        }
    }
    to_col(main_struct.cursor_pos.0 + col);
}

pub fn cursor_max_col(main_struct: &mut AugeliteState) {
    let text = main_struct.buffer.clone().finish();
    to_col(text.line(main_struct.cursor_pos.1.into()).len_chars() as u16);
}
