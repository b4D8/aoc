use super::*;

const SAMPLE: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

#[test]
fn test_part1() {
    assert_eq!(SAMPLE.parse::<Day12>().unwrap().solve1(), 31);
}

#[test]
fn test_part2() {
    assert_eq!(SAMPLE.parse::<Day12>().unwrap().solve2(), 29);
}
