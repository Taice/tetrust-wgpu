#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Rotate(f32),
    HardDrop,
    Reset,
    Hold,
}
