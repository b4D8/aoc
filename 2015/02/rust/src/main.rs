use utils::Puzzle;

struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl std::fmt::Debug for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}x{}", self.length, self.width, self.height)
    }
}

impl Present {
    fn sides(&self) -> [usize; 3] {
        [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
    }

    fn surface(&self) -> usize {
        self.sides().iter().fold(0, |acc, x| acc + 2 * x)
    }

    fn extra(&self) -> usize {
        *self.sides().iter().min().unwrap()
    }

    fn papper(&self) -> usize {
        self.surface() + self.extra()
    }

    fn dim(&self) -> [usize; 3] {
        let mut dim = [self.length, self.width, self.height];
        dim.sort();
        dim
    }

    fn ribbon(&self) -> usize {
        let dim = self.dim();
        let wrap = dim.first().unwrap() * 2 + dim.get(1).unwrap() * 2;
        let bow = dim.iter().fold(1, |acc, x| acc * *x);
        wrap + bow
    }
}

#[derive(Debug)]
struct Day2 {
    presents: Vec<Present>,
}

impl Puzzle<usize, usize> for Day2 {
    fn from_string(s: String) -> Self {
        let presents: Vec<Present> = s
            .lines()
            .map(|line| {
                let dim: Vec<usize> = line.split('x').map(|n| n.parse().unwrap()).collect();
                Present {
                    length: *dim.first().unwrap(),
                    width: *dim.get(1).unwrap(),
                    height: *dim.get(2).unwrap(),
                }
            })
            .collect();
        Self { presents }
    }

    fn solve1(&self) -> usize {
        self.presents
            .iter()
            .fold(0, move |acc, present| acc + present.papper())
    }

    fn solve2(&self) -> usize {
        self.presents
            .iter()
            .fold(0, move |acc, present| acc + present.ribbon())
    }
}

fn main() {
    let puzzle = Day2::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 1586300);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 3737498);
}
