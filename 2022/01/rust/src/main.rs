use utils::Puzzle;

struct Day1 {
    cal: Vec<usize>,
}

impl Day1 {
    fn desc_cum_sum(&self, len: usize) -> usize {
        {
            let mut cal: Vec<usize> = self.cal.clone();
            cal.sort_by(|a, b| b.cmp(a));
            cal.truncate(len);
            cal
        }
        .iter()
        .sum()
    }
}

impl Puzzle for Day1 {
    fn from_string(s: String) -> Self {
        let cal: Vec<usize> = s
            .split("\n\n")
            .map(|elf| {
                elf.lines().fold(0, |acc, cal| {
                    cal.parse::<usize>().unwrap().checked_add(acc).unwrap()
                })
            })
            .collect();

        Self { cal }
    }

    fn solve1(&self) -> usize {
        self.desc_cum_sum(1)
    }

    fn solve2(&self) -> usize {
        self.desc_cum_sum(3)
    }
}

fn main() {
    let puzzle = Day1::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 72070);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 211805);
}
