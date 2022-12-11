use {
    super::*,
    std::{
        collections::{btree_map::Entry, BTreeMap},
        usize,
    },
};

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct SparseGrid<T> {
    max: Point,
    min: Point,
    values: BTreeMap<Point, T>,
}

impl<T> SparseGrid<T> {
    pub fn new() -> Self {
        Self {
            max: Point::default(),
            min: Point::default(),
            values: BTreeMap::new(),
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
        self.values.entry(point)
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        self.values.get(point)
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        self.values.get_mut(point)
    }

    pub fn insert(&mut self, point: Point, value: T) -> Option<T> {
        self.update_limits(&point);
        self.values.insert(point, value)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn can_contain_point(&self, point: &Point) -> bool {
        point.0 > self.max.0 || point.1 > self.max.1 || point.0 < self.min.0 || point.1 < self.min.1
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.can_contain_point(point) && self.values.contains_key(point)
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

impl<T> IntoIterator for SparseGrid<T> {
    type Item = (Point, T);
    type IntoIter = std::collections::btree_map::IntoIter<Point, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
