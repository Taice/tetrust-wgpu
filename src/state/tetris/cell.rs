#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    #[default]
    Empty,
    Filled([f32; 3]),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }

    pub fn is_filled(&self) -> bool {
        !self.is_empty()
    }
}
