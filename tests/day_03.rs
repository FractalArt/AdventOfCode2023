use aoc2023::{self, read_data};

#[test]
fn test_day_03() {
    let data = read_data::<String, _>("data/day_03.txt").unwrap();
    let task_1 = aoc2023::day_03::day_03_1(&data);
    assert_eq!(task_1, 553079);
    let task_2 = aoc2023::day_03::day_03_2(&data);
    assert_eq!(task_2, 84363105);
}
