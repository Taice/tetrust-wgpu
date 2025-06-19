pub mod bag;
pub mod cell;
pub mod point;
pub mod tetromino;

use bag::Bag;
use cell::Cell;
use point::Point;
use tetromino::Tetromino;

use std::{
    fmt::Display,
    time::{Duration, Instant},
};

const FALL_TIME: u64 = 1000;

#[derive(Debug, Clone, Copy)]
pub struct Tetris {
    pub board: [[Cell; 10]; 20],
    pub tetro: Tetromino,
    pub bag: Bag,

    fall_timer: Instant,
}

impl Default for Tetris {
    fn default() -> Self {
        Self {
            board: [[Cell::default(); 10]; 20],
            tetro: Tetromino::default(),
            bag: Bag::default(),
            fall_timer: Instant::now(),
        }
    }
}

impl Tetris {
    pub fn new() -> Self {
        let mut bag = Bag::new();
        let tetro = Tetromino::from_kind(bag.next());

        Self {
            bag,
            tetro,
            ..Default::default()
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        for x in [0.0, 1.0, -1.0] {
            let mut new = self.tetro;
            new.anchor.x += x;
            new.rotate(radians);

            if self.is_valid(Some(&new)) {
                self.tetro = new;
                break;
            }
        }
    }

    pub fn hard_fall(&mut self) {
        self.tetro.anchor.y += self.hard_fall_tetro(None);
        self.finish();
    }
    pub fn fall(&mut self) {
        if self.fall_tetro(None) {
            self.tetro.anchor.y += 1.0;
        } else {
            self.finish();
        }
    }

    pub fn move_left(&mut self) {
        self.tetro.anchor.x -= 1.0;
        if !self.is_valid(None) {
            self.tetro.anchor.x += 1.0;
        }
    }
    pub fn move_right(&mut self) {
        self.tetro.anchor.x += 1.0;
        if !self.is_valid(None) {
            self.tetro.anchor.x -= 1.0;
        }
    }

    /// This function checks where a tetromino would hard fall to and returns the amount of y you
    /// have to add to reach that point.
    fn hard_fall_tetro(&self, tetro: Option<&Tetromino>) -> f32 {
        let mut tro = *tetro.unwrap_or(&self.tetro);
        let mut diff = 0.0;
        while self.fall_tetro(Some(&tro)) {
            tro.anchor.y += 1.0;
            diff += 1.0;
        }

        diff
    }
    /// This function checks whether a tetromino can possibly fall without causing collision
    fn fall_tetro(&self, tetro: Option<&Tetromino>) -> bool {
        let mut tro = *tetro.unwrap_or(&self.tetro);
        tro.anchor.y += 1.0;
        self.is_valid(Some(&tro))
    }

    fn finish(&mut self) {
        self.engrave();
        self.tetro = Tetromino::from_kind(self.bag.next());
        if !self.is_valid(None) {
            self.reset();
        }
        self.fix_board();
    }

    fn fix_board(&mut self) {
        for i in 0..20 {
            let mut thing = true;
            for cell in self.board[i] {
                if cell == Cell::Empty {
                    thing = false;
                    break;
                }
            }
            if thing {
                let mut prev = [Cell::Empty; 10];
                for y in 0..=i {
                    std::mem::swap(&mut prev, &mut self.board[y]);
                }
            }
        }
    }

    fn engrave(&mut self) {
        for point in self
            .tetro
            .get_points_vec()
            .iter()
            .map(|x| Point::new(x.x as usize, x.y as usize))
        {
            self.board[point.y][point.x] = Cell::Filled(self.tetro.color);
        }
    }

    fn is_valid(&self, tetro: Option<&Tetromino>) -> bool {
        let points = if let Some(tetr) = tetro {
            tetr.get_points_vec()
        } else {
            self.tetro.get_points_vec()
        };

        for point in points {
            if !(0..10).contains(&point.x) || !(0..20).contains(&point.y) {
                return false;
            }
            if self.board[point.y as usize][point.x as usize] != Cell::Empty {
                return false;
            }
        }

        true
    }

    pub fn update(&mut self, soft: bool) -> bool {
        let time = FALL_TIME - if soft { 920 } else { 0 };
        if self.fall_timer.elapsed() > Duration::from_millis(time) {
            self.fall();
            self.fall_timer = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn get_full_board(&self) -> [[Cell; 10]; 20] {
        let mut board = self.board;

        let mut new_tetro = self.tetro;
        new_tetro.anchor.y += self.hard_fall_tetro(Some(&new_tetro));

        for point in new_tetro
            .get_points_vec()
            .iter()
            .map(|x| Point::new(x.x as usize, x.y as usize))
        {
            board[point.y][point.x] = Cell::Filled(self.tetro.color.map(|x| x + 0.2));
        }
        for point in self
            .tetro
            .get_points_vec()
            .iter()
            .map(|x| Point::new(x.x as usize, x.y as usize))
        {
            board[point.y][point.x] = Cell::Filled(self.tetro.color);
        }

        board
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Display for Tetris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        for row in self.board {
            for cell in row {
                if cell == Cell::Empty {
                    text += "  ";
                } else {
                    text += "██";
                }
            }
            text += "\n";
        }
        write!(f, "{}", text)
    }
}
