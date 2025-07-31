use std::io::stdout;

use crossterm::execute;

pub fn to_col(col: u16) {
    execute!(stdout(), crossterm::cursor::MoveToColumn(col)).unwrap();
}

pub fn to_row(row: u16) {
    execute!(stdout(), crossterm::cursor::MoveToRow(row)).unwrap();
}

pub fn move_left_one() {
    execute!(stdout(), crossterm::cursor::MoveLeft(1)).unwrap();
}

pub fn move_right_one() {
    execute!(stdout(), crossterm::cursor::MoveRight(1)).unwrap();
}

pub fn move_up_one() {
    execute!(stdout(), crossterm::cursor::MoveUp(1)).unwrap();
}

pub fn move_down_one() {
    execute!(stdout(), crossterm::cursor::MoveDown(1)).unwrap();
}

pub fn down_line_one() {
    execute!(stdout(), crossterm::cursor::MoveToNextLine(1)).unwrap();
}

pub fn up_line_one() {
    execute!(stdout(), crossterm::cursor::MoveToPreviousLine(1)).unwrap();
}
