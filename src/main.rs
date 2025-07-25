use std::io::stdout;

use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
};
use ropey::{Rope, RopeBuilder};

struct AugeliteState {
    editor_content: RopeBuilder,
    file_path: String,
}

fn main() -> std::io::Result<()> {
    execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    /*
    stdout()
        .execute(crossterm::terminal::DisableLineWrap)
        .unwrap();
    */
    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(stdout(), crossterm::cursor::Show).unwrap();
    AugeliteState::run(&mut AugeliteState {
        editor_content: RopeBuilder::new(),
        file_path: "".to_string(),
    });

    Ok(())
}

impl AugeliteState {
    fn run(&mut self) {
        to_col(0);
        to_row(0);
        loop {
            if let Event::Key(key) = event::read().unwrap() {
                if let KeyEventKind::Press = key.kind {
                    match key.code {
                        KeyCode::Char(c) => {
                            move_right();
                            self.editor_content.append(c.to_string().as_str());
                        }
                        KeyCode::Left => {
                            move_left();
                        }
                        KeyCode::Right => {
                            move_right();
                        }
                        KeyCode::Up => {
                            move_up();
                        }
                        KeyCode::Down => {
                            move_down();
                        }
                        KeyCode::Enter => {
                            self.editor_content.append("\n");
                            new_line();
                        }
                        KeyCode::Esc => {
                            stdout()
                                .execute(crossterm::terminal::LeaveAlternateScreen)
                                .unwrap();
                            break;
                        }
                        _ => {}
                    }
                    print_content(self.editor_content.clone().finish(), false).unwrap();
                }
            }
        }
    }
}

fn print_content(content: Rope, will_clear: bool) -> std::io::Result<()> {
    if will_clear {
        clear_terminal();
    }
    execute!(stdout(), crossterm::terminal::BeginSynchronizedUpdate)?;
    execute!(stdout(), cursor::SavePosition)?;
    execute!(stdout(), cursor::Hide)?;
    to_col(0);
    to_row(0);
    for line in content.lines() {
        print!("{}", line);
        to_col(0);
    }
    execute!(stdout(), cursor::RestorePosition)?;
    execute!(stdout(), crossterm::terminal::EndSynchronizedUpdate)?;
    execute!(stdout(), cursor::Show)?;

    Ok(())
}

fn to_col(col: u16) {
    execute!(stdout(), crossterm::cursor::MoveToColumn(col)).unwrap();
}

fn to_row(row: u16) {
    execute!(stdout(), crossterm::cursor::MoveToRow(row)).unwrap();
}

fn move_left() {
    execute!(stdout(), crossterm::cursor::MoveLeft(1)).unwrap();
}

fn move_right() {
    execute!(stdout(), crossterm::cursor::MoveRight(1)).unwrap();
}

fn move_up() {
    execute!(stdout(), crossterm::cursor::MoveUp(1)).unwrap();
}

fn move_down() {
    execute!(stdout(), crossterm::cursor::MoveDown(1)).unwrap();
}

fn new_line() {
    execute!(stdout(), crossterm::cursor::MoveToNextLine(1)).unwrap();
}

fn up_line() {
    execute!(stdout(), crossterm::cursor::MoveToPreviousLine(1)).unwrap();
}

fn clear_terminal() {
    execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
}
