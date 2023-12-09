//! # Advent of Code 2023 - Day 9
//!
//! This module contains the solution of the [ninth day's challenges](https://adventofcode.com/2023/day/9).

/// The solution to task 1 of day 9.
pub fn day_09_1(data: &[String]) -> i64 {
    data.iter()
        .map(|history| {
            history
                .split_whitespace()
                .filter_map(|x| x.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .map(|history| {
            let mut lasts = vec![history[history.len() - 1]];
            let mut diff = history;
            while !diff.iter().all(|x| x == &0) {
                diff = diff[..].windows(2).map(|w| w[1] - w[0]).collect();
                lasts.push(diff[diff.len() - 1])
            }
            lasts.iter().rev().sum::<i64>()
        })
        .sum::<i64>()
}

/// The solution to task 2 of day 9.
pub fn day_09_2(data: &[String]) -> i64 {
    data.iter()
        .map(|history| {
            history
                .split_whitespace()
                .filter_map(|x| x.parse::<i64>().ok())
                .rev()
                .collect::<Vec<_>>()
        })
        .map(|history| {
            let mut lasts = vec![history[history.len() - 1]];
            let mut diff = history;
            while !diff.iter().all(|x| x == &0) {
                diff = diff[..].windows(2).map(|w| w[0] - w[1]).collect();
                lasts.push(diff[diff.len() - 1])
            }
            lasts.iter().rev().fold(0, |state, last| last - state)
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_09_1() {
        let data = [
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ];
        assert_eq!(day_09_1(&data), 114);
    }

    #[test]
    fn test_day_09_2() {
        let data = [
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ];
        assert_eq!(day_09_2(&data), 2);
    }
}
