use aoc2023::{self, read_data};

#[test]
fn test_day_01() {
    let data = read_data::<String, _>("data/day_01.txt").unwrap();
    let task_1 = aoc2023::day_01::day_01_1(&data);
    assert_eq!(task_1, 54304);
    let task_2 = aoc2023::day_01::day_01_2(&data);
    assert_eq!(task_2, 54418);
}
