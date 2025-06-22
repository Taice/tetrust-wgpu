use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::state::tetris::cell::Cell;

const LINE_CLEAR_WEIGHT: f32 = 3.5;
const HEIGHT_DIFFERENCE_WEIGHT: f32 = 1.0;
const HEIGHT_WEIGHT: f32 = 1.5;
const HOLES_WEIGHT: f32 = 3.4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    pub line_clear_weight: f32,
    pub height_difference_weight: f32,
    pub height_weight: f32,
    pub holes_weight: f32,
    pub board: [[Cell; 10]; 20],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            line_clear_weight: LINE_CLEAR_WEIGHT,
            height_difference_weight: HEIGHT_DIFFERENCE_WEIGHT,
            height_weight: HEIGHT_WEIGHT,
            holes_weight: HOLES_WEIGHT,
            board: [[Cell::default(); 10]; 20],
        }
    }
}

impl Board {
    pub fn print_weights(&self) {
        println!("line_clear_weight: {}", self.line_clear_weight);
        println!("HDW: {}", self.height_difference_weight);
        println!("height_weight: {}", self.height_weight);
        println!("holes_weight: {}", self.holes_weight);
    }

    pub fn increase_weight(&mut self, i: u8, amount: f32) {
        match i {
            0 => self.line_clear_weight += amount,
            1 => self.height_difference_weight += amount,
            2 => self.height_weight += amount,
            3 => self.holes_weight += amount,
            _ => unreachable!(),
        }
    }

    /// + Kolk linow cleara move
    /// + avg height
    /// - lukne
    pub fn grade(&self) -> f32 {
        let holes = self.get_holes();
        let holes_grade = (holes).powf(HOLES_WEIGHT);
        let line_grade = (self.get_rows_cleared() as f32 * 2.).powf(LINE_CLEAR_WEIGHT);
        let heights = self.get_heights();
        let diff_grade = (self.get_diff()).powf(HEIGHT_DIFFERENCE_WEIGHT);
        let max_grade = (heights.0 as f32).powf(HEIGHT_WEIGHT);

        line_grade - diff_grade - max_grade - holes_grade
    }

    fn get_diff(&self) -> f32 {
        let mut avg = 0.0;
        for x in 0..10 {
            avg = (avg * x as f32 + self.get_height_col(x) as f32) / (x as f32 + 1.);
        }

        let mut diff = 0.0;
        for x in 0..10 {
            let height = self.get_height_col(x);
            diff += (height as f32 - avg).abs();
        }

        diff
    }

    fn get_holes(&self) -> f32 {
        let mut holes = 0.;
        for x in 0..10 {
            let height = 20 - self.get_height_col(x) as usize;
            for row in self[height..].iter() {
                if row[x].is_empty() {
                    holes += 1.;
                }
            }
        }
        holes
    }

    fn get_rows_cleared(&self) -> u8 {
        let mut row_count = 0;
        for row in **self {
            let mut is = true;
            for cell in row {
                if cell.is_empty() {
                    is = false;
                    break;
                }
            }
            if is {
                row_count += 1;
            }
        }
        row_count
    }

    fn get_height_col(&self, col: usize) -> u8 {
        let mut height = 20;
        for row in **self {
            if row[col].is_filled() {
                break;
            }
            height -= 1;
        }
        height
    }

    /// returns (max, min)
    fn get_heights(&self) -> (u8, u8) {
        let mut max = 0u8;
        let mut min = 0u8;
        for x in 0..10 {
            let height = self.get_height_col(x);
            if height > max {
                max = height;
            }
            if height < min || min == 0 {
                min = height;
            }
        }

        (max, min)
    }
}

impl Deref for Board {
    type Target = [[Cell; 10]; 20];
    fn deref(&self) -> &Self::Target {
        &self.board
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        for row in **self {
            for cell in row {
                if cell.is_empty() {
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
