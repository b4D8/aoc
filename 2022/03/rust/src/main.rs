use utils::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Item(char);

impl Item {
    fn priority(&self) -> usize {
        match self.0.is_uppercase() {
            true => self.0 as usize - 64 + 26,
            false => self.0 as usize - 64 - 6 - 26,
        }
    }
}

#[derive(Debug, Clone)]
struct Compartiment(Vec<Item>);

impl Compartiment {
    fn contains(&self, item: &Item) -> bool {
        self.0.contains(item)
    }
}

impl std::str::FromStr for Compartiment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .split("")
            .filter_map(|item| item.chars().next().map(Item))
            .collect();
        Ok(Self(items))
    }
}

trait CommonItem {
    fn common_item(&self) -> Option<Item>;
}

#[derive(Debug, Clone)]
struct Rucksack(Compartiment, Compartiment);

impl Rucksack {
    fn contains(&self, item: &Item) -> bool {
        self.0.contains(item) | self.1.contains(item)
    }
}

impl CommonItem for Rucksack {
    fn common_item(&self) -> Option<Item> {
        self.0
             .0
            .iter()
            .find_map(|a| self.1 .0.contains(a).then_some(Item(a.0)))
    }
}

impl std::str::FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s.len() / 2;
        let (first, last) = s.split_at(mid);
        let first = first.parse::<Compartiment>()?;
        let last = last.parse::<Compartiment>()?;
        Ok(Self(first, last))
    }
}

#[derive(Debug, Clone)]
struct Group(Rucksack, Rucksack, Rucksack);

impl CommonItem for Group {
    fn common_item(&self) -> Option<Item> {
        self.0
             .0
             .0
            .iter()
            .chain(self.0 .1 .0.iter())
            .find_map(|a| (self.1.contains(a) && self.2.contains(a)).then_some(Item(a.0)))
    }
}

#[derive(Debug)]
struct Day3(Vec<Rucksack>);

impl Day3 {
    fn dispatch(&self) -> Vec<Group> {
        self.0
            .chunks_exact(3)
            .map(|item| Group(item[0].clone(), item[1].clone(), item[2].clone()))
            .collect()
    }
}

impl Puzzle for Day3 {
    fn from_string(s: String) -> Self {
        let rucksacks: Vec<Rucksack> = s
            .lines()
            .map(|line| line.parse::<Rucksack>().unwrap())
            .collect();

        Self(rucksacks)
    }

    fn solve1(&self) -> usize {
        self.0
            .iter()
            .map(|rucksack| rucksack.common_item().unwrap().priority())
            .sum()
    }

    fn solve2(&self) -> usize {
        self.dispatch()
            .iter()
            .map(|group| group.common_item().unwrap().priority())
            .sum()
    }
}

fn main() {
    let puzzle = Day3::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 7737);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 2697);
}
