use std::hash::Hash;
use std::ops::Add;

#[derive(Hash, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
pub fn euclid_distance(from: &Position, to: &Position) -> usize {
    let x_dist = (from.x - to.x).abs();
    let y_dist = (from.y - to.y).abs();
    (x_dist + y_dist) as usize
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl Add<&Position> for &Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Position> for Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
