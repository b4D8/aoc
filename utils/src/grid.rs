use std::{
    cmp::{self, max},
    collections::{btree_map::Entry, BTreeMap},
    ops::{Add, Sub},
    usize,
};

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

pub const ORTH_DIRECTIONS: [OrthDirection; 4] = [
    OrthDirection::North,
    OrthDirection::East,
    OrthDirection::South,
    OrthDirection::West,
];

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct SparseGrid<T> {
    max: Point,
    min: Point,
    points: BTreeMap<Point, T>,
}

impl<T> SparseGrid<T> {
    pub fn new() -> Self {
        Self {
            max: Point::default(),
            min: Point::default(),
            points: BTreeMap::new(),
        }
    }

    pub fn min(&self) -> Point {
        self.min
    }

    pub fn max(&self) -> Point {
        self.max
    }

    pub fn update_limits(&mut self, point: &Point) {
        if point.0 > self.max.0 {
            self.max.0 = point.0;
        }
        if point.1 > self.max.1 {
            self.max.1 = point.1;
        }
        if point.0 < self.min.0 {
            self.min.0 = point.0;
        }
        if point.1 < self.min.1 {
            self.min.1 = point.1;
        }
    }

    pub fn entry(&mut self, point: Point) -> Entry<Point, T> {
        self.update_limits(&point);
        self.points.entry(point)
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        self.points.get(point)
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        self.points.get_mut(point)
    }

    pub fn insert(&mut self, point: Point, value: T) -> Option<T> {
        self.update_limits(&point);
        self.points.insert(point, value)
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    fn can_contain(&self, point: &Point) -> bool {
        point.0 > self.max.0 || point.1 > self.max.1 || point.0 < self.min.0 || point.1 < self.min.1
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.can_contain(point) && self.points.contains_key(point)
    }
}

impl<T: num_traits::NumAssign> SparseGrid<T> {
    pub fn increment(&mut self, point: Point) {
        self.entry(point)
            .and_modify(|t| *t += num_traits::one())
            .or_insert_with(num_traits::one);
    }
}

impl<T: num_traits::NumAssign> std::ops::AddAssign<Point> for SparseGrid<T> {
    fn add_assign(&mut self, rhs: Point) {
        self.entry(rhs)
            .and_modify(|t| *t += num_traits::one())
            .or_insert_with(num_traits::one);
    }
}

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

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum OrthDirection {
    North,
    East,
    South,
    West,
}

impl From<OrthDirection> for Direction {
    fn from(dir: OrthDirection) -> Self {
        match dir {
            OrthDirection::North => Direction::North,
            OrthDirection::East => Direction::East,
            OrthDirection::South => Direction::South,
            OrthDirection::West => Direction::West,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct Step(pub Direction, pub isize);

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct OrthStep(pub OrthDirection, pub isize);

impl From<OrthStep> for Step {
    fn from(step: OrthStep) -> Self {
        Self(step.0.into(), step.1)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default, PartialOrd, Ord)]
/// col (x) and row (y)
pub struct Point(pub isize, pub isize);

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

    pub fn follow(&self, head: &Point) -> Self {
        Self(
            self.0 + cmp::max(cmp::min(head.0 - self.0, 1), -1),
            self.1 + cmp::max(cmp::min(head.1 - self.1, 1), -1),
        )
    }
}

// Orthogonal variants (with only 4 directions)
impl Point {
    // L1 or Manhattan distance
    // 2 1 2
    // 1 0 1
    // 2 1 2
    pub fn orth_distance(&self, other: &Point) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs())
            .try_into()
            .unwrap()
    }

    pub fn orth_neighbour(&self, direction: OrthDirection) -> Self {
        *self + OrthStep(direction, 1)
    }

    // Clockwise collection of Points surrounding a Point starting at from
    pub fn orth_neighbours(&self, from: OrthDirection) -> Vec<Self> {
        let index = ORTH_DIRECTIONS.partition_point(|dir| *dir == from);
        let (after, before) = ORTH_DIRECTIONS.split_at(index - 1);
        [before, after]
            .concat()
            .iter()
            .map(|direction| {
                dbg!(direction);
                self.orth_neighbour(*direction)
            })
            .collect()
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
        *self = self.sub(rhs);
    }
}

impl std::ops::Add<OrthStep> for Point {
    type Output = Point;

    fn add(self, rhs: OrthStep) -> Self::Output {
        match rhs.0 {
            OrthDirection::North => Self(self.0, self.1 + rhs.1),
            OrthDirection::East => Self(self.0 + rhs.1, self.1),
            OrthDirection::South => Self(self.0, self.1 - rhs.1),
            OrthDirection::West => Self(self.0 - rhs.1, self.1),
        }
    }
}

impl std::ops::AddAssign<OrthStep> for Point {
    fn add_assign(&mut self, rhs: OrthStep) {
        *self = self.add(rhs);
    }
}

impl std::ops::Sub<OrthStep> for Point {
    type Output = Point;

    fn sub(self, rhs: OrthStep) -> Self::Output {
        match rhs.0 {
            OrthDirection::North => Self(self.0, self.1 - rhs.1),
            OrthDirection::East => Self(self.0 - rhs.1, self.1),
            OrthDirection::South => Self(self.0, self.1 + rhs.1),
            OrthDirection::West => Self(self.0 + rhs.1, self.1),
        }
    }
}

impl std::ops::SubAssign<OrthStep> for Point {
    fn sub_assign(&mut self, rhs: OrthStep) {
        *self = self.sub(rhs);
    }
}
