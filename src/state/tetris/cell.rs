#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    #[default]
    Empty,
    Filled([f32; 3]),
}
