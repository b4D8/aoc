use utils::Puzzle;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up = 1,
    Down = -1,
}

struct Day1 {
    moves: Vec<Move>,
}

impl Puzzle<isize, usize> for Day1 {
    fn from_string(s: String) -> Self {
        let moves: Vec<Move> = s
            .split("")
            .filter_map(|m| match m {
                "(" => Some(Move::Up),
                ")" => Some(Move::Down),
                _ => None,
            })
            .collect();
        Self { moves }
    }

    fn solve1(&self) -> isize {
        self.moves.iter().fold(0, move |acc, m| acc + *m as isize)
    }

    fn solve2(&self) -> usize {
        let mut floor = 0;
        let mut enter = 0;
        for (i, m) in self.moves.iter().enumerate() {
            floor += *m as isize;
            if floor == -1 {
                enter = i + 1;
                break;
            }
        }
        enter
    }
}

fn main() {
    let puzzle = Day1::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 280);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 1797);
}
