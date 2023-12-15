//! # Advent of Code 2023 - Day 15
//!
//! This module contains the solution of the [fifteenth day's challenges](https://adventofcode.com/2023/day/15).
fn hash(s: &str) -> usize {
    s.chars().fold(0, |curr, c| (curr + c as usize) * 17 % 256)
}

/// The solution to task 1 of day 15.
pub fn day_15_1(data: &str) -> usize {
    data.trim().split(',').map(hash).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_15_1() {
        let data = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(day_15_1(data), 1320);
    }

    //#[test]
    //fn test_day_15_2() {
    //}
}
