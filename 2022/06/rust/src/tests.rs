use super::*;

const SAMPLE: [(&str, usize, usize); 5] = [
    ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
    ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
    ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
    ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
    ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
];

#[test]
fn test() {
    for (sample, part1, part2) in SAMPLE.into_iter() {
        let stream = Day6::from_string(sample.into());
        assert_eq!(stream.solve1(), part1);
        assert_eq!(stream.solve2(), part2);
    }
}
