use aoc2023::{self, read_data};

#[test]
fn test_day_13() {
    let data = read_data::<String, _>("data/day_13.txt").unwrap();
    let task_1 = aoc2023::day_13::day_13_1(&data);
    assert_eq!(task_1, 33047);
    //let task_2 = aoc2023::day_13::day_13_1(&data);
    //assert_eq!(task_2, 731244261352);
}
