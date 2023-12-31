use aoc2023::{self, read_data};

#[test]
fn test_day_04() {
    let data = read_data::<String, _>("data/day_04.txt").unwrap();
    let task_1 = aoc2023::day_04::day_04_1(&data);
    assert_eq!(task_1, 15268);
    let task_2 = aoc2023::day_04::day_04_2(&data);
    assert_eq!(task_2, 6283755);
}
