//! # Advent of Code 2023 - Day 3
//!
//! This module contains the solution of the [third day's challenges](https://adventofcode.com/2023/day/3).
use regex::Regex;
use std::collections::{HashMap as HM, HashSet as HS};

type Symbols = HM<(usize, usize), char>;

struct Number {
    value: u64,
    adjacents: HS<(usize, usize)>,
}

impl Number {
    fn new(value: u64, start: (usize, usize), end: (usize, usize)) -> Self {
        let sxs = if start.0 == 0 { 0 } else { start.0 - 1 };
        let sys = if start.1 == 0 { 0 } else { start.1 - 1 };
        let adjacents: HS<(usize, usize)> = (sxs..=end.0)
            .flat_map(|x| (sys..=end.1 + 1).map(move |y| (x, y)))
            .collect();
        Self { value, adjacents }
    }

    fn is_adjacent(&self, target: (usize, usize)) -> bool {
        self.adjacents.contains(&target)
    }
}

fn parse_engine_schematic(data: &[String]) -> (Symbols, Vec<Number>) {
    let mut numbers = vec![];
    let mut symbols: Symbols = HM::new();

    for (y, line) in data.iter().enumerate() {
        // parse numbers
        Regex::new(r"(\d+)").unwrap().find_iter(line).for_each(|m| {
            numbers.push(Number::new(
                m.as_str().parse::<u64>().unwrap(),
                (m.start(), y),
                (m.end(), y),
            ));
        });

        line.chars()
            .enumerate()
            .filter(|(_, c)| !(c.is_ascii_digit() || c == &'.'))
            .for_each(|(x, c)| {
                symbols.insert((x, y), c);
            });
    }

    (symbols, numbers)
}

/// The solution to task 1 of day 3.
pub fn day_03_1(data: &[String]) -> u64 {
    let (symbols, numbers) = parse_engine_schematic(data);
    numbers
        .into_iter()
        .filter(|n| symbols.keys().any(|k| n.is_adjacent(*k)))
        .map(|n| n.value)
        .sum()
}

/// The solution to task 2 of day 3.
pub fn day_03_2(data: &[String]) -> u64 {
    let (symbols, numbers) = parse_engine_schematic(data);
    symbols
        .into_iter()
        .filter(|(_, v)| v == &'*')
        .filter_map(|(k, _)| {
            let adjacents = numbers
                .iter()
                .filter(|n| n.is_adjacent(k))
                .map(|n| n.value)
                .collect::<Vec<_>>();
            if adjacents.len() == 2 {
                Some(adjacents.iter().product::<u64>())
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
