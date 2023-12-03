//! # Advent of Code 2023 - Day 3
//!
//! This module contains the solution of the [third day's challenges](https://adventofcode.com/2023/day/3).
use std::collections::{HashMap as HM, HashSet as HS};

/// The solution to task 1 of day 3.
pub fn day_03_1(data: &[String]) -> u64 {
    let mut numbers = vec![];
    let mut symbols: HS<(usize, usize)> = HS::new();
    let mut parsing_number = false;
    let mut number_start = (0, 0);
    let mut number_candidate = vec![];
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            // check if we are parsing a nunmber
            if c.is_ascii_digit() {
                if !parsing_number {
                    number_start = (x, y);
                    parsing_number = true;
                    number_candidate.push(c.to_string().parse::<u64>().unwrap());
                } else {
                    number_candidate.push(c.to_string().parse::<u64>().unwrap());
                }
                if x == data[0].len() - 1 {
                    // finish the number parsing
                    parsing_number = false;
                    let number_end = (x, y);
                    let number: u64 = number_candidate
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(p, x)| x * 10u64.pow(p as u32))
                        .sum();
                    if number > 0 {
                        numbers.push((number, number_start, number_end));
                    }
                    number_candidate = vec![];
                }
            } else if parsing_number || x == data[0].len() - 1 {
                // finish the number parsing
                parsing_number = false;
                let number_end = (x, y);
                let number: u64 = number_candidate
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(p, x)| x * 10u64.pow(p as u32))
                    .sum();
                if number > 0 {
                    numbers.push((number, number_start, number_end));
                }
                number_candidate = vec![];
                // parse the actual symbol
                if c == '.' {
                    continue;
                } else {
                    symbols.insert((x, y));
                }
            } else if c == '.' {
                continue;
            } else {
                symbols.insert((x, y));
            }
        }
    }

    // now find adjacent
    numbers
        .into_iter()
        .filter_map(|(n, (sx, sy), (ex, ey))| {
            let mut targets: HS<(usize, usize)> = HS::new();
            let sxs = if sx == 0 { 0 } else { sx - 1 };
            let sys = if sy == 0 { 0 } else { sy - 1 };
            for x in sxs..=ex {
                for y in sys..=ey + 1 {
                    targets.insert((x, y));
                }
            }
            if symbols.intersection(&targets).count() > 0 {
                Some(n)
            } else {
                None
            }
        })
        .sum()
}

/// The solution to task 2 of day 3.
pub fn day_03_2(data: &[String]) -> u64 {
    let mut numbers = vec![];
    let mut symbols: HS<(usize, usize)> = HS::new();
    let mut parsing_number = false;
    let mut number_start = (0, 0);
    let mut number_candidate = vec![];
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            // check if we are parsing a nunmber
            if c.is_ascii_digit() {
                if !parsing_number {
                    number_start = (x, y);
                    parsing_number = true;
                    number_candidate.push(c.to_string().parse::<u64>().unwrap());
                } else {
                    number_candidate.push(c.to_string().parse::<u64>().unwrap());
                }
                if x == data[0].len() - 1 {
                    // finish the number parsing
                    parsing_number = false;
                    let number_end = (x, y);
                    let number: u64 = number_candidate
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(p, x)| x * 10u64.pow(p as u32))
                        .sum();
                    if number > 0 {
                        numbers.push((number, number_start, number_end));
                    }
                    number_candidate = vec![];
                }
            } else if parsing_number || x == data[0].len() - 1 {
                // finish the number parsing
                parsing_number = false;
                let number_end = (x, y);
                let number: u64 = number_candidate
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(p, x)| x * 10u64.pow(p as u32))
                    .sum();
                if number > 0 {
                    numbers.push((number, number_start, number_end));
                }
                number_candidate = vec![];
                // parse the actual symbol
                if c == '*' {
                    symbols.insert((x, y));
                } else {
                    continue;
                }
            } else if c == '*' {
                symbols.insert((x, y));
            } else {
                continue;
            }
        }
    }

    let mut sym_map: HM<(usize, usize), Vec<u64>> = HM::new();
    numbers.into_iter().for_each(|(n, (sx, sy), (ex, ey))| {
        let mut targets: HS<(usize, usize)> = HS::new();
        let sxs = if sx == 0 { 0 } else { sx - 1 };
        let sys = if sy == 0 { 0 } else { sy - 1 };
        for x in sxs..=ex {
            for y in sys..=ey + 1 {
                targets.insert((x, y));
            }
        }
        symbols
            .intersection(&targets)
            .for_each(|coord| sym_map.entry(*coord).or_default().push(n));
    });

    sym_map
        .into_iter()
        .filter_map(|(_, v)| {
            if v.len() == 2 {
                Some(v.iter().product::<u64>())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_03_1() {
        let data = [
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!(day_03_1(&data), 4361);
    }

    #[test]
    fn test_day_03_2() {
        let data = [
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        assert_eq!(day_03_2(&data), 467835);
    }
}
