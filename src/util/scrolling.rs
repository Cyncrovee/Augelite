use std::io::stdout;

use crossterm::{
    cursor, queue, terminal::{self}
};

use super::{
    model::AugeliteState,
    view::{print_content, statusline},
};

pub fn scroll_down(main_struct: &mut AugeliteState) {
    if (main_struct.scroll_offset as usize)
        < main_struct.buffer.clone().finish().lines().count() - 1
    {
        queue!(stdout(), terminal::ScrollUp(1), cursor::MoveUp(1)).unwrap();
        statusline(main_struct).unwrap();
        main_struct.scroll_offset += 1;
        print_content(main_struct, false).unwrap();
    }
}

pub fn scroll_up(main_struct: &mut AugeliteState) {
    if main_struct.scroll_offset != 0 {
        queue!(stdout(), terminal::ScrollDown(1), cursor::MoveDown(1)).unwrap();
        statusline(main_struct).unwrap();
        main_struct.scroll_offset -= 1;
        print_content(main_struct, false).unwrap();
    }
}
