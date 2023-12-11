//! # Advent of Code 2023 - Day 11
//!
//! This module contains the solution of the [eleventh day's challenges](https://adventofcode.com/2023/day/11).
use std::cmp::{max, min};

/// The solution to tasks 1 & 2 of day 11.
pub fn day_11(data: &[String], factor: usize) -> usize {
    let (x_count, y_count, galaxies) = data.iter().enumerate().fold(
        (vec![0; data.len()], vec![0; data[0].len()], vec![]),
        |(mut xs, mut ys, mut galaxies), (y, line)| {
            line.trim().chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    xs[x] += 1;
                    ys[y] += 1;
                    galaxies.push((x, y));
                }
            });
            (xs, ys, galaxies)
        },
    );

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(idx, g1)| galaxies[idx + 1..].iter().map(move |g2| (g1, g2)))
        .map(|((g1x, g1y), (g2x, g2y))| {
            (min(*g1x, *g2x) + 1..=max(*g1x, *g2x))
                .map(|x| if x_count[x] == 0 { factor } else { 1 })
                .sum::<usize>()
                + (min(*g1y, *g2y) + 1..=max(*g1y, *g2y))
                    .map(|y| if y_count[y] == 0 { factor } else { 1 })
                    .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_11_1() {
        let data = [
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ];
        assert_eq!(day_11(&data, 2), 374);
    }

    #[test]
    fn test_day_11_2() {
        let data = [
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ];
        assert_eq!(day_11(&data, 10), 1030);
        assert_eq!(day_11(&data, 100), 8410);
    }
}
