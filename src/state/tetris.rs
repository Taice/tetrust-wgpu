pub mod action;
pub mod bag;
pub mod board;
pub mod cell;
pub mod point;
pub mod tetromino;

use action::Action;
use bag::Bag;
use board::Board;
use cell::Cell;
use point::Point;
use tetromino::{Tetromino, tetromino_kind::TetrominoKind};

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const FALL_TIME: u64 = 1000;
const AUTOPLAY_SPEED: u64 = 10;

#[derive(Debug, Clone)]
pub struct Tetris {
    pub board: Board,
    pub tetro: Tetromino,
    pub bag: Bag,

    moved: bool,
    hold: Option<TetrominoKind>,
    fall_timer: Instant,
    lines: u32,

    autoplay: Option<(Vec<Action>, Instant)>,
}

impl Default for Tetris {
    fn default() -> Self {
        Self {
            board: Board::default(),
            tetro: Tetromino::default(),
            bag: Bag::default(),
            fall_timer: Instant::now(),
            moved: false,
            hold: None,
            autoplay: None,
            lines: 0,
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

    fn autoplay(&mut self) -> u32 {
        loop {
            let actions = self.get_autoplay();
            for action in actions {
                if let Some(lines) = self.process_action(action) {
                    return lines;
                }
            }
        }
    }

    fn get_avg(&self) -> f32 {
        let mut avg = 0.0;
        let mut new = self.clone();
        for x in 0..100 {
            scores.push(new.autoplay());
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

    /// returns lines cleared if reset
    pub fn hard_fall(&mut self) -> Option<u32> {
        self.tetro.anchor.y += self.hard_fall_tetro(None);
        self.finish()
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
            Some((self.get_autoplay(), Instant::now()))
        }
    }

    // get permutations for auto-play
    pub fn get_autoplay(&self) -> Vec<Action> {
        let mut final_vec = vec![];

        // let mut max = 0.;
        let mut max = f32::MIN;
        for rotation in 0..4 {
            let mut actions = Vec::new();
            let mut new = self.clone();
            new.rotate(rotation as f32 * std::f32::consts::FRAC_PI_2);
            actions.push(Action::Rotate(rotation * 90));

            while new.move_x(-1.0) {
                actions.push(Action::Move(-1));
            }
            actions.push(Action::Move(-1));
            new.tetro.anchor.x -= 1.0;
            while new.move_x(1.0) {
                actions.push(Action::Move(1));
                let mut new_actions = actions.clone();
                let mut new_new = new.clone();
                new_new.tetro.anchor.y += new_new.hard_fall_tetro(None);
                new_new.engrave();
                let grade = new_new.board.grade();
                if grade >= max {
                    max = grade;
                    new_actions.push(Action::HardDrop);
                    final_vec = new_actions;
                }
            }
        }

        final_vec.reverse();
        compile_actions(&final_vec)
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

    /// returns lines cleared if reset
    fn finish(&mut self) -> Option<u32> {
        self.engrave();
        self.tetro = Tetromino::from_kind(self.bag.next());
        if !self.is_valid(None) {
            println!("lines: {}", self.lines);
            let lines = self.lines;
            self.reset();
            self.lines = 0;
            return Some(lines);
        }
        self.fix_board();
        self.moved = false;
        None
    }

    fn fix_board(&mut self) {
        for i in 0..20 {
            let mut thing = true;
            for cell in self.board[i] {
                if cell.is_empty() {
                    thing = false;
                    break;
                }
            }
            if thing {
                self.lines += 1;
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
                        vec = self.get_autoplay();
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

    // returns the amount of lines if the game reset
    pub fn process_action(&mut self, action: Action) -> Option<u32> {
        match action {
            Action::Move(x) => {
                self.move_x(x as f32);
                None
            }
            Action::Rotate(degrees) => {
                self.rotate((degrees as f32).to_radians());
                None
            }
            Action::HardDrop => self.hard_fall(),
            Action::Reset => {
                self.reset();
                None
            }
            Action::Hold => {
                self.hold();
                None
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

        *board
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

fn compile_actions(actions: &Vec<Action>) -> Vec<Action> {
    let mut final_vec = vec![Action::HardDrop];
    let mut map = HashMap::new();
    for action in actions {
        if let Action::Move(x) = action {
            *map.entry(action).or_insert(1) += 1;
        }
    }
    let left_count = map.get(&Action::Move(-1)).unwrap_or(&0);
    let right_count = map.get(&Action::Move(1)).unwrap_or(&0);
    if *left_count > *right_count {
        for _ in 0..*left_count - *right_count {
            final_vec.push(Action::Move(-1));
        }
    } else if *right_count > *left_count {
        for _ in 0..*right_count - *left_count {
            final_vec.push(Action::Move(1));
        }
    }

    final_vec.push(*actions.last().unwrap());
    final_vec
}
