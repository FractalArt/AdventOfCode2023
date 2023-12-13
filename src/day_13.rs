//! # Advent of Code 2023 - Day 13
//!
//! This module contains the solution of the [thirteenth day's challenges](https://adventofcode.com/2023/day/13).
//type Pattern = Vec<Vec<char>>;
use ndarray::{Array, Axis, Ix2};
type Pattern = Array<char, Ix2>;

fn parse_patterns(data: &[String]) -> Vec<Pattern> {
    let patterns = data
        .iter()
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
        .1;
    let v: Vec<Array<char, Ix2>> = patterns
        .clone()
        .into_iter()
        .map(|pattern| {
            let y = pattern.len();
            let x = pattern[0].len();
            let iter = pattern.into_iter().flat_map(|line| line.into_iter());
            Array::from_iter(iter).into_shape((y, x)).unwrap()
        })
        .collect();
    v
}

pub fn find_horizontal(pattern: &Array<char, Ix2>) -> usize {
    pattern
        .axis_iter(ndarray::Axis(0))
        .enumerate()
        .filter(|(idx, _)| *idx < pattern.nrows() - 1)
        .filter(|(idx, row)| row == pattern.index_axis(Axis(0), idx + 1))
        .filter(|(idx, _)| {
            for r in 1..=std::cmp::min(*idx, pattern.nrows() - idx - 2) {
                if pattern.index_axis(Axis(0), idx - r) != pattern.index_axis(Axis(0), idx + 1 + r)
                {
                    return false;
                }
            }
            true
        })
        .map(|(idx, _)| idx + 1)
        .next()
        .unwrap_or(0)
}

/// The solution to tasks 1 of day 13.
pub fn day_13_1(data: &[String]) -> usize {
    let patterns = parse_patterns(data);

    patterns
        .into_iter()
        .map(|p| {
            let horizontal = find_horizontal(&p);
            println!("Horizontal: {}", horizontal);
            let transposed = p.permuted_axes([1, 0]);
            println!("Transposed:\n{:?}", transposed);
            let vertical = find_horizontal(&transposed);
            println!("Vertical: {}", vertical);
            100 * horizontal + vertical
        })
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
