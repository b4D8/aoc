use super::*;

const SAMPLE: &str = r#"
30373
25512
65332
33549
35390
"#;

#[test]
fn test_grid() {
    let grid = SAMPLE.parse::<Day8>().unwrap();
    println!("{}", grid);

    let mat: Day8 = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ]
    .into();
    assert_eq!(grid, mat);

    assert_eq!(grid.row(0), vec![3, 0, 3, 7, 3]);
    assert_eq!(grid.col(0), vec![3, 2, 6, 3, 3]);
    assert_eq!(grid.get(2, 2), 3);

    assert_eq!(grid.rows(), 5);
    assert_eq!(grid.cols(), 5);
}

#[test]
fn test_part1() {
    let grid = SAMPLE.parse::<Day8>().unwrap();

    // outer
    assert_eq!(grid.is_visible(0, 1), true);
    assert_eq!(grid.is_visible(5, 1), true);
    assert_eq!(grid.is_visible(1, 5), true);
    assert_eq!(grid.is_visible(1, 0), true);

    // top
    assert_eq!(grid.get(1, 1), 5);
    assert_eq!(grid.is_visible(1, 1), true);

    assert_eq!(grid.get(1, 2), 5);
    assert_eq!(grid.is_visible(1, 2), true);

    assert_eq!(grid.get(1, 3), 1);
    assert_eq!(grid.is_visible(1, 3), false);

    // mid
    assert_eq!(grid.get(2, 1), 5);
    assert_eq!(grid.is_visible(2, 1), true);

    assert_eq!(grid.get(2, 2), 3);
    assert_eq!(grid.is_visible(2, 2), false);

    assert_eq!(grid.get(2, 3), 3);
    assert_eq!(grid.is_visible(2, 3), true);

    // bottom
    assert_eq!(grid.get(3, 1), 3);
    assert_eq!(grid.is_visible(3, 1), false);

    assert_eq!(grid.get(3, 2), 5);
    assert_eq!(grid.is_visible(3, 2), true);

    assert_eq!(grid.get(3, 3), 4);
    assert_eq!(grid.is_visible(3, 3), false);

    assert_eq!(grid.count_visible(), 21);
}

#[test]
fn test_part2() {
    let grid = SAMPLE.parse::<Day8>().unwrap();

    let dir = Direction::Outwards;
    assert_eq!(grid.get(1, 2), 5);
    assert_eq!(grid.trees(1, 2, View::North, dir), vec![3]);
    assert_eq!(grid.trees(1, 2, View::East, dir), vec![5, 2]);
    assert_eq!(grid.trees(1, 2, View::West, dir), vec![1, 2]);
    assert_eq!(grid.trees(1, 2, View::South, dir), vec![3, 5, 3]);
    assert_eq!(grid.viewing_distance(1, 2), vec![1, 1, 2, 2]);
    assert_eq!(grid.scenic_score(1, 2), 4);

    assert_eq!(grid.get(3, 2), 5);
    assert_eq!(grid.trees(3, 2, View::North, dir), vec![3, 5, 3]);
    assert_eq!(grid.trees(3, 2, View::East, dir), vec![3, 3]);
    assert_eq!(grid.trees(3, 2, View::West, dir), vec![4, 9]);
    assert_eq!(grid.trees(3, 2, View::South, dir), vec![3]);
    assert_eq!(grid.viewing_distance(3, 2), vec![2, 2, 2, 1]);
    assert_eq!(grid.best_scenic_score(), 8);
}
