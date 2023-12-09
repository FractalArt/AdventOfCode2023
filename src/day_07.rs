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
    fn new(input: &str, jokers: bool) -> Self {
        let (hand, bid) = input.split_once(' ').unwrap();
        let bid = bid.parse().unwrap();
        let hand: Vec<usize> = hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' if jokers => 1,
                'J' => 11,
                'T' => 10,
                c => c.to_string().parse().unwrap(),
            })
            .collect();
        let counts: HM<usize, usize> = hand.iter().fold(HM::new(), |mut state, c| {
            *state.entry(*c).or_insert(0) += 1;
            state
        });

        let counts_v: Vec<usize> = counts
            .iter()
            //.into_values()
            .map(|(_, &v)| v)
            .sorted()
            .rev()
            //.sorted_by(|(x1, c1), (x2, c2)| Ord::cmp(c1, c2))
            .collect();

        let joker_count = if jokers {
            if let Some(c) = counts.get(&1) {
                *c
            } else {
                0
            }
        } else {
            0
        };

        let strength = match (counts.len(), joker_count) {
            // five of a kind
            (1, _) => 7,
            // four of a kind
            (2, 0) if counts_v[0] == 4 => 6,
            (2, _) if counts_v[0] == 4 => 7,
            // full house
            (2, 0) if counts_v[0] == 3 && counts_v[1] == 2 => 5,
            (2, _) if counts_v[0] == 3 && counts_v[1] == 2 => 7,
            // three of a kind
            (3, 0) if counts_v[0] == 3 => 4,
            (3, _) if counts_v[0] == 3 => 6,
            // two pair
            (3, 0) if counts_v[0] == 2 && counts_v[1] == 2 => 3,
            (3, 1) if counts_v[0] == 2 && counts_v[1] == 2 => 5,
            (3, 2) if counts_v[0] == 2 && counts_v[1] == 2 => 6,
            // one pair
            (4, 0) => 2,
            (4, _) => 4,
            // all different
            (5, 1) => 2,
            (_, _) => 1,
        };

        Self {
            hand,
            strength,
            bid,
        }
    }
}

/// The solution to tasks 1 and 2 of day 7.
pub fn day_07(data: &[String], jokers: bool) -> usize {
    data.iter()
        .map(|l| Hand::new(l, jokers))
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
        assert_eq!(day_07(&data, false), 6440);
    }

    #[test]
    fn test_day_07_2() {
        let data = [
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ];
        assert_eq!(day_07(&data, true), 5905);
    }
}
