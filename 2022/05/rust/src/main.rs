use nom::Finish;
use utils::Puzzle;

type ParseResult<'a, T> = nom::IResult<&'a str, T, ()>;

#[derive(Debug, Clone, PartialEq)]
struct Crate(String);

impl Crate {
    #[cfg(test)]
    fn new(s: &str) -> Self {
        Self(s.to_owned())
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
    #[cfg(test)]
    fn new(n: usize, from: usize, to: usize) -> Self {
        Self { n, from, to }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Day5 {
    cargo: Vec<Vec<Crate>>,
    steps: Vec<Step>,
}

impl Day5 {
    fn top(&self) -> String {
        self.cargo
            .iter()
            .filter_map(|stack| stack.last().map(|a| a.0.clone()))
            .collect::<Vec<String>>()
            .join("")
    }
}

fn parse_crate(input: &str) -> ParseResult<Crate> {
    nom::sequence::delimited(
        nom::bytes::complete::tag("["),
        nom::combinator::map(nom::character::complete::alpha1, |s: &str| Crate(s.into())),
        nom::bytes::complete::tag("]"),
    )(input)
}

fn parse_slot(input: &str) -> ParseResult<Slot> {
    nom::branch::alt((
        nom::combinator::map(parse_crate, Slot::Full),
        nom::combinator::map(
            nom::multi::many_m_n(3, 3, nom::bytes::complete::tag(" ")),
            |_| Slot::Empty,
        ),
    ))(input)
}

fn transpose<T>(mat: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = mat[0].len();
    let mut cols: Vec<_> = mat.into_iter().map(|col| col.into_iter()).collect();
    (0..len)
        .map(|_| {
            cols.iter_mut()
                .map(|col| col.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_cargo(input: &str) -> ParseResult<Vec<Vec<Crate>>> {
    nom::combinator::map(
        nom::multi::many1(nom::sequence::terminated(
            nom::multi::many1(nom::sequence::terminated(
                parse_slot,
                nom::combinator::opt(nom::bytes::complete::tag(" ")),
            )),
            nom::character::complete::line_ending,
        )),
        |mat| -> Vec<Vec<Crate>> {
            transpose(mat)
                .into_iter()
                .map(|col| -> Vec<Crate> {
                    let mut col: Vec<Crate> = col
                        .into_iter()
                        .filter_map(|slot| match slot {
                            Slot::Empty => None,
                            Slot::Full(c) => Some(c),
                        })
                        .collect();
                    col.reverse();
                    col
                })
                .collect()
        },
    )(input)
}

fn step_tag<'a>(tag: &'static str) -> impl FnMut(&'a str) -> ParseResult<usize> {
    nom::combinator::map_res(
        nom::sequence::delimited(
            nom::sequence::pair(
                nom::bytes::complete::tag(tag),
                nom::character::complete::space1,
            ),
            nom::character::complete::digit1,
            nom::character::complete::space0,
        ),
        |val: &str| val.parse::<usize>(),
    )
}

fn parse_steps(input: &str) -> ParseResult<Vec<Step>> {
    nom::combinator::map(
        nom::multi::many1(nom::sequence::terminated(
            nom::combinator::map(
                nom::sequence::tuple((step_tag("move"), step_tag("from"), step_tag("to"))),
                |(n, from, to)| Step { n, from, to },
            ),
            nom::character::complete::line_ending,
        )),
        |mut steps| {
            steps.reverse();
            steps
        },
    )(input)
}

fn parse_day5(input: &str) -> ParseResult<Day5> {
    nom::combinator::map(
        nom::sequence::tuple((
            nom::sequence::delimited(
                nom::combinator::opt(nom::character::complete::line_ending),
                parse_cargo,
                nom::multi::many1(nom::branch::alt((
                    nom::character::complete::digit1,
                    nom::character::complete::space1,
                    nom::character::complete::line_ending,
                ))),
            ),
            parse_steps,
        )),
        |(cargo, steps)| Day5 { cargo, steps },
    )(input)
}

impl std::str::FromStr for Day5 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, day) = nom::combinator::all_consuming(parse_day5)(s).finish()?;
        Ok(day)
    }
}

impl Puzzle<String, String> for Day5 {
    fn from_string(s: String) -> Self {
        s.parse::<Day5>().unwrap()
    }

    fn solve1(&self) -> String {
        {
            let mut day = self.clone();
            while let Some(step) = day.steps.pop() {
                for _ in 0..step.n {
                    let obj = day.cargo.get_mut(step.from - 1).unwrap().pop().unwrap();
                    day.cargo.get_mut(step.to - 1).unwrap().push(obj)
                }
            }
            day
        }
        .top()
    }

    fn solve2(&self) -> String {
        {
            let mut day = self.clone();
            while let Some(step) = day.steps.pop() {
                let mut range = Vec::new();
                for _ in 0..step.n {
                    let obj = day.cargo.get_mut(step.from - 1).unwrap().pop().unwrap();
                    range.push(obj);
                }
                while let Some(obj) = range.pop() {
                    day.cargo.get_mut(step.to - 1).unwrap().push(obj)
                }
            }
            day
        }
        .top()
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
