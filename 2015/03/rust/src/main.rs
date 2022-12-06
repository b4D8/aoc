use utils::Puzzle;

#[derive(Debug, Clone)]
enum Move {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
struct House {
    x: isize,
    y: isize,
}

impl std::ops::AddAssign<Move> for House {
    fn add_assign(&mut self, rhs: Move) {
        use std::ops::SubAssign;
        match rhs {
            Move::North => {
                self.x.add_assign(1);
            }
            Move::South => {
                self.x.sub_assign(1);
            }
            Move::East => {
                self.y.add_assign(1);
            }
            Move::West => {
                self.y.sub_assign(1);
            }
        }
    }
}

#[derive(Debug)]
struct Day3 {
    moves: Vec<Move>,
}

impl Day3 {
    fn dispatch(&self, santas: usize) -> usize {
        assert!(santas > 0);
        let mut visitors = vec![House::default(); santas];
        let mut houses = std::collections::HashSet::new();
        let start = visitors.get(0).unwrap().clone();
        houses.insert(start);
        for (i, m) in self.moves.clone().into_iter().enumerate() {
            let index = i % santas;
            *visitors.get_mut(index).unwrap() += m;
            let visited = visitors.get(index).unwrap().clone();
            houses.insert(visited);
        }
        houses.len()
    }
}

impl Puzzle<usize, usize> for Day3 {
    fn from_string(s: String) -> Self {
        let moves: Vec<Move> = s
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Move::North),
                'v' => Some(Move::South),
                '>' => Some(Move::East),
                '<' => Some(Move::West),
                _ => None,
            })
            .collect();
        Self { moves }
    }

    fn solve1(&self) -> usize {
        self.dispatch(1)
    }

    fn solve2(&self) -> usize {
        self.dispatch(2)
    }
}

fn main() {
    let puzzle = Day3::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 2081);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 2341);
}
