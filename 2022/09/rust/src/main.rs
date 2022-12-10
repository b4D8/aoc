use {
    utils::grid::{Direction, Point, SparseGrid, Step},
    utils::Puzzle,
};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
struct Day9(Vec<Step>);

impl std::str::FromStr for Day9 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let (direction, steps) = line.split_once(' ').unwrap();
                    let steps = steps.parse::<isize>().unwrap();
                    let direction = match direction {
                        "U" => Direction::North,
                        "D" => Direction::South,
                        "R" => Direction::East,
                        "L" => Direction::West,
                        _ => unreachable!(),
                    };
                    Step(direction, steps)
                })
                .collect(),
        ))
    }
}

impl Day9 {
    fn trace(&self, knots: usize) -> SparseGrid<usize> {
        assert!(knots > 1);

        let mut rope = Vec::with_capacity(knots);
        rope.resize_with(knots, Point::default);

        let mut trace = SparseGrid::new();
        trace += *rope.get(0).unwrap();

        self.0.iter().for_each(|Step(direction, distance)| {
            (0..*distance).for_each(|_step| {
                *rope.get_mut(0).unwrap() += Step(*direction, 1);

                (0..(knots - 1)).for_each(|follower| {
                    let head = rope.get(follower).unwrap();
                    let pred = rope.get(follower + 1).unwrap();
                    if head.distance(pred) > 1 {
                        let motion = pred.follow(head);
                        *rope.get_mut(follower + 1).unwrap() = motion;
                        if follower == knots - 2 {
                            trace += motion;
                        }
                    }
                });
            });
        });
        trace
    }
}

impl Puzzle for Day9 {
    fn solve1(&self) -> usize {
        self.trace(2).len()
    }

    fn solve2(&self) -> usize {
        // Visualize with: https://scristobal.github.io/rust-wasm-playground/
        self.trace(10).len()
    }
}

fn main() {
    let puzzle = Day9::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 6044);

    let part1 = puzzle.solve2();
    println!("Part 2: answer is {}.", part1);
    assert_eq!(part1, 2384);
}
