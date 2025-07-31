use std::io::stdout;

use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};

use super::model::AugeliteState;

pub fn scroll_down(main_struct: &mut AugeliteState) {
    execute!(stdout(), cursor::SavePosition).unwrap();
    execute!(stdout(), cursor::Hide).unwrap();
    execute!(stdout(), cursor::MoveToRow(terminal::size().unwrap().1 - 1)).unwrap();
    execute!(stdout(), terminal::Clear(ClearType::CurrentLine)).unwrap();
    execute!(stdout(), terminal::ScrollUp(1)).unwrap();
    execute!(stdout(), cursor::RestorePosition).unwrap();
    execute!(stdout(), cursor::MoveUp(1)).unwrap();
    execute!(stdout(), cursor::Show).unwrap();
    main_struct.scroll_offset += 1;
}

pub fn scroll_up(main_struct: &mut AugeliteState) {
    execute!(stdout(), cursor::SavePosition).unwrap();
    execute!(stdout(), cursor::Hide).unwrap();
    execute!(stdout(), cursor::MoveToRow(terminal::size().unwrap().1 - 1)).unwrap();
    execute!(stdout(), terminal::Clear(ClearType::CurrentLine)).unwrap();
    execute!(stdout(), terminal::ScrollDown(1)).unwrap();
    execute!(stdout(), cursor::RestorePosition).unwrap();
    execute!(stdout(), cursor::MoveDown(1)).unwrap();
    execute!(stdout(), cursor::Show).unwrap();
    main_struct.scroll_offset -= 1;
}
