#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Move(i32),
    Rotate(i32),
    HardDrop,
    Reset,
    Hold,
}
