//! # Advent of Code 2023 - Day 2
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2023/day/s).

fn helper(data: &[String]) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    data.iter()
        .map(|s| s.split(':').nth(1).unwrap().trim())
        .map(|game| {
            game.split(';').flat_map(|draw| {
                draw.split(',').map(|x| {
                    let mut count_color = x.split_whitespace();
                    (
                        count_color.next().unwrap().parse::<usize>().unwrap(),
                        count_color.next().unwrap(),
                    )
                })
            })
        })
        .map(|game| {
            game.fold((0, 0, 0), |acc, (count, col)| match col {
                "red" if count > acc.0 => (count, acc.1, acc.2),
                "green" if count > acc.1 => (acc.0, count, acc.2),
                "blue" if count > acc.2 => (acc.0, acc.1, count),
                _ => acc,
            })
        })
}

/// The solution to task 1 of day 1.
pub fn day_02_1(data: &[String], r_max: usize, g_max: usize, b_max: usize) -> usize {
    helper(data)
        .enumerate()
        .filter_map(|(game, (r, g, b))| {
            if r > r_max || g > g_max || b > b_max {
                None
            } else {
                Some(game + 1)
            }
        })
        .sum()
}

/// The solution to task 2 of day 1.
pub fn day_02_2(data: &[String]) -> usize {
    helper(data).map(|(r, g, b)| r * g * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02_1() {
        let data = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        assert_eq!(day_02_1(&data, 12, 13, 14), 8);
    }

    #[test]
    fn test_day_02_2() {
        let data = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        assert_eq!(day_02_2(&data), 2286);
    }
}
