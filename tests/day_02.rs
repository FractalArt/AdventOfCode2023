use aoc2023::{self, read_data};

#[test]
fn test_day_01() {
    let data = read_data::<String, _>("data/day_02.txt").unwrap();
    let task_1 = aoc2023::day_02::day_02_1(&data, 12, 13, 14);
    assert_eq!(task_1, 2505);
    let task_2 = aoc2023::day_02::day_02_2(&data);
    assert_eq!(task_2, 70265);
}
