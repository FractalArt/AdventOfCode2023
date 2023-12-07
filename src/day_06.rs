//! # Advent of Code 2023 - Day 6
//!
//! This module contains the solution of the [sixth day's challenges](https://adventofcode.com/2023/day/6).
fn parse_line(line: &str) -> Vec<usize> {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.trim().parse().ok())
        .collect()
}

/// The solution to task 1 of day 6.
pub fn day_06_1(data: &[String]) -> usize {
    parse_line(&data[0])
        .iter()
        .zip(&parse_line(&data[1]))
        .map(|(&t, d)| (1..t).map(|h| (t - h) * h).filter(|x| x > d).count())
        .product()
}

/// The solution to task 2 of day 6.
pub fn day_06_2(data: &[String]) -> usize {
    parse_line(&data[0].replace(' ', ""))
        .iter()
        .zip(&parse_line(&data[1].replace(' ', "")))
        .map(|(&t, d)| (1..t).map(|h| (t - h) * h).filter(|x| x > d).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_06_1() {
        let data = [
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];
        assert_eq!(day_06_1(&data), 288);
    }

    #[test]
    fn test_day_06_2() {
        let data = [
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];
        assert_eq!(day_06_2(&data), 71503);
    }
}
