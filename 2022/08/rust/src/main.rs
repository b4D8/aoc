use {std::iter::IntoIterator, utils::Puzzle};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
enum View {
    North,
    South,
    East,
    West,
}

const VIEWS: [View; 4] = [View::North, View::East, View::West, View::South];

#[derive(Clone, Copy)]
enum Direction {
    Inwards,
    Outwards,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Day8 {
    rows: usize,
    cols: usize,
    data: Vec<Vec<usize>>,
}

impl Day8 {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn transpose(mut self) -> Self {
        let len = self.rows();
        let mut iters: Vec<_> = self.data.into_iter().map(IntoIterator::into_iter).collect();
        self.data = (0..len)
            .map(|_| -> Vec<_> { iters.iter_mut().map(|n| n.next().unwrap()).collect() })
            .collect::<Vec<Vec<_>>>();
        self
    }

    fn get(&self, row: usize, col: usize) -> usize {
        *self.data.get(row).unwrap().get(col).unwrap()
    }

    fn row(&self, row: usize) -> Vec<usize> {
        self.data.get(row).unwrap().clone()
    }

    fn col(&self, col: usize) -> Vec<usize> {
        // FIXME: This is way too costly!
        self.clone().transpose().row(col)
    }

    fn is_outer(&self, row: usize, col: usize) -> bool {
        row == 0 || col == 0 || self.rows() == row || self.cols() == col
    }

    fn trees(&self, row: usize, col: usize, from: View, dir: Direction) -> Vec<usize> {
        match dir {
            Direction::Inwards => match from {
                View::North => self.col(col).drain(0..row).collect(),
                View::South => self.col(col).drain((row + 1)..self.rows()).rev().collect(),
                View::East => self.row(row).drain(0..col).collect(),
                View::West => self.row(row).drain((col + 1)..self.cols()).rev().collect(),
            },
            Direction::Outwards => match from {
                View::North => self.col(col).drain(0..row).rev().collect(),
                View::South => self.col(col).drain((row + 1)..self.rows()).collect(),
                View::East => self.row(row).drain(0..col).rev().collect(),
                View::West => self.row(row).drain((col + 1)..self.cols()).collect(),
            },
        }
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        if self.is_outer(row, col) {
            true
        } else {
            let height = self.get(row, col);
            VIEWS.into_iter().any(|view| {
                self.trees(row, col, view, Direction::Inwards)
                    .iter()
                    .all(|tree| tree < &height)
            })
        }
    }

    fn count_visible(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(row, cols)| -> Vec<usize> {
                cols.iter()
                    .enumerate()
                    .filter_map(|(col, height)| self.is_visible(row, col).then_some(*height))
                    .collect()
            })
            .count()
    }

    fn viewing_distance(&self, row: usize, col: usize) -> Vec<usize> {
        if self.is_outer(row, col) {
            vec![0, 0, 0, 0]
        } else {
            let height = self.get(row, col);
            VIEWS
                .into_iter()
                .map(|view| {
                    let mut trees_at_sight = 0;
                    for tree in self.trees(row, col, view, Direction::Outwards) {
                        trees_at_sight += 1;
                        if tree >= height {
                            break;
                        }
                    }
                    trees_at_sight
                })
                .collect()
        }
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        self.viewing_distance(row, col).iter().product()
    }

    fn best_scenic_score(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(row, cols)| {
                cols.iter()
                    .enumerate()
                    .map(|(col, _)| self.scenic_score(row, col))
                    .collect::<Vec<usize>>()
            })
            .max()
            .unwrap()
    }
}

impl From<Vec<Vec<usize>>> for Day8 {
    fn from(data: Vec<Vec<usize>>) -> Self {
        let rows = data.len();
        let cols = data.get(0).unwrap().len();
        Self { rows, cols, data }
    }
}

impl std::str::FromStr for Day8 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(
            s.lines()
                .filter(|row| !row.is_empty())
                .map(|cols| {
                    cols.split("")
                        .filter(|col| !col.is_empty())
                        .map(|height| height.parse::<usize>().unwrap())
                        .collect()
                })
                .collect::<Vec<Vec<usize>>>(),
        ))
    }
}

impl std::fmt::Display for Day8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .fold(String::new(), |acc, row| acc + &format!("{:?},\n", row))
        )
    }
}

impl Puzzle for Day8 {
    fn from_string(s: String) -> Self {
        s.parse::<Self>().unwrap()
    }

    fn solve1(&self) -> usize {
        self.count_visible()
    }

    fn solve2(&self) -> usize {
        self.best_scenic_score()
    }
}

fn main() {
    let puzzle = Day8::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 1_829);

    let part1 = puzzle.solve2();
    println!("Part 2: answer is {}.", part1);
    assert_eq!(part1, 291_840);
}
