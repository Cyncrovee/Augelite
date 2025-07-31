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

pub fn clear_terminal() {
    execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
}

pub fn print_content(main_struct: &mut AugeliteState, will_clear: bool) -> std::io::Result<()> {
    if will_clear {
        clear_terminal();
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

pub fn statusline(main_struct: &crate::AugeliteState) -> std::io::Result<()> {
    execute!(stdout(), cursor::SavePosition)?;
    to_col(1);
    to_row(terminal::size().unwrap().1 - 1);
    execute!(stdout(), terminal::Clear(ClearType::CurrentLine))?;
    match main_struct.mode {
        Mode::Ovr => print!("OVERVIEW"),
        Mode::Ins => print!("INSERT"),
    }
    print!(" | ");
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
