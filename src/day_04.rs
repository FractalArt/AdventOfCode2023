//! # Advent of Code 2023 - Day 4
//!
//! This module contains the solution of the [fourth day's challenges](https://adventofcode.com/2023/day/4).
use std::collections::{HashMap as HM, HashSet as HS};

fn count_winners(card: &str) -> usize {
    let (winners, have) = card.split(':').nth(1).unwrap().split_once(" | ").unwrap();
    winners
        .split_whitespace()
        .map(|w| w.trim().parse::<u32>().unwrap())
        .collect::<HS<_>>()
        .intersection(
            &have
                .split_whitespace()
                .map(|h| h.trim().parse::<u32>().unwrap())
                .collect::<HS<_>>(),
        )
        .count()
}

/// The solution to task 1 of day 4.
pub fn day_04_1(data: &[String]) -> usize {
    data.iter()
        .map(|card| count_winners(card))
        .filter(|count| count > &0)
        .map(|count| 2usize.pow(count as u32 - 1))
        .sum()
}

/// The solution to task 2 of day 4.
pub fn day_04_2(data: &[String]) -> usize {
    data.iter()
        .enumerate()
        .fold(
            (1..=data.len()).map(|x| (x, 1)).collect::<HM<_, _>>(),
            |mut card_counts, (idx, card)| {
                (idx + 2..idx + 2 + count_winners(card))
                    .for_each(|c| *card_counts.get_mut(&c).unwrap() += card_counts[&(idx + 1)]);
                card_counts
            },
        )
        .values()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_04_1() {
        let data = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
        assert_eq!(day_04_1(&data), 13);
    }

    #[test]
    fn test_day_04_2() {
        let data = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
        assert_eq!(day_04_2(&data), 30);
    }
}
