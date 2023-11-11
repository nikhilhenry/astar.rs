use std::cmp::Ordering;
use std::ops::Add;

#[derive(Hash, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_valid(&self) -> bool {
        self.x >= 0 && self.x < 10 && self.y >= 0 && self.y < 10
    }
    pub(crate) fn new(x: i32, y: i32) -> Self {
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

impl Eq for Position {}

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd<Self> for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.y).then_with(|| self.y.cmp(&other.y))
    }
}
