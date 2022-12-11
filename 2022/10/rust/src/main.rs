use utils::{Direction, Grid, Point, Puzzle, Step};

#[cfg(test)]
mod tests;

const CYCLES: usize = 240;
const ROWS: usize = 6;
const COLS: usize = CYCLES / ROWS;

#[derive(PartialEq, Debug, Clone, Default)]
enum Instruction {
    #[default]
    Noop,
    Addx(isize),
}

impl Instruction {
    fn todo(&self) -> Vec<Option<isize>> {
        let mut bare = vec![None];
        if let Self::Addx(ins) = self {
            bare.push(Some(*ins));
        }
        bare
    }
}

#[derive(PartialEq, Debug)]
struct Day10(Vec<Instruction>);

impl std::str::FromStr for Day10 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    if line.starts_with("addx") {
                        let (_, val) = line.split_once(' ').unwrap();
                        Instruction::Addx(val.parse::<isize>().unwrap())
                    } else {
                        Instruction::Noop
                    }
                })
                .collect(),
        ))
    }
}

impl Day10 {
    fn todo(&self) -> Vec<Option<isize>> {
        self.0
            .clone()
            .into_iter()
            .flat_map(|ins| ins.todo())
            .collect()
    }
}

#[derive(PartialEq, Copy, Clone, Default)]
enum Pixel {
    #[default]
    Dark,
    Lit,
}

impl std::fmt::Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Lit => '#',
                Self::Dark => '.',
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Sprite([Point; 3]);

impl Default for Sprite {
    fn default() -> Self {
        Self([
            Point::default(),
            Point::default() + Step(Direction::East, 1),
            Point::default() + Step(Direction::East, 2),
        ])
    }
}

impl Sprite {
    fn align(&mut self, register: isize, cycle: usize) {
        let row = (cycle / COLS) as isize;
        self.0[0] = Point(register - 1, row);
        self.0[1] = Point(register, row);
        self.0[2] = Point(register + 1, row);
    }
}

#[derive(PartialEq, Debug, Clone)]
struct CathodeRayTube(Grid<Pixel>);

impl Default for CathodeRayTube {
    fn default() -> Self {
        Self(Grid::new(COLS, ROWS))
    }
}

impl CathodeRayTube {
    #[allow(dead_code)]
    fn show_sprite(&self, sprite: &Sprite) {
        let mut crt = self.clone();
        crt.0.insert(sprite.0[0], Pixel::Lit);
        crt.0.insert(sprite.0[1], Pixel::Lit);
        crt.0.insert(sprite.0[2], Pixel::Lit);
        println!("Sprite position: {}\n{}", sprite.0[1].0, crt);
    }

    fn draw(&mut self, cycle: usize, sprite: &Sprite) {
        let cursor = Point((cycle % COLS) as isize, (cycle / COLS) as isize);
        if sprite.0.contains(&cursor) {
            self.0.insert(cursor, Pixel::Lit);
        }
    }
}

impl std::fmt::Display for CathodeRayTube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .chunks_exact(COLS)
                .into_iter()
                .map(|row| {
                    format!(
                        "{}\n",
                        row.iter()
                            .map(|col| format!("{:?}", col))
                            .collect::<String>()
                    )
                })
                .collect::<String>()
        )
    }
}

impl Puzzle<isize, String> for Day10 {
    fn solve1(&self) -> isize {
        self.todo()
            .into_iter()
            .try_fold(
                // as fold_while
                (0, 1, Vec::new()),
                |(mut cycle, mut register, mut terminal), todo| {
                    cycle += 1;
                    if cycle % COLS == 20 {
                        terminal.push(cycle as isize * register);
                        if cycle == CYCLES {
                            return Err((cycle, register, terminal));
                        }
                    }
                    if let Some(addx) = todo {
                        register += addx;
                    }
                    Ok((cycle, register, terminal))
                },
            )
            .unwrap_or_else(|v| v)
            .2
            .iter()
            .sum()
    }

    fn solve2(&self) -> String {
        let mut cycle = 0;
        let mut register = 1;
        let mut crt = CathodeRayTube::default();
        let mut sprite = Sprite::default();
        self.todo().into_iter().for_each(|todo| {
            crt.draw(cycle, &sprite);
            cycle += 1;
            if let Some(addx) = todo {
                register += addx;
            }
            sprite.align(register, cycle);
        });
        format!("\n{}", crt)
    }
}

fn main() {
    let puzzle = Day10::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 12_520);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(
        part2,
        r#"
####.#..#.###..####.###....##..##..#....
#....#..#.#..#....#.#..#....#.#..#.#....
###..####.#..#...#..#..#....#.#....#....
#....#..#.###...#...###.....#.#.##.#....
#....#..#.#....#....#....#..#.#..#.#....
####.#..#.#....####.#.....##...###.####.
"#
    ); // EHPZPJGL
}
