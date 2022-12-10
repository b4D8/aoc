use super::*;

const SAMPLES: [&str; 2] = [
    r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#,
    r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#,
];

#[test]
fn test_parser() {
    assert_eq!(
        SAMPLES[0].parse::<Day9>().unwrap(),
        Day9(vec![
            Step(Direction::East, 4),
            Step(Direction::North, 4),
            Step(Direction::West, 3),
            Step(Direction::South, 1),
            Step(Direction::East, 4),
            Step(Direction::South, 1),
            Step(Direction::West, 5),
            Step(Direction::East, 2),
        ])
    );
}

#[test]
fn test_part1() {
    let mut day = Day9(Vec::new());

    let mut rope: SparseGrid<usize> = SparseGrid::new();
    rope += Point(0, 0);

    // R 4
    day.0.push(Step(Direction::East, 4));
    // ......
    // ......
    // ......
    // ......
    // s..TH.
    rope += Point(1, 0);
    rope += Point(2, 0);
    rope += Point(3, 0);
    assert_eq!(day.trace(2), rope);
    assert_eq!(day.trace(2).len(), 4);

    // U 4
    day.0.push(Step(Direction::North, 4));
    // ....H.
    // ....T.
    // ......
    // ......
    // s.....
    rope += Point(4, 1);
    rope += Point(4, 2);
    rope += Point(4, 3);
    assert_eq!(day.trace(2), rope);
    assert_eq!(day.trace(2).len(), 7);

    // L 3
    day.0.push(Step(Direction::West, 3));
    // .HT...
    // ......
    // ......
    // ......
    // s.....
    rope += Point(3, 4);
    rope += Point(2, 4);
    assert_eq!(day.trace(2), rope);
    assert_eq!(day.trace(2).len(), 9);

    // D 1
    day.0.push(Step(Direction::South, 1));
    // ..T...
    // .H....
    // ......
    // ......
    // s.....
    assert_eq!(day.trace(2), rope);
    assert_eq!(day.trace(2).len(), 9);

    // R 4
    day.0.push(Step(Direction::East, 4));
    // ......
    // ....TH
    // ......
    // ......
    // s.....
    rope += Point(3, 3);
    rope += Point(4, 3);
    assert_eq!(day.trace(2), rope);
    assert_eq!(day.trace(2).len(), 10);

    // D 1
    day.0.push(Step(Direction::South, 1));
    // ......
    // ....T.
    // .....H
    // ......
    // s.....
    assert_eq!(day.trace(2), rope);
    assert_eq!(day.trace(2).len(), 10);

    // ...

    day.0.push(Step(Direction::West, 5));
    day.0.push(Step(Direction::East, 2));
    assert_eq!(day.trace(2).len(), 13);

    assert_eq!(SAMPLES[0].parse::<Day9>().unwrap().solve1(), 13);
}

#[test]
fn test_part2() {
    let day = SAMPLES[0].parse::<Day9>().unwrap();
    assert_eq!(day.solve2(), 1);

    let day = SAMPLES[1].parse::<Day9>().unwrap();
    assert_eq!(day.solve2(), 36);
}
