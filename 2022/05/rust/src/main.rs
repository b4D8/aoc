mod parser;
use utils::Puzzle;

#[derive(Debug, Clone, PartialEq)]
struct Crate(char);

impl Crate {
    fn new(s: &str) -> Self {
        Self(s.chars().next().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Step {
    n: usize,
    from: usize,
    to: usize,
}

impl Step {
    fn new(n: usize, from: usize, to: usize) -> Self {
        Self {
            n,
            from: from - 1,
            to: to - 1,
        }
    }
}

#[derive(Debug, Clone)]
enum Crane {
    Mover9000,
    Mover9001,
}

impl Crane {
    fn lift(&self, n: usize, from: &mut Vec<Crate>) -> Vec<Crate> {
        let range = (from.len() - n)..;
        let block = from.drain(range);
        match self {
            Crane::Mover9000 => block.rev().collect(),
            Crane::Mover9001 => block.collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Day5 {
    cargo: Vec<Vec<Crate>>,
    steps: Vec<Step>,
}

impl Day5 {
    fn rearrange(&mut self, step: Step, crane: Crane) {
        let from = self.cargo.get_mut(step.from).unwrap();
        let iter = crane.lift(step.n, from);
        self.cargo.get_mut(step.to).unwrap().extend(iter);
    }

    fn first(&self) -> String {
        self.cargo
            .iter()
            .filter_map(|stack| stack.last().map(|c| c.0))
            .collect()
    }
}

impl Puzzle<String, String> for Day5 {
    fn from_string(s: String) -> Self {
        parser::parse_day5(&s).unwrap().1
    }

    fn solve1(&self) -> String {
        let mut ship = self.clone();
        while let Some(step) = ship.steps.pop() {
            ship.rearrange(step, Crane::Mover9000)
        }
        ship.first()
    }

    fn solve2(&self) -> String {
        let mut ship = self.clone();
        while let Some(step) = ship.steps.pop() {
            ship.rearrange(step, Crane::Mover9001)
        }
        ship.first()
    }
}

fn main() {
    let puzzle = Day5::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, "CVCWCRTVQ");

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, "CNSCZWLVT");
}
