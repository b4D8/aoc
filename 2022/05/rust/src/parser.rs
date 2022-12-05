use {
    crate::{Crate, Day5, Step},
    nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, digit1, line_ending, space0, space1},
        combinator::{all_consuming, map, map_res, opt},
        multi::{many1, many_m_n},
        sequence::{delimited, pair, terminated, tuple},
        IResult,
    },
};

type ParseResult<'a, T> = IResult<&'a str, T, ()>;

fn parse_crate(input: &str) -> ParseResult<Crate> {
    delimited(char('['), map(alpha1, Crate::new), char(']'))(input)
}

#[derive(Debug, Clone)]
enum Slot {
    Empty,
    Full(Crate),
}

fn parse_row(input: &str) -> ParseResult<Vec<Slot>> {
    many1(terminated(
        alt((
            map(parse_crate, Slot::Full),
            map(many_m_n(3, 3, char(' ')), |_| Slot::Empty),
        )),
        opt(char(' ')),
    ))(input)
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
        many1(terminated(parse_row, line_ending)),
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

pub(crate) fn parse_day5(input: &str) -> ParseResult<Day5> {
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
    ))(input)
}
