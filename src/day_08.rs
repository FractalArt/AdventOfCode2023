//! # Advent of Code 2023 - Day 8
//!
//! This module contains the solution of the [eight day's challenges](https://adventofcode.com/2023/day/8).
use regex::Regex;
use std::collections::HashMap as HM;

type LRMap = HM<String, (String, String)>;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"^([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)$").unwrap();
}

fn parse_input(data: &[String]) -> (Vec<char>, LRMap) {
    (
        data[0].trim().chars().collect(),
        data[2..]
            .iter()
            .map(|m| {
                let caps = RE.captures(m).unwrap();
                (
                    caps.get(1).unwrap().as_str().to_string(),
                    (
                        caps.get(2).unwrap().as_str().to_string(),
                        caps.get(3).unwrap().as_str().to_string(),
                    ),
                )
            })
            .collect::<LRMap>(),
    )
}

/// The solution to task 1 of day 8.
pub fn day_08_1(data: &[String]) -> usize {
    let (dirs, lr_map) = parse_input(data);

    dirs.into_iter()
        .cycle()
        .scan("AAA".to_string(), |state, d| {
            *state = if d == 'L' {
                lr_map[state].0.clone()
            } else {
                lr_map[state].1.clone()
            };
            if *state == "ZZZ" {
                None
            } else {
                Some(state.to_string())
            }
        })
        .count()
        + 1
}

/// The solution to task 2 of day 8.
pub fn day_08_2(data: &[String]) -> i64 {
    let (dirs, lr_map) = parse_input(data);

    lr_map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|start| {
            dirs.iter()
                .cycle()
                .scan(start.to_string(), |state, d| {
                    *state = if d == &'L' {
                        lr_map[state].0.clone()
                    } else {
                        lr_map[state].1.clone()
                    };
                    if state.ends_with('Z') {
                        None
                    } else {
                        Some(())
                    }
                })
                .count() as i64
                + 1
        })
        .fold(1, num_integer::lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_08_1() {
        let data = [
            "LLR".to_string(),
            "".to_string(),
            "AAA = (BBB, BBB)".to_string(),
            "BBB = (AAA, ZZZ)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];
        assert_eq!(day_08_1(&data), 6);

        let data = [
            "RL".to_string(),
            "".to_string(),
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCC = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];
        assert_eq!(day_08_1(&data), 2);
    }

    #[test]
    fn test_day_08_2() {
        let data = [
            "LR".to_string(),
            "".to_string(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ];
        assert_eq!(day_08_2(&data), 6);
    }
}
