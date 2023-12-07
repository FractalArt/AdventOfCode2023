//! # Advent of Code 2023 - Day 7
//!
//! This module contains the solution of the [seventh day's challenges](https://adventofcode.com/2023/day/7).
use itertools::Itertools;
use std::collections::HashMap as HM;

#[derive(Debug)]
struct Hand {
    hand: Vec<usize>,
    strength: usize,
    bid: usize,
}

impl Hand {
    fn new(input: &str) -> Self {
        let (hand, bid) = input.split_once(' ').unwrap();
        let bid = bid.parse().unwrap();
        let hand: Vec<usize> = hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                c => c.to_string().parse().unwrap(),
            })
            .collect();
        let counts: Vec<_> = hand
            .iter()
            .fold(HM::new(), |mut state, c| {
                *state.entry(c).or_insert(0) += 1;
                state
            })
            .into_values()
            .sorted()
            .rev()
            //.sorted_by(|(x1, c1), (x2, c2)| Ord::cmp(c1, c2))
            .collect();

        let strength = match counts.len() {
            1 => 7,
            2 if counts[0] == 4 => 6,
            2 if counts[0] == 3 && counts[1] == 2 => 5,
            3 if counts[0] == 3 => 4,
            3 if counts[0] == 2 && counts[1] == 2 => 3,
            4 => 2,
            _ => 1,
        };

        Self {
            hand,
            strength,
            bid,
        }
    }
}

/// The solution to task 1 of day 7.
pub fn day_07_1(data: &[String]) -> usize {
    data.iter()
        .map(|l| Hand::new(l))
        .sorted_by(|h1, h2| {
            if h1.strength != h2.strength {
                Ord::cmp(&h1.strength, &h2.strength)
            } else {
                h1.hand
                    .iter()
                    .zip(&h2.hand)
                    .filter(|(x, y)| x != y)
                    .map(|(x, y)| Ord::cmp(x, y))
                    .next()
                    .unwrap()
            }
        })
        .enumerate()
        .map(|(idx, h)| h.bid * (idx + 1))
        .sum()
}

/// The solution to task 2 of day 7.
//pub fn day_07_2(data: &[String]) -> usize {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_07_1() {
        let data = [
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ];
        assert_eq!(day_07_1(&data), 6440);
    }

    //#[test]
    //fn test_day_07_2() {
    //let data = [
    //];
    //}
}
