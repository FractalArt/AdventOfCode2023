//! # Advent of Code 2023 - Day 1
//!
//! This module contains the solution of the [first day's challenges](https://adventofcode.com/2023/day/1).
use itertools::Itertools;
use itertools::MinMaxResult as MMR;
use std::collections::HashMap as HM;

fn process_str(s: &str) -> String {
    let digits: HM<_, _> = vec![
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .into_iter()
    .collect();
    match digits
        .iter()
        .filter_map(|(&ds, _)| s.find(ds).map(|idx| (idx, ds)))
        .minmax_by(|a, b| Ord::cmp(&b.0, &a.0))
    {
        MMR::NoElements => s.to_string(),
        MMR::OneElement(one) => s.replace(one.1, digits[one.1]),
        MMR::MinMax(first, last) => s
            .replace(last.1, digits[last.1])
            .replace(first.1, digits[first.1]),
    }
}

/// The solution to task 1 of day 1.
pub fn day_01_1(data: &[String]) -> u32 {
    data.iter()
        .map(|s| {
            s.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| v[0] * 10 + v.last().unwrap())
        .sum()
}

/// The solution to task 2 of day 1.
pub fn day_01_2(data: &[String]) -> u32 {
    data.iter()
        .map(|s| process_str(s))
        .map(|s| {
            s.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| v[0] * 10 + v.last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_1() {
        let data = [
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        assert_eq!(day_01_1(&data), 142);
    }

    #[test]
    fn test_day_01_2() {
        let data = [
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        assert_eq!(day_01_2(&data), 281);
    }
}
