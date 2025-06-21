#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Move(f32),
    Rotate(f32),
    HardDrop,
    Reset,
    Hold,
}
