use {std::collections::HashSet, utils::Puzzle};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
struct Day6 {
    stream: Vec<char>,
}

impl Day6 {
    fn detect(&self, size: usize) -> usize {
        self.stream
            .windows(size)
            .enumerate()
            .find_map(|(i, window)| {
                (window.iter().collect::<HashSet<&char>>().len() == size).then_some(i + size)
            })
            .unwrap()
    }
}

impl Puzzle for Day6 {
    fn from_string(s: String) -> Self {
        let stream = s.chars().filter(|c| c.is_alphanumeric()).collect();
        Self { stream }
    }

    fn solve1(&self) -> usize {
        self.detect(4)
    }

    fn solve2(&self) -> usize {
        self.detect(14)
    }
}

fn main() {
    let puzzle = Day6::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 1876);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 2202);
}
