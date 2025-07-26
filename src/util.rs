use std::io::stdout;

use crossterm::{cursor, execute};
use ropey::Rope;

pub fn clear_terminal() {
    execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
}

pub fn print_content(content: Rope, will_clear: bool) -> std::io::Result<()> {
    if will_clear {
        clear_terminal();
    }
    execute!(stdout(), crossterm::terminal::BeginSynchronizedUpdate)?;
    execute!(stdout(), cursor::SavePosition)?;
    execute!(stdout(), cursor::Hide)?;
    to_col(0);
    to_row(0);
    for line in content.lines() {
        print!("{line}");
        to_col(0);
    }
    execute!(stdout(), cursor::RestorePosition)?;
    execute!(stdout(), crossterm::terminal::EndSynchronizedUpdate)?;
    execute!(stdout(), cursor::Show)?;

    Ok(())
}

pub fn to_col(col: u16) {
    execute!(stdout(), crossterm::cursor::MoveToColumn(col)).unwrap();
}

pub fn to_row(row: u16) {
    execute!(stdout(), crossterm::cursor::MoveToRow(row)).unwrap();
}

pub fn move_left() {
    execute!(stdout(), crossterm::cursor::MoveLeft(1)).unwrap();
}

pub fn move_right() {
    execute!(stdout(), crossterm::cursor::MoveRight(1)).unwrap();
}

pub fn move_up() {
    execute!(stdout(), crossterm::cursor::MoveUp(1)).unwrap();
}

pub fn move_down() {
    execute!(stdout(), crossterm::cursor::MoveDown(1)).unwrap();
}

pub fn new_line() {
    execute!(stdout(), crossterm::cursor::MoveToNextLine(1)).unwrap();
}

pub fn up_line() {
    execute!(stdout(), crossterm::cursor::MoveToPreviousLine(1)).unwrap();
}
