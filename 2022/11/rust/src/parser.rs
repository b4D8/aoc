use {
    super::{Monkey, MonkeyId, Operand, Operation, Test, WorryLevel},
    nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, digit1, multispace0, space0},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::{delimited, pair, preceded, terminated, tuple},
        IResult,
    },
};

type ParseResult<'a, T> = IResult<&'a str, T, ()>;

fn unsigned_digit1(s: &str) -> ParseResult<usize> {
    map_res(digit1, str::parse)(s)
}

fn parse_worries(s: &str) -> ParseResult<Vec<WorryLevel>> {
    preceded(
        pair(tag("Starting items:"), space0),
        separated_list1(pair(tag(","), space0), map(unsigned_digit1, WorryLevel)),
    )(s)
}

fn parse_operation(s: &str) -> ParseResult<Operation> {
    map(
        tuple((
            delimited(
                pair(tag("Operation: new ="), space0),
                map(tag("old"), |_| None),
                space0,
            ),
            terminated(alt((char('+'), char('*'))), space0),
            terminated(
                alt((map(unsigned_digit1, Some), map(tag("old"), |_| None))),
                space0,
            ),
        )),
        |(rhs, op, lhs)| match op {
            '*' => Operation::Mul(lhs, rhs),
            '+' => Operation::Add(lhs, rhs),
            _ => unreachable!(),
        },
    )(s)
}

fn parse_test(s: &str) -> ParseResult<Test> {
    map(
        tuple((
            delimited(
                pair(tag("Test: divisible by"), space0),
                unsigned_digit1,
                multispace0,
            ),
            delimited(
                pair(tag("If true: throw to monkey"), space0),
                map(unsigned_digit1, MonkeyId),
                multispace0,
            ),
            delimited(
                pair(tag("If false: throw to monkey"), space0),
                map(unsigned_digit1, MonkeyId),
                multispace0,
            ),
        )),
        |(div, tru, fals)| Test(div, tru, fals),
    )(s)
}

pub(crate) fn parse_monkey(s: &str) -> ParseResult<(MonkeyId, Monkey)> {
    let (s, id) = delimited(
        multispace0,
        delimited(
            pair(tag("Monkey"), space0),
            map(unsigned_digit1, MonkeyId),
            tag(":"),
        ),
        multispace0,
    )(s)?;
    let (s, worries) = terminated(parse_worries, multispace0)(s)?;
    let (s, operation) = terminated(parse_operation, multispace0)(s)?;
    let (s, test) = terminated(parse_test, multispace0)(s)?;
    Ok((
        s,
        (
            id,
            Monkey {
                worries: worries.into(),
                operation,
                test,
                business: 0,
            },
        ),
    ))
}
