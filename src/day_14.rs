//! # Advent of Code 2023 - Day 14
//!
//! This module contains the solution of the [fourteenth day's challenges](https://adventofcode.com/2023/day/14).

fn parse(data: &[String]) -> Vec<Vec<char>> {
    data.iter().map(|line| line.chars().collect()).collect()
}

/// The solution to task 1 of day 14.
pub fn day_14_1(data: &[String]) -> usize {
    let mut data = parse(data);
    (0..data[0].len())
        .map(|col| {
            (0..data.len()).for_each(|idx| {
                if data[idx][col] == 'O' {
                    for row in (0..idx).rev() {
                        match (row, data[row][col]) {
                            (r, 'O' | '#') => {
                                data[idx][col] = '.';
                                data[r + 1][col] = 'O';
                                break;
                            }
                            (0, '.') => {
                                data[idx][col] = '.';
                                data[0][col] = 'O';
                                break;
                            }
                            (_, _) => {}
                        }
                    }
                }
            });
            (0..data.len())
                .filter(|&r| data[r][col] == 'O')
                .map(|r| data.len() - r)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_14_1() {
        let data = [
            "O....#....".to_string(),
            "O.OO#....#".to_string(),
            ".....##...".to_string(),
            "OO.#O....O".to_string(),
            ".O.....O#.".to_string(),
            "O.#..O.#.#".to_string(),
            "..O..#O..O".to_string(),
            ".......O..".to_string(),
            "#....###..".to_string(),
            "#OO..#....".to_string(),
        ];
        assert_eq!(day_14_1(&data), 136);
    }

    //#[test]
    //fn test_day_13_2() {
    //}
}
