#[derive(Debug, thiserror::Error)]
pub enum DayTwoError {
    #[error("Failed to parse shape: {0}.")]
    Shape(String),

    #[error("Failed to get 2 shapes from: {0}.")]
    Game(String),
}

#[derive(Debug, Clone)]
#[repr(usize)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
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

impl From<&Shape> for Outcome {
    fn from(shape: &Shape) -> Self {
        match shape {
            Shape::Rock => Outcome::Loss,
            Shape::Paper => Outcome::Draw,
            Shape::Scissor => Outcome::Win,
        }
    }
}

#[derive(Debug)]
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

    fn play(&self) -> usize {
        let score = match (&self.predict, &self.reco) {
            (Shape::Rock, Shape::Scissor) => Outcome::Loss,
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Scissor, Shape::Paper) => Outcome::Loss,
            (Shape::Scissor, Shape::Rock) => Outcome::Win,
            (Shape::Paper, Shape::Rock) => Outcome::Loss,
            (Shape::Paper, Shape::Scissor) => Outcome::Win,
            _ => Outcome::Draw,
        };
        self.reco.clone() as usize + score as usize
    }

    fn play_again(&self) -> usize {
        let score = Outcome::from(&self.reco);
        let reco = match (&self.predict, &score) {
            (Shape::Rock, Outcome::Loss) => Shape::Scissor,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Loss) => Shape::Rock,
            (Shape::Paper, Outcome::Win) => Shape::Scissor,
            (Shape::Scissor, Outcome::Loss) => Shape::Paper,
            (Shape::Scissor, Outcome::Win) => Shape::Rock,
            (other, Outcome::Draw) => other.clone(),
        };
        reco as usize + score as usize
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
struct DayTwo {
    games: Vec<Game>,
}

impl DayTwo {
    fn from_file(path: &str) -> Self {
        let data = {
            let mut file = std::fs::File::open(path).unwrap();
            let mut data = String::new();
            let _ = std::io::Read::read_to_string(&mut file, &mut data).unwrap();
            data
        };

        let games: Vec<Game> = data
            .lines()
            .map(|line| line.parse::<Game>().unwrap())
            .collect();

        Self { games }
    }

    fn play(&self) -> usize {
        self.games.iter().fold(0, |acc, game| acc + game.play())
    }

    fn play_again(&self) -> usize {
        self.games
            .iter()
            .fold(0, |acc, game| acc + game.play_again())
    }
}

fn main() {
    let day = DayTwo::from_file("../input");

    let part_one = day.play();
    println!("Part One answer is: {}.", part_one);
    assert_eq!(part_one, 13009);

    let part_two = day.play_again();
    println!("Part Two answer is: {}.", part_two);
    assert_eq!(part_two, 10398);
}
