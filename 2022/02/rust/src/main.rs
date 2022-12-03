use utils::Puzzle;

#[derive(Debug, thiserror::Error)]
pub enum DayTwoError {
    #[error("Failed to parse shape: {0}.")]
    Shape(String),

    #[error("Failed to get 2 shapes from: {0}.")]
    Game(String),
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl TryFrom<usize> for Shape {
    type Error = DayTwoError;

    fn try_from(u: usize) -> Result<Self, Self::Error> {
        match u {
            1 => Ok(Self::Rock),
            2 => Ok(Self::Paper),
            3 => Ok(Self::Scissor),
            _ => Err(DayTwoError::Shape(u.to_string())),
        }
    }
}

impl std::str::FromStr for Shape {
    type Err = DayTwoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            _ => Err(DayTwoError::Shape(s.into())),
        }
    }
}

impl std::ops::Add for &Shape {
    type Output = usize;

    fn add(self, rhs: Self) -> Self::Output {
        *self as usize + *rhs as usize
    }
}

impl std::ops::Add<&Outcome> for &Shape {
    type Output = usize;

    fn add(self, rhs: &Outcome) -> Self::Output {
        *self as usize + *rhs as usize
    }
}

impl From<&Shape> for Outcome {
    fn from(shape: &Shape) -> Self {
        match shape {
            Shape::Rock => Outcome::Loss,
            Shape::Paper => Outcome::Draw,
            Shape::Scissor => Outcome::Win,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
struct Game {
    predict: Shape,
    reco: Shape,
}

impl Game {
    fn new(predict: Shape, reco: Shape) -> Self {
        Self { predict, reco }
    }

    fn play_reco(&self) -> usize {
        match (&self.predict, &self.reco) {
            (Shape::Rock, Shape::Scissor) => &self.reco + &Outcome::Loss,
            (Shape::Rock, Shape::Paper) => &self.reco + &Outcome::Win,
            (Shape::Scissor, Shape::Paper) => &self.reco + &Outcome::Loss,
            (Shape::Scissor, Shape::Rock) => &self.reco + &Outcome::Win,
            (Shape::Paper, Shape::Rock) => &self.reco + &Outcome::Loss,
            (Shape::Paper, Shape::Scissor) => &self.reco + &Outcome::Win,
            _ => &self.reco + &Outcome::Draw,
        }
    }

    fn play_outcome(&self) -> usize {
        let score = Outcome::from(&self.reco);
        match (&self.predict, &score) {
            (reco, Outcome::Draw) => reco + &score,
            (Shape::Rock, Outcome::Loss) => &Shape::Scissor + &score,
            (Shape::Rock, _) => &Shape::Paper + &score,
            (Shape::Paper, Outcome::Loss) => &Shape::Rock + &score,
            (Shape::Paper, _) => &Shape::Scissor + &score,
            (Shape::Scissor, Outcome::Loss) => &Shape::Paper + &score,
            (Shape::Scissor, _) => &Shape::Rock + &score,
        }
    }
}

impl std::str::FromStr for Game {
    type Err = DayTwoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shapes: Vec<&str> = s.split(' ').collect();
        match shapes.len() {
            2 => Ok(Self::new(
                shapes[0].parse::<Shape>()?,
                shapes[1].parse::<Shape>()?,
            )),
            _ => Err(DayTwoError::Game(s.into())),
        }
    }
}

#[derive(Debug)]
struct Day2 {
    games: Vec<Game>,
}

impl Puzzle for Day2 {
    fn from_string(s: String) -> Self {
        let games: Vec<Game> = s
            .lines()
            .map(|line| line.parse::<Game>().unwrap())
            .collect();

        Self { games }
    }

    fn solve1(&self) -> usize {
        self.games
            .iter()
            .fold(0, |acc, game| acc + game.play_reco())
    }

    fn solve2(&self) -> usize {
        self.games
            .iter()
            .fold(0, |acc, game| acc + game.play_outcome())
    }
}

fn main() {
    let puzzle = Day2::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 13009);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 10398);
}
