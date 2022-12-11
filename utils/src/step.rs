use super::*;

pub const DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct Step(pub Direction, pub isize);

impl std::ops::Add<Step> for Point {
    type Output = Point;

    fn add(self, rhs: Step) -> Self::Output {
        match rhs.0 {
            Direction::North => Self(self.0, self.1 + rhs.1),
            Direction::NorthEast => Self(self.0 + rhs.1, self.1 + rhs.1),
            Direction::East => Self(self.0 + rhs.1, self.1),
            Direction::SouthEast => Self(self.0 + rhs.1, self.1 - rhs.1),
            Direction::South => Self(self.0, self.1 - rhs.1),
            Direction::SouthWest => Self(self.0 - rhs.1, self.1 - rhs.1),
            Direction::West => Self(self.0 - rhs.1, self.1),
            Direction::NorthWest => Self(self.0 - rhs.1, self.1 + rhs.1),
        }
    }
}

impl std::ops::AddAssign<Step> for Point {
    fn add_assign(&mut self, rhs: Step) {
        use std::ops::Add;

        *self = self.add(rhs);
    }
}

impl std::ops::Sub<Step> for Point {
    type Output = Point;

    fn sub(self, rhs: Step) -> Self::Output {
        match rhs.0 {
            Direction::North => Self(self.0, self.1 - rhs.1),
            Direction::NorthEast => Self(self.0 - rhs.1, self.1 - rhs.1),
            Direction::East => Self(self.0 - rhs.1, self.1),
            Direction::SouthEast => Self(self.0 - rhs.1, self.1 + rhs.1),
            Direction::South => Self(self.0, self.1 + rhs.1),
            Direction::SouthWest => Self(self.0 + rhs.1, self.1 + rhs.1),
            Direction::West => Self(self.0 + rhs.1, self.1),
            Direction::NorthWest => Self(self.0 + rhs.1, self.1 - rhs.1),
        }
    }
}

impl std::ops::SubAssign<Step> for Point {
    fn sub_assign(&mut self, rhs: Step) {
        use std::ops::Sub;

        *self = self.sub(rhs);
    }
}
