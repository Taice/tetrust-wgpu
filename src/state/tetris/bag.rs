use rand::seq::SliceRandom;

use super::tetromino::tetromino_kind::TetrominoKind;

type SevenBag = [TetrominoKind; 7];

#[derive(Default, Debug, Clone, Copy)]
pub struct Bag {
    index: usize,
    bag: SevenBag,
}

impl Bag {
    pub fn new() -> Self {
        Self {
            index: 0,
            bag: get_bag(),
        }
    }

    pub fn next(&mut self) -> TetrominoKind {
        let prevt = self.bag[self.index];

        self.index += 1;
        if self.index >= 7 {
            self.bag.shuffle(&mut rand::rng());
            self.index = 0;
        }

        prevt
    }
}

fn get_bag() -> SevenBag {
    let mut bag = (0..7)
        .enumerate()
        .fold([TetrominoKind::default(); 7], |mut acc, (i, x)| {
            acc[i] = x.into();
            acc
        });
    bag.shuffle(&mut rand::rng());
    bag
}
