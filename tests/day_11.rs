use aoc2023::{self, read_data};

#[test]
fn test_day_11() {
    let data = read_data::<String, _>("data/day_11.txt").unwrap();
    let task_1 = aoc2023::day_11::day_11(&data, 2);
    assert_eq!(task_1, 9723824);
    let task_2 = aoc2023::day_11::day_11(&data, 1_000_000);
    assert_eq!(task_2, 731244261352);
}
