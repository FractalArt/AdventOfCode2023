use aoc2023::{self, read_data};

#[test]
fn test_day_10() {
    let data = read_data::<String, _>("data/day_10.txt").unwrap();
    let task_1 = aoc2023::day_10::day_10_1(&data);
    assert_eq!(task_1, 6800);
    let task_2 = aoc2023::day_10::day_10_2(&data);
    assert_eq!(task_2, 483);
}
