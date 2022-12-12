use super::*;

const SAMPLES: [&str; 2] = [
    r#"
noop
addx 3
addx -5"#,
    r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#,
];

#[test]
fn test_parser() {
    assert_eq!(
        SAMPLES[0].parse::<Day10>().unwrap(),
        Day10(vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ])
    );
}

#[test]
fn test_todo() {
    assert_eq!(
        SAMPLES[0].parse::<Day10>().unwrap().todo(),
        vec![None, None, Some(3), None, Some(-5),]
    );
}

#[test]
fn test_part1() {
    assert_eq!(SAMPLES[0].parse::<Day10>().unwrap().solve1(), 0);

    //vec![420, 1_140, 1_800, 2_940, 2_880, 3_960]
    assert_eq!(SAMPLES[1].parse::<Day10>().unwrap().solve1(), 13_140);
}

#[test]
fn test_part2() {
    assert_eq!(
        SAMPLES[1].parse::<Day10>().unwrap().solve2(),
        r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
    );
}
