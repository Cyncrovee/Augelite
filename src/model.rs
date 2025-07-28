pub enum Mode {
    Ovr,
    Ins,
}

pub(crate) struct AugeliteState {
    pub buffer: ropey::RopeBuilder,
    pub cursor_pos: (u16, u16),
    pub cursor_char: usize,
    pub target_col: usize,
    pub mode: Mode,
}
