use std::io::stdout;

use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use ropey::Rope;

use crate::util::model::Mode;

use super::{
    misc::{to_col, to_row},
    model::AugeliteState,
};

pub fn print_content(main_struct: &mut AugeliteState, will_clear: bool) -> std::io::Result<()> {
    if will_clear {
        execute!(stdout(), crossterm::terminal::Clear(ClearType::UntilNewLine))?;
    }
    execute!(stdout(), crossterm::terminal::BeginSynchronizedUpdate)?;
    execute!(stdout(), cursor::SavePosition)?;
    execute!(stdout(), cursor::Hide)?;
    to_col(0);
    to_row(0);
    for line in main_struct
        .buffer
        .clone()
        .finish()
        .lines_at(main_struct.scroll_offset as usize)
    {
        print!("{line}");
        to_col(0);
    }
    execute!(stdout(), cursor::RestorePosition)?;
    execute!(stdout(), crossterm::terminal::EndSynchronizedUpdate)?;
    execute!(stdout(), cursor::Show)?;

    Ok(())
}

pub fn statusline(main_struct: &mut AugeliteState) -> std::io::Result<()> {
    execute!(stdout(), cursor::SavePosition)?;
    to_col(1);
    to_row(terminal::size().unwrap().1 - 2);
    execute!(stdout(), terminal::Clear(ClearType::CurrentLine))?;
    to_row(terminal::size().unwrap().1 - 1);
    execute!(stdout(), terminal::Clear(ClearType::CurrentLine))?;
    match main_struct.mode {
        Mode::Ovr => print!("OVERVIEW"),
        Mode::Ins => print!("INSERT"),
    }
    print!(" | ");
    if let Some(path) = main_struct.file_path.clone() {
        print!("{}", path);
        print!(" | ");
    }
    print!("{}", main_struct.cursor_pos.1 + main_struct.scroll_offset);
    print!(":");
    print!("{}", main_struct.cursor_pos.0);
    print!(" | ");
    print!("{}", main_struct.cursor_char);
    execute!(stdout(), cursor::RestorePosition)?;
    Ok(())
}

pub fn check_target_col(buffer: Rope, line_num: usize, target_col: usize) -> bool {
    buffer.line(line_num).get_char(target_col).is_some()
}

pub fn check_end_of_view(main_struct: &mut AugeliteState) -> bool {
    if main_struct.cursor_pos.1 >= terminal::size().unwrap().1 - 3 {
        true
    } else {
        false
    }
}
