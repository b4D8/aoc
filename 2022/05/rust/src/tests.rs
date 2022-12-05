use super::*;

const SAMPLE: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

#[test]
fn test_parser() {
    assert_eq!(
        Day5::from_string(SAMPLE.into()),
        Day5 {
            cargo: vec![
                vec![Crate::new("Z"), Crate::new("N")],
                vec![Crate::new("M"), Crate::new("C"), Crate::new("D")],
                vec![Crate::new("P")],
            ],
            steps: vec![
                Step::new(1, 1, 2),
                Step::new(2, 2, 1),
                Step::new(3, 1, 3),
                Step::new(1, 2, 1),
            ],
        }
    );
}

#[test]
fn test_part1() {
    assert_eq!(Day5::from_string(SAMPLE.into()).solve1(), "CMZ")
}

#[test]
fn test_part2() {
    assert_eq!(Day5::from_string(SAMPLE.into()).solve2(), "MCD")
}
