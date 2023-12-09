use aoc2023::{self, read_data};

#[test]
fn test_day_08() {
    let data = read_data::<String, _>("data/day_08.txt").unwrap();
    let task_1 = aoc2023::day_08::day_08_1(&data);
    assert_eq!(task_1, 15871);
    //let task_2 = aoc2023::day_08::day_08_2(&data);
    //assert_eq!(task_2, );
}
