#[derive(Default, Debug, Clone, Copy)]
pub enum TetrominoKind {
    #[default]
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

impl From<u8> for TetrominoKind {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::I,
            1 => Self::O,
            2 => Self::S,
            3 => Self::Z,
            4 => Self::J,
            5 => Self::L,
            6 => Self::T,
            _ => panic!("Cant convert to tetromino type"),
        }
    }
}
