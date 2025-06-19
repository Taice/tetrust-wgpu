use std::ops::Add;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T: Add<Output = T> + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + Copy> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
