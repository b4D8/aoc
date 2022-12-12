use {
    std::collections::{BTreeMap, VecDeque},
    utils::Puzzle,
};

mod parser;
#[cfg(test)]
mod tests;

type Operand = Option<usize>;

#[derive(Debug, Clone)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

#[derive(Debug, Clone)]
struct Test(usize, MonkeyId, MonkeyId);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct MonkeyId(usize);

#[derive(Debug, Clone, PartialEq)]
struct WorryLevel(usize);

#[derive(Debug, Clone)]
struct Monkey {
    worries: VecDeque<WorryLevel>,
    operation: Operation,
    test: Test,
    business: usize,
}

#[derive(Debug, Clone)]
struct Day11 {
    monkeys: BTreeMap<MonkeyId, Monkey>,
}

impl std::str::FromStr for Day11 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = nom::multi::many1(parser::parse_monkey)(s)
            .unwrap()
            .1
            .into_iter()
            .fold(BTreeMap::new(), |mut acc, (key, value)| {
                acc.insert(key, value);
                acc
            });
        Ok(Self { monkeys })
    }
}

impl Day11 {
    fn play(&self, rounds: usize, worry_factor: Option<usize>, verbose: bool) -> Self {
        let mut game = self.clone();
        let factor = worry_factor.unwrap_or_else(|| {
            game.monkeys
                .iter()
                .map(|(_, monkey)| monkey.test.0)
                .product()
        });
        let players = game.monkeys.len();
        let rounds = rounds * players;
        let mut round = 0;

        while round < rounds {
            let current_player = MonkeyId(round % players);
            if verbose {
                println!();
                println!(
                    "===== Round {} - Monkey {} =====",
                    round / players + 1,
                    current_player.0
                );
            }

            if let Some(mut monkey) = game.monkeys.remove(&current_player) {
                while let Some(mut worry) = monkey.worries.pop_front() {
                    monkey.business += 1;
                    if verbose {
                        println!();
                        println!(
                            "Monkey inspects an worry with a worry level of {}.",
                            worry.0
                        );
                    }

                    worry.0 = match &monkey.operation {
                        Operation::Add(lhs, rhs) => {
                            let ans = lhs.unwrap_or(worry.0) + rhs.unwrap_or(worry.0);
                            if verbose {
                                println!("   Worry level is increased by {} to {}.", worry.0, ans);
                            }
                            ans
                        }
                        Operation::Mul(lhs, rhs) => {
                            let ans = lhs.unwrap_or(worry.0) * rhs.unwrap_or(worry.0);
                            if verbose {
                                println!("   Worry level is multiplied by {} to {}.", worry.0, ans);
                            }
                            ans
                        }
                    };

                    if worry_factor.is_some() {
                        worry.0 /= factor;
                    } else {
                        worry.0 %= factor;
                    };

                    let next_player = if worry.0 % monkey.test.0 == 0 {
                        if verbose {
                            println!("   Current worry level is divisible by {}.", monkey.test.0);
                        }
                        monkey.test.1
                    } else {
                        if verbose {
                            println!(
                                "   Current worry level is not divisible by {}.",
                                monkey.test.0
                            );
                        }
                        monkey.test.2
                    };
                    if verbose {
                        println!(
                            "   worry with worry level {} is thrown to monkey {}.",
                            worry.0, next_player.0
                        );
                    }
                    game.monkeys
                        .get_mut(&next_player)
                        .unwrap()
                        .worries
                        .push_back(worry);
                }

                game.monkeys.insert(current_player, monkey);
                if verbose {
                    println!();
                    game.monkeys.iter().for_each(|(id, monkey)| {
                        println!(
                            "Monkey {}: {:?}",
                            id.0,
                            monkey
                                .worries
                                .iter()
                                .map(|worry| &worry.0)
                                .collect::<Vec<&usize>>()
                        );
                    });
                }

                round += 1;
            }
        }
        game
    }

    fn score(&self) -> usize {
        let mut business = self
            .monkeys
            .iter()
            .map(|(_, monkey)| monkey.business)
            .collect::<Vec<usize>>();
        business.sort_by(|a, b| b.cmp(a));
        business.truncate(2);
        business.into_iter().product()
    }
}

impl Puzzle<usize, usize> for Day11 {
    fn solve1(&self) -> usize {
        self.play(20, Some(3), false).score()
    }

    fn solve2(&self) -> usize {
        self.play(10_000, None, false).score()
    }
}

fn main() {
    let puzzle = Day11::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 55944);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 15_117_269_860);
}
