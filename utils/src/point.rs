use {
    super::*,
    std::cmp::{self, max},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default, PartialOrd, Ord)]
///  ^
///  |
///  y
///  |
///  0---x--->
pub struct Point(
    // x (col)
    pub isize,
    // y (row)
    pub isize,
);

impl Point {
    // Chebyshev or Chess distance
    // 1 1 1
    // 1 0 1
    // 1 1 1
    pub fn distance(&self, other: &Point) -> usize {
        max((self.0 - other.0).abs(), (self.1 - other.1).abs())
            .try_into()
            .unwrap()
    }

    // L1 or Manhattan distance
    // 2 1 2
    // 1 0 1
    // 2 1 2
    pub fn orth_distance(&self, other: &Point) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs())
            .try_into()
            .unwrap()
    }

    pub fn neighbour(&self, direction: Direction) -> Self {
        *self + Step(direction, 1)
    }

    // Clockwise collection of Points surrounding a Point starting at from
    pub fn neighbours(&self, from: Direction) -> Vec<Self> {
        let index = DIRECTIONS.iter().position(|&dir| dir == from).unwrap();
        let (after, before) = DIRECTIONS.split_at(index + 1);
        [before, after]
            .concat()
            .iter()
            .map(|direction| self.neighbour(*direction))
            .collect()
    }

    pub fn orth_neighbour(&self, direction: Direction) -> Self {
        *self + Step(direction, 1)
    }

    // Clockwise collection of Points surrounding a Point starting at from
    pub fn orth_neighbours(&self, from: Direction) -> Vec<Self> {
        let directions: Vec<Direction> = match from {
            Direction::East | Direction::North | Direction::South | Direction::West => DIRECTIONS
                .into_iter()
                .enumerate()
                .filter_map(|(i, dir)| (i % 2 == 0).then_some(dir))
                .collect(),
            _ => DIRECTIONS
                .into_iter()
                .enumerate()
                .filter_map(|(i, dir)| (i % 2 == 1).then_some(dir))
                .collect(),
        };
        let index = directions.partition_point(|dir| *dir == from);
        let (after, before) = directions.split_at(index - 1);
        [before, after]
            .concat()
            .iter()
            .map(|direction| self.orth_neighbour(*direction))
            .collect()
    }

    pub fn follow(&self, point: &Point) -> Self {
        Self(
            self.0 + cmp::max(cmp::min(point.0 - self.0, 1), -1),
            self.1 + cmp::max(cmp::min(point.1 - self.1, 1), -1),
        )
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0), self.1.add(rhs.1))
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0.add_assign(rhs.0);
        self.1.add_assign(rhs.1);
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0), self.1.sub(rhs.1))
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.0.sub_assign(rhs.0);
        self.1.sub_assign(rhs.1);
    }
}

impl From<(usize, usize)> for Point {
    fn from(s: (usize, usize)) -> Self {
        Self(s.0 as isize, s.1 as isize)
    }
}
