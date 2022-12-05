use {
    nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, digit1, line_ending, space0, space1},
        combinator::{all_consuming, map, map_res, opt},
        multi::{many1, many_m_n},
        sequence::{delimited, pair, terminated, tuple},
        IResult,
    },
    utils::Puzzle,
};

type ParseResult<'a, T> = IResult<&'a str, T, ()>;

#[derive(Debug, Clone, PartialEq)]
struct Crate(char);

impl Crate {
    fn new(s: &str) -> Self {
        Self(s.chars().next().unwrap())
    }
}

#[derive(Debug, Clone)]
enum Slot {
    Empty,
    Full(Crate),
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
        match self {
            Crane::Mover9000 => from.drain(range).rev().collect(),
            Crane::Mover9001 => from.drain(range).collect(),
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
        let mut block = crane.lift(step.n, from);
        let to = self.cargo.get_mut(step.to).unwrap();
        to.append(&mut block);
    }

    fn first(&self) -> String {
        self.cargo
            .iter()
            .filter_map(|stack| stack.last().map(|c| c.0))
            .collect()
    }
}

fn transpose<T>(mat: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = mat.get(0).unwrap().len();
    let mut iters: Vec<_> = mat.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| -> Vec<T> { iters.iter_mut().map(|n| n.next().unwrap()).collect() })
        .collect()
}

fn parse_cargo(input: &str) -> ParseResult<Vec<Vec<Crate>>> {
    map(
        many1(terminated(
            many1(terminated(
                alt((
                    map(
                        delimited(char('['), map(alpha1, Crate::new), char(']')),
                        Slot::Full,
                    ),
                    map(many_m_n(3, 3, char(' ')), |_| Slot::Empty),
                )),
                opt(char(' ')),
            )),
            line_ending,
        )),
        |mat| -> Vec<Vec<Crate>> {
            transpose(mat)
                .into_iter()
                .map(|col| -> Vec<Crate> {
                    col.into_iter()
                        .filter_map(|slot| match slot {
                            Slot::Empty => None,
                            Slot::Full(c) => Some(c),
                        })
                        .rev()
                        .collect()
                })
                .collect()
        },
    )(input)
}

fn step_tag<'a>(t: &'static str) -> impl FnMut(&'a str) -> ParseResult<usize> {
    map_res(
        delimited(pair(tag(t), space1), digit1, space0),
        |val: &str| val.parse::<usize>(),
    )
}

fn parse_steps(input: &str) -> ParseResult<Vec<Step>> {
    map(
        many1(terminated(
            map(
                tuple((step_tag("move"), step_tag("from"), step_tag("to"))),
                |(n, from, to)| Step::new(n, from, to),
            ),
            line_ending,
        )),
        |mut steps| {
            steps.reverse();
            steps
        },
    )(input)
}

impl Puzzle<String, String> for Day5 {
    fn from_string(s: String) -> Self {
        all_consuming(map(
            tuple((
                delimited(
                    opt(line_ending),
                    parse_cargo,
                    many1(alt((digit1, space1, line_ending))),
                ),
                parse_steps,
            )),
            |(cargo, steps)| Day5 { cargo, steps },
        ))(&s)
        .unwrap()
        .1
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

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn test_parser() {
        let data = Day5::from_string(SAMPLE.into());
        assert_eq!(
            data,
            Day5 {
                cargo: vec![
                    vec![Crate::new("Z"), Crate::new("N")],
                    vec![Crate::new("M"), Crate::new("C"), Crate::new("D")],
                    vec![Crate::new("P")],
                ],
                steps: vec![
                    Step::new(1, 1, 2),
                    Step::new(2, 2, 1),
                    Step::new(3, 1, 3),
                    Step::new(1, 2, 1),
                ],
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day5::from_string(SAMPLE.into()).solve1(), "CMZ")
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day5::from_string(SAMPLE.into()).solve2(), "MCD")
    }
}
