use {
    petgraph::{
        algo::dijkstra::dijkstra,
        prelude::{DiGraphMap, GraphMap},
        Directed,
    },
    std::iter,
    utils::{Direction, Grid, Point, Puzzle},
};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd, Eq, Ord)]
enum Square {
    Start,
    End,
    Other(char),
}

impl std::str::FromStr for Square {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().next() {
            if c.is_ascii_lowercase() {
                Ok(Self::Other(c))
            } else {
                match c {
                    'S' => Ok(Self::Start),
                    'E' => Ok(Self::End),
                    _ => Err(()),
                }
            }
        } else {
            Err(())
        }
    }
}

impl Square {
    fn eval(self) -> usize {
        let level = match self {
            Self::Start => 'a',
            Self::End => 'z',
            Self::Other(z) => z,
        };
        level as usize % 'a' as usize
    }

    fn can_reach(self, destination: Square) -> bool {
        destination.eval() <= self.eval() + 1
    }

    fn is_lowest(self) -> bool {
        self.eval() == 0
    }
}

#[derive(Debug, Clone)]
struct Day12(Grid<Square>);

impl std::str::FromStr for Day12 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .filter(|row| !row.is_empty())
            .map(|row| {
                row.split("")
                    .filter(|cell| !cell.is_empty())
                    .map(|cell| cell.parse::<Square>().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<Square>>>();
        Ok(Self(grid.into()))
    }
}

type Position = (Point, Square);

type Path = (Position, Position);

impl Day12 {
    fn start(&self) -> Position {
        self.0
            .points_values()
            .into_iter()
            .find_map(|position| matches!(position.1, Square::Start).then_some(position))
            .unwrap()
    }

    fn end(&self) -> Position {
        self.0
            .points_values()
            .into_iter()
            .find_map(|position| matches!(position.1, Square::End).then_some(position))
            .unwrap()
    }

    fn get(&self, point: Point) -> Option<Position> {
        self.0.get(point).map(|square| (point, *square))
    }

    fn neighbours(&self, (point, square): Position) -> Vec<Position> {
        point
            .orth_neighbours(Direction::North)
            .into_iter()
            .filter_map(|candidate| {
                self.get(candidate)
                    .and_then(|neighbour| square.can_reach(neighbour.1).then_some(neighbour))
            })
            .collect()
    }

    fn paths(&self, reverse: bool) -> Vec<Path> {
        self.0
            .points_values()
            .into_iter()
            .flat_map(|position| iter::repeat(position).zip(self.neighbours(position)))
            .map(|(from, to)| if reverse { (to, from) } else { (from, to) })
            .collect()
    }

    fn graph(&self, reverse: bool) -> GraphMap<Position, (), Directed> {
        DiGraphMap::from_edges(&self.paths(reverse))
    }
}

impl Puzzle for Day12 {
    fn solve1(&self) -> usize {
        *dijkstra(&self.graph(false), self.start(), Some(self.end()), |_| 1)
            .get(&self.end())
            .unwrap() as usize
    }

    fn solve2(&self) -> usize {
        dijkstra(&self.graph(true), self.end(), Some(self.start()), |_| 1)
            .into_iter()
            .filter_map(|((_, square), cost)| square.is_lowest().then_some(cost))
            .min()
            .unwrap() as usize
    }
}

fn main() {
    let puzzle = Day12::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 437);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 430);
}
