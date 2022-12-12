use super::*;

const SAMPLE: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

#[test]
fn test_parser() {
    println!("{:?}", SAMPLE.parse::<Day11>().unwrap());
}

#[test]
fn test_part1() {
    assert_eq!(SAMPLE.parse::<Day11>().unwrap().solve1(), 10_605);
}

#[test]
fn test_part2() {
    assert_eq!(SAMPLE.parse::<Day11>().unwrap().solve2(), 2_713_310_158);
}
