use aoc2023::{self, read_data};

#[test]
fn test_day_14() {
    let data = read_data::<String, _>("data/day_14.txt").unwrap();
    let task_1 = aoc2023::day_14::day_14_1(&data);
    assert_eq!(task_1, 112046);
    //let task_2 = aoc2023::day_14::day_14_1(&data);
    //assert_eq!(task_2, 731244261352);
}
