pub mod tetromino_kind;

use tetromino_kind::TetrominoKind;

use super::point::Point;

#[derive(Default, Debug, Clone, Copy)]
pub struct Tetromino {
    pub points: [Point<f32>; 4],
    pub anchor: Point<f32>,
    pub color: [f32; 3],
}

impl Tetromino {
    pub fn from_kind(kind: TetrominoKind) -> Self {
        match kind {
            TetrominoKind::I => Self {
                points: [
                    Point::new(-1.5, -0.5),
                    Point::new(-0.5, -0.5),
                    Point::new(0.5, -0.5),
                    Point::new(1.5, -0.5),
                ],
                anchor: Point::new(4.5, 0.5),
                color: [0.19, 0.65, 0.80],
            },
            TetrominoKind::O => Self {
                points: [
                    Point::new(-0.5, -0.5),
                    Point::new(-0.5, 0.5),
                    Point::new(0.5, -0.5),
                    Point::new(0.5, 0.5),
                ],
                anchor: Point::new(4.5, 0.5),
                color: [0.80, 0.70, 0.03],
            },
            TetrominoKind::S => Self {
                points: [
                    Point::new(1., -1.),
                    Point::new(0., -1.),
                    Point::new(0., 0.),
                    Point::new(-1., 0.),
                ],
                anchor: Point::new(4., 0.),
                color: [0.26, 0.71, 0.26],
            },
            TetrominoKind::Z => Self {
                points: [
                    Point::new(-1., -1.),
                    Point::new(0., -1.),
                    Point::new(0., 0.),
                    Point::new(1., 0.),
                ],
                anchor: Point::new(4., 0.),
                color: [0.80, 0.13, 0.16],
            },
            TetrominoKind::J => Self {
                points: [
                    Point::new(-1., -1.),
                    Point::new(-1., 0.),
                    Point::new(0., 0.),
                    Point::new(1., 0.),
                ],
                anchor: Point::new(4., 0.),
                color: [0.35, 0.4, 0.68],
            },
            TetrominoKind::L => Self {
                points: [
                    Point::new(1., -1.),
                    Point::new(-1., 0.),
                    Point::new(0., 0.),
                    Point::new(1., 0.),
                ],
                anchor: Point::new(4., 0.),
                color: [0.80, 0.40, 0.10],
            },
            TetrominoKind::T => Self {
                points: [
                    Point::new(0., 1.),
                    Point::new(-1., 0.),
                    Point::new(0., 0.),
                    Point::new(1., 0.),
                ],
                anchor: Point::new(4., 0.),
                color: [0.68, 0.3, 0.61],
            },
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        let sin = radians.sin();
        let cos = radians.cos();

        for point in self.points.iter_mut() {
            let x = point.x;
            let y = point.y;

            point.x = ((x * cos - y * sin) * 10.).round() / 10.;
            point.y = ((x * sin + y * cos) * 10.).round() / 10.;
        }
    }

    pub fn get_points_vec(&self) -> Vec<Point<isize>> {
        self.points
            .iter()
            .map(|x| {
                Point::new(
                    (x.x + self.anchor.x) as isize,
                    (x.y + self.anchor.y) as usize as isize,
                )
            })
            .collect::<Vec<Point<isize>>>()
    }
}
