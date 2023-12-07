use aoc2023::read_data;

fn main() {
    // day 01
    let data_01 = read_data::<String, _>("data/day_01.txt").unwrap();
    println!("[Day 01 - Task 1]: {}", aoc2023::day_01::day_01_1(&data_01));
    println!("[Day 01 - Task 2]: {}", aoc2023::day_01::day_01_2(&data_01));

    // day 02
    let data_02 = read_data::<String, _>("data/day_02.txt").unwrap();
    println!(
        "[Day 02 - Task 1]: {}",
        aoc2023::day_02::day_02_1(&data_02, 12, 13, 14)
    );
    println!("[Day 02 - Task 2]: {}", aoc2023::day_02::day_02_2(&data_02));

    // day 03
    let data_03 = read_data::<String, _>("data/day_03.txt").unwrap();
    println!("[Day 03 - Task 1]: {}", aoc2023::day_03::day_03_1(&data_03));
    println!("[Day 03 - Task 2]: {}", aoc2023::day_03::day_03_2(&data_03));

    // day 04
    let data_04 = read_data::<String, _>("data/day_04.txt").unwrap();
    println!("[Day 04 - Task 1]: {}", aoc2023::day_04::day_04_1(&data_04));
    println!("[Day 04 - Task 2]: {}", aoc2023::day_04::day_04_2(&data_04));

    // day 05
    let data_05 = read_data::<String, _>("data/day_05.txt").unwrap();
    println!("[Day 05 - Task 1]: {}", aoc2023::day_05::day_05_1(&data_05));
    println!("[Day 05 - Task 2]: {}", aoc2023::day_05::day_05_2(&data_05));

    // day 06
    let data_06 = read_data::<String, _>("data/day_06.txt").unwrap();
    println!("[Day 06 - Task 1]: {}", aoc2023::day_06::day_06_1(&data_06));
    println!("[Day 06 - Task 2]: {}", aoc2023::day_06::day_06_2(&data_06));

    // day 07
    let data_07 = read_data::<String, _>("data/day_07.txt").unwrap();
    println!("[Day 07 - Task 1]: {}", aoc2023::day_07::day_07_1(&data_07));
    //println!("[Day 07 - Task 2]: {}", aoc2023::day_07::day_07_2(&data_07));
}
