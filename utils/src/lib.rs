use std::fmt::Debug;
use std::str::FromStr;
pub mod grid;

pub trait Puzzle<A = usize, B = usize> {
    const FILE: &'static str = "../input";

    fn from_string(s: String) -> Self
    where
        Self: FromStr,
        <Self as FromStr>::Err: Debug,
    {
        s.parse::<Self>().unwrap()
    }

    fn from_file() -> Self
    where
        Self: FromStr,
        <Self as FromStr>::Err: Debug,
        Self: Sized,
    {
        let data = {
            let mut file = std::fs::File::open(Self::FILE).unwrap();
            let mut data = String::new();
            let _ = std::io::Read::read_to_string(&mut file, &mut data).unwrap();
            data
        };

        Self::from_string(data)
    }

    fn solve1(&self) -> A;

    fn solve2(&self) -> B;
}
