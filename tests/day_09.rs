use aoc2023::{self, read_data};

#[test]
fn test_day_09() {
    let data = read_data::<String, _>("data/day_09.txt").unwrap();
    let task_1 = aoc2023::day_09::day_09_1(&data);
    assert_eq!(task_1, 2174807968);
    let task_2 = aoc2023::day_09::day_09_2(&data);
    assert_eq!(task_2, 1208);
}
