pub mod action;
pub mod bag;
pub mod cell;
pub mod point;
pub mod tetromino;

use action::Action;
use bag::Bag;
use cell::Cell;
use point::Point;
use rand::Rng;
use tetromino::{Tetromino, tetromino_kind::TetrominoKind};

use std::{
    fmt::Display,
    time::{Duration, Instant},
};

const FALL_TIME: u64 = 1000;
const AUTOPLAY_SPEED: u64 = 10;

#[derive(Debug, Clone)]
pub struct Tetris {
    pub board: [[Cell; 10]; 20],
    pub tetro: Tetromino,
    pub bag: Bag,

    moved: bool,
    hold: Option<TetrominoKind>,
    fall_timer: Instant,

    autoplay: Option<(Vec<Action>, Instant)>,
}

impl Default for Tetris {
    fn default() -> Self {
        Self {
            board: [[Cell::default(); 10]; 20],
            tetro: Tetromino::default(),
            bag: Bag::default(),
            fall_timer: Instant::now(),
            moved: false,
            hold: None,
            autoplay: None,
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
        let mut new = self.tetro;
        new.rotate(radians);
        for y in [0.0, 1.0, -1.0] {
            for x in [0.0, 1.0, -1.0] {
                new.anchor.x += x;
                new.anchor.y += y;

                if self.is_valid(Some(&new)) {
                    self.tetro = new;
                    break;
                }
                new.anchor.x -= x;
                new.anchor.y -= y;
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

    pub fn move_x(&mut self, x: f32) -> bool {
        let before = self.fall_tetro(None);
        self.tetro.anchor.x += x;

        let is_valid = self.is_valid(None);
        if !is_valid {
            self.tetro.anchor.x -= x;
        }

        if before && !self.fall_tetro(None) {
            self.fall_timer = Instant::now();
        }
        is_valid
    }

    pub fn toggle_autoplay(&mut self) {
        self.autoplay = if self.autoplay.is_some() {
            None
        } else {
            Some((self.get_auto_play(), Instant::now()))
        }
    }

    // get permutations for auto-play
    pub fn get_auto_play(&self) -> Vec<Action> {
        let mut final_vec = vec![];

        // temp cause no grading algorithm yet
        let mut rng = rand::rng();
        // let mut max = 0.;
        let rotation_wanted = rng.random_range(0..4);
        let x_wanted = if rng.random_bool(0.5) { -1. } else { 1. };
        let index_wanted = rng.random_range(0..=2);
        for rotation in 0..4 {
            let mut actions = Vec::new();
            let mut new = self.clone();
            new.rotate(rotation as f32 * std::f32::consts::FRAC_PI_2);
            for _ in 0..rotation {
                actions.push(Action::Rotate(std::f32::consts::FRAC_PI_2));
            }

            let unc = new.tetro.anchor.x;
            for x in [-1.0, 1.0] {
                let mut new_vec = vec![];
                let mut i = 0;
                while new.move_x(x) {
                    // print all info for debugging
                    new.tetro.anchor.y += new.hard_fall_tetro(None);
                    new_vec.push(Action::Move(x));

                    if rotation == rotation_wanted && x == x_wanted && i == index_wanted {
                        new_vec.iter().for_each(|x| actions.push(*x));
                        actions.push(Action::HardDrop);
                        final_vec = actions.clone();
                    }

                    // let grade = new.grade();
                    // if grade > max {
                    //     max = grade;
                    //     final_vec = actions.clone();
                    // }
                    i += 1;
                }
                new.tetro.anchor.x = unc;
            }
        }

        final_vec.reverse();
        final_vec
    }

    fn grade(&self) -> f32 {
        todo!()
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
        self.moved = false;
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
                    (prev, self.board[y]) = (self.board[y], prev);
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

    /// returns true if something changed; signaling to the renderer that it needs to update
    pub fn update(&mut self, soft: bool) -> bool {
        let mut thing = false;
        if let Some((mut vec, mut timer)) = self.autoplay.clone() {
            if timer.elapsed() >= Duration::from_millis(AUTOPLAY_SPEED) {
                match vec.pop() {
                    Some(action) => {
                        self.process_action(action);
                    }
                    None => {
                        vec = self.get_auto_play();
                    }
                }
                timer = Instant::now();
            }
            self.autoplay = Some((vec, timer));
            thing = true;
        }

        let time = FALL_TIME
            - if soft && self.fall_tetro(None) {
                920
            } else {
                0
            };
        if self.fall_timer.elapsed() > Duration::from_millis(time) {
            self.fall();
            self.fall_timer = Instant::now();
            true
        } else {
            thing
        }
    }

    pub fn hold(&mut self) {
        if !self.moved {
            let kind = self.tetro.kind;

            if let Some(hold) = self.hold {
                self.tetro = Tetromino::from_kind(hold);
            } else {
                self.tetro = Tetromino::from_kind(self.bag.next());
            }

            self.hold = Some(kind);
            self.moved = true;
        }
    }

    pub fn process_action(&mut self, action: Action) {
        match action {
            Action::Move(x) => {
                self.move_x(x);
            }
            Action::Rotate(radians) => {
                self.rotate(radians);
            }
            Action::HardDrop => {
                self.hard_fall();
            }
            Action::Reset => {
                self.reset();
            }
            Action::Hold => {
                self.hold();
            }
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
