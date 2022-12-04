use utils::Puzzle;

#[derive(Debug)]
struct Section(std::ops::RangeInclusive<usize>);

impl std::str::FromStr for Section {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elf = s
            .split_once('-')
            .map(|(a, b)| {
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                Section(a..=b)
            })
            .unwrap();
        Ok(elf)
    }
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.0.start() <= other.0.start() && self.0.end() >= other.0.end()
    }

    fn overlaps(&self, other: &Section) -> bool {
        self.0.start() <= other.0.end() && self.0.end() >= other.0.start()
    }
}

#[derive(Debug)]
struct Pair(Section, Section);

impl std::str::FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s
            .split_once(',')
            .map(|(a, b)| Pair(a.parse::<Section>().unwrap(), b.parse::<Section>().unwrap()))
            .unwrap();
        Ok(pair)
    }
}

impl Pair {
    fn has_contained(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn has_overlap(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

#[derive(Debug)]
struct Day4(Vec<Pair>);

impl Puzzle for Day4 {
    fn from_string(s: String) -> Self {
        let pairs: Vec<Pair> = s
            .lines()
            .map(|line| line.parse::<Pair>().unwrap())
            .collect();

        Self(pairs)
    }

    fn solve1(&self) -> usize {
        self.0.iter().filter(|pair| pair.has_contained()).count()
    }

    fn solve2(&self) -> usize {
        self.0.iter().filter(|pair| pair.has_overlap()).count()
    }
}

fn main() {
    let puzzle = Day4::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1,);
    assert_eq!(part1, 518);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2,);
    assert_eq!(part2, 909);
}
