#[test]
fn test_day_15() {
    let data = std::fs::read_to_string("data/day_15.txt").unwrap();
    let task_1 = aoc2023::day_15::day_15_1(&data);
    assert_eq!(task_1, 112046);
    //let task_2 = aoc2023::day_15::day_15_1(&data);
    //assert_eq!(task_2, 731244261352);
}
