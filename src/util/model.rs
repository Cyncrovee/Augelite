pub enum Mode {
    Ovr,
    Ins,
}

pub struct AugeliteState {
    pub buffer: ropey::RopeBuilder,
    pub cursor_pos: (u16, u16),
    pub cursor_char: usize,
    pub target_col: usize,
    pub scroll_offset: u16,
    pub mode: Mode,
    pub file_path: Option<String>,
}
