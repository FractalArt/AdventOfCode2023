//! # Advent of Code 2023 - Day 10
//!
//! This module contains the solution of the [tenth day's challenges](https://adventofcode.com/2023/day/10).
use std::collections::HashSet as HS;

#[derive(Clone)]
struct Pipe {
    symbol: char,
    coord: (usize, usize),
    prev: (usize, usize),
}

impl Pipe {
    fn next(&self) -> (usize, usize) {
        match (
            self.symbol,
            self.coord.0 as isize - self.prev.0 as isize,
            self.coord.1 as isize - self.prev.1 as isize,
        ) {
            ('F', 0, -1) => (self.coord.0 + 1, self.coord.1),
            ('F', -1, 0) => (self.coord.0, self.coord.1 + 1),
            ('J', 1, 0) => (self.coord.0, self.coord.1 - 1),
            ('J', 0, 1) => (self.coord.0 - 1, self.coord.1),
            ('L', -1, 0) => (self.coord.0, self.coord.1 - 1),
            ('L', 0, 1) => (self.coord.0 + 1, self.coord.1),
            ('7', 1, 0) => (self.coord.0, self.coord.1 + 1),
            ('7', 0, -1) => (self.coord.0 - 1, self.coord.1),
            ('|', 0, 1) => (self.coord.0, self.coord.1 + 1),
            ('|', 0, -1) => (self.coord.0, self.coord.1 - 1),
            ('-', 1, 0) => (self.coord.0 + 1, self.coord.1),
            ('-', -1, 0) => (self.coord.0 - 1, self.coord.1),
            _ => panic!("Wrong"),
        }
    }
}

fn get_start_pos(start: (usize, usize), grid: &[Vec<char>]) -> (Pipe, Pipe) {
    let left = grid[start.1][start.0 - 1];
    let right = grid[start.1][start.0 + 1];
    let up = grid[start.1 - 1][start.0];
    let down = grid[start.1 + 1][start.0];
    let mut ret = vec![];
    if ['L', 'F', '-'].contains(&left) {
        ret.push(Pipe {
            symbol: left,
            coord: (start.0 - 1, start.1),
            prev: start,
        });
    }
    if ['7', 'J', '-'].contains(&right) {
        ret.push(Pipe {
            symbol: right,
            coord: (start.0 + 1, start.1),
            prev: start,
        });
    }
    if ['|', 'F', '7'].contains(&up) {
        ret.push(Pipe {
            symbol: up,
            coord: (start.0, start.1 - 1),
            prev: start,
        });
    }
    if ['|', 'J', 'L'].contains(&down) {
        ret.push(Pipe {
            symbol: down,
            coord: (start.0, start.1 + 1),
            prev: start,
        });
    }

    (ret[0].clone(), ret[1].clone())
}

fn parse_grid(data: &[String]) -> ((usize, usize), Vec<Vec<char>>) {
    let mut start = (0, 0);
    let grid = data
        .iter()
        .enumerate()
        // the lines
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (start, grid)
}

/// The solution to task 1 of day 10.
pub fn day_10_1(data: &[String]) -> usize {
    let (start, grid) = parse_grid(data);
    let (next1, next2) = get_start_pos(start, &grid);

    (1..)
        .scan((next1, next2), |(n1, n2), _| {
            *n1 = Pipe {
                symbol: grid[n1.next().1][n1.next().0],
                coord: n1.next(),
                prev: n1.coord,
            };
            *n2 = Pipe {
                symbol: grid[n2.next().1][n2.next().0],
                coord: n2.next(),
                prev: n2.coord,
            };
            if n1.coord == n2.coord {
                None
            } else {
                Some(())
            }
        })
        .count()
        + 2
}

/// The solution to task 2 of day 10.
pub fn day_10_2(data: &[String]) -> usize {
    let (start, grid) = parse_grid(data);
    let (next1, _) = get_start_pos(start, &grid);
    let mut contour = [start].into_iter().collect::<HS<_>>();
    let mut contour_v = vec![];
    let mut current = next1;
    while current.coord != start {
        contour.insert(current.coord);
        contour_v.push(current.coord);
        let (x, y) = current.next();
        current = Pipe {
            symbol: grid[y][x],
            coord: (x, y),
            prev: current.coord,
        };
    }

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if contour.contains(&(x, y)) {
                continue;
            }
            if (x + 1..grid[y].len())
                .filter(|xx| contour.contains(&(*xx, y)))
                .count()
                % 2
                == 1
                && (0..x).any(|xx| contour.contains(&(xx, y)))
            {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10_1() {
        let data = [
            "J7-F7-".to_string(),
            "J.FJ|7".to_string(),
            "JSJLL7".to_string(),
            "J|F--J".to_string(),
            "JLJ.LJ".to_string(),
        ];
        assert_eq!(day_10_1(&data), 8);
    }

    #[test]
    fn test_day_10_2() {
        let data = [
            "...........".to_string(),
            ".S-------7.".to_string(),
            ".|F-----7|.".to_string(),
            ".||.....||.".to_string(),
            ".||.....||.".to_string(),
            ".|L-7.F-J|.".to_string(),
            ".|..|.|..|.".to_string(),
            ".L--J.L--J.".to_string(),
            "...........".to_string(),
        ];
        assert_eq!(day_10_2(&data), 4);

        let data = [
            ".F----7F7F7F7F-7....".to_string(),
            ".|F--7||||||||FJ....".to_string(),
            ".||.FJ||||||||L7....".to_string(),
            "FJL7L7LJLJ||LJ.L-7..".to_string(),
            "L--J.L7...LJS7F-7L7.".to_string(),
            "....F-J..F7FJ|L7L7L7".to_string(),
            "....L7.F7||L7|.L7L7|".to_string(),
            ".....|FJLJ|FJ|F7|.LJ".to_string(),
            "....FJL-7.||.||||...".to_string(),
            "....L---J.LJ.LJLJ...".to_string(),
        ];
        assert_eq!(day_10_2(&data), 8);

        //let data = [
        //"....................".to_string(),
        //"FF7FSF7F7F7F7F7F---7".to_string(),
        //"L|LJ||||||||||||F--J".to_string(),
        //"FL-7LJLJ||||||LJL-77".to_string(),
        //"F--JF--7||LJLJ7F7FJ-".to_string(),
        //"L---JF-JLJ.||-FJLJJ7".to_string(),
        //"|F|F-JF---7F7-L7L|7|".to_string(),
        //"|FFJF7L7F-JF7|JL---7".to_string(),
        //"7-L-JL7||F7|L7F-7F7|".to_string(),
        //"L.L7LFJ|||||FJL7||LJ".to_string(),
        //"L7JLJL-JLJLJL--JLJ.L".to_string(),
        //];
        //assert_eq!(day_10_2(&data), 10);
    }
}
