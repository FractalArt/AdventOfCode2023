use aoc2023::{self, read_data};

#[test]
fn test_day_05() {
    let data = read_data::<String, _>("data/day_05.txt").unwrap();
    let task_1 = aoc2023::day_05::day_05_1(&data);
    assert_eq!(task_1, 165788812);
    let task_2 = aoc2023::day_05::day_05_2(&data);
    assert_eq!(task_2, 1928058);
}
