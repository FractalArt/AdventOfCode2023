//! # Advent of Code 2023 - Day 13
//!
//! This module contains the solution of the [thirteenth day's challenges](https://adventofcode.com/2023/day/13).
use ndarray::{Array, Axis as Ax, Ix2};

fn parse_patterns(data: &[String]) -> Vec<Array<char, Ix2>> {
    data.iter()
        .chain([&"".to_string()])
        .map(|l| l.trim())
        .fold((vec![], vec![]), |(mut curr, mut patterns), l| {
            if l.is_empty() {
                patterns.push(curr);
                (vec![], patterns)
            } else {
                curr.push(l.chars().collect::<Vec<char>>());
                (curr, patterns)
            }
        })
        .1
        .into_iter()
        .map(|pattern| {
            let (y, x) = (pattern.len(), pattern[0].len());
            let iter = pattern.into_iter().flat_map(|p| p.into_iter());
            Array::from_iter(iter).into_shape((y, x)).unwrap()
        })
        .collect()
}

fn symmetry_axis(pattern: &Array<char, Ix2>) -> usize {
    pattern
        .axis_iter(ndarray::Axis(0))
        .enumerate()
        .filter(|(idx, _)| *idx < pattern.nrows() - 1)
        .filter(|(idx, row)| row == pattern.index_axis(Ax(0), idx + 1))
        .filter(|(idx, _)| {
            (1..=std::cmp::min(*idx, pattern.nrows() - idx - 2)).all(|r| {
                pattern.index_axis(Ax(0), idx - r) == pattern.index_axis(Ax(0), idx + 1 + r)
            })
        })
        .map(|(idx, _)| idx + 1)
        .next()
        .unwrap_or(0)
}

/// The solution to task 1 of day 13.
pub fn day_13_1(data: &[String]) -> usize {
    parse_patterns(data)
        .into_iter()
        .map(|p| 100 * symmetry_axis(&p) + symmetry_axis(&p.permuted_axes([1, 0])))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_13_1() {
        let data = [
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
            "".to_string(),
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ];
        assert_eq!(day_13_1(&data), 405);
    }

    //#[test]
    //fn test_day_13_2() {
    //}
}
