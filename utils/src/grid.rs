use super::*;

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct Grid<T> {
    max: Point,
    values: Vec<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn cols(&self) -> usize {
        self.max.0 as usize + 1
    }

    pub fn rows(&self) -> usize {
        self.max.1 as usize + 1
    }

    pub fn min(&self) -> Point {
        Point::default()
    }

    pub fn max(&self) -> Point {
        self.max
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_outer(&self, Point(col, row): Point) -> bool {
        let Point(max_col, max_row) = self.max;
        row == 0 || col == 0 || col == max_col || row == max_row
    }

    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            max: Point(cols as isize - 1, rows as isize - 1),
            values: vec![T::default(); rows * cols],
        }
    }

    pub fn get(&self, Point(col, row): Point) -> Option<&T> {
        let cols = self.cols();
        ((row as usize) < self.rows() && (col as usize) < cols)
            .then(|| &self.values[col as usize + row as usize * cols])
    }

    pub fn get_mut(&mut self, Point(col, row): Point) -> Option<&mut T> {
        let cols = self.cols();
        ((row as usize) < self.rows() && (col as usize) < cols)
            .then(|| &mut self.values[col as usize + row as usize * cols])
    }

    pub fn get_row(&self, row: usize) -> Option<Vec<&T>> {
        println!("{} vs {}", row, self.rows());
        (row < self.rows()).then(|| {
            (0..self.cols())
                .map(move |col| self.get((col, row).into()).unwrap())
                .collect()
        })
    }

    pub fn get_col(&self, col: usize) -> Option<Vec<&T>> {
        (col < self.cols()).then(|| {
            (0..self.rows())
                .map(move |row| self.get((row, col).into()).unwrap())
                .collect()
        })
    }

    pub fn transpose(&self) -> Self {
        Self {
            max: self.max,
            values: Vec::from_iter((0..self.rows()).flat_map(move |row| {
                self.get_row(row)
                    .unwrap()
                    .into_iter()
                    .map(|value| value.clone())
            })),
        }
    }

    pub fn apply<F: FnMut(&T)>(&self, fun: F) {
        self.values.iter().for_each(fun);
    }

    pub fn apply_mut<F: FnMut(&mut T)>(&mut self, fun: F) {
        self.values.iter_mut().for_each(fun);
    }

    pub fn insert(&mut self, Point(row, col): Point, value: T) {
        self.get_mut(Point(row, col)).map(|cell| {
            *cell = value;
        });
    }

    pub fn insert_at(&mut self, index: usize, value: T) {
        self.values.insert(index, value)
    }
}

impl<T: Default + Clone + PartialEq> Grid<T> {
    pub fn empty(&self) -> usize {
        self.values.iter().filter(|v| **v == T::default()).count()
    }

    pub fn full(&self) -> usize {
        self.values.iter().filter(|v| **v != T::default()).count()
    }
}

impl<T: Default + Clone> From<SparseGrid<T>> for Grid<T> {
    fn from(sparse: SparseGrid<T>) -> Self {
        let Point(cols, rows) = sparse.max();
        let mut dense: Grid<T> = Self::new(cols as usize, rows as usize);
        for (point, value) in sparse.into_iter() {
            dense.insert(point, value);
        }
        dense
    }
}

impl From<(usize, usize)> for Point {
    fn from(s: (usize, usize)) -> Self {
        Self(s.0 as isize, s.1 as isize)
    }
}

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;
    type IntoIter = ::std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}
