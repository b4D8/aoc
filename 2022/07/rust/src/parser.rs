use {
    crate::{Command, Day7, Dest, Dir, Entry, File},
    nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete::{alpha1, char, newline, space0, space1},
        combinator::{all_consuming, map, map_res, opt},
        multi::{many1, separated_list0},
        sequence::{delimited, pair, preceded, separated_pair},
        Finish, IResult,
    },
};

type ParseResult<'a, T> = IResult<&'a str, T, ()>;

fn parse_dir_name(input: &str) -> ParseResult<Dir> {
    map(alpha1, |name: &str| Dir { name: name.into() })(input)
}

fn parse_cmd_cd(input: &str) -> ParseResult<Command> {
    delimited(
        pair(tag("cd"), space1),
        alt((
            map(tag("/"), |_| Command::Change(Dest::Root)),
            map(tag(".."), |_| Command::Change(Dest::Backward)),
            map(parse_dir_name, |dir| Command::Change(Dest::Forward(dir))),
        )),
        opt(newline),
    )(input)
}

fn parse_file_name(input: &str) -> ParseResult<&str> {
    take_while1(|c: char| c.is_alphabetic() || c == '.')(input)
}

fn parse_file_size(input: &str) -> ParseResult<usize> {
    map_res(nom::character::complete::digit1, |d: &str| {
        d.parse::<usize>()
    })(input)
}

fn parse_file(input: &str) -> ParseResult<Entry> {
    map(
        separated_pair(parse_file_size, char(' '), parse_file_name),
        |(size, name)| {
            Entry::File(File {
                size,
                _name: name.into(),
            })
        },
    )(input)
}

fn parse_dir(input: &str) -> ParseResult<Entry> {
    map(
        preceded(pair(tag("dir"), space0), parse_dir_name),
        Entry::Dir,
    )(input)
}

fn parse_cmd_ls(input: &str) -> ParseResult<Command> {
    map(
        preceded(
            pair(tag("ls"), newline),
            separated_list0(newline, alt((parse_file, parse_dir))),
        ),
        Command::List,
    )(input)
}

fn parse_cmd(input: &str) -> ParseResult<Command> {
    delimited(
        pair(tag("$"), space1),
        alt((parse_cmd_cd, parse_cmd_ls)),
        opt(newline),
    )(input)
}

impl std::str::FromStr for Day7 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, cmd) = all_consuming(preceded(opt(newline), many1(parse_cmd)))(s).finish()?;
        Ok(Self { cmd })
    }
}
