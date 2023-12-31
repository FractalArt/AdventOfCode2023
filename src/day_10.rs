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
    fn new(symbol: char, coord: (usize, usize), prev: (usize, usize)) -> Self {
        Self {
            symbol,
            coord,
            prev,
        }
    }

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
            (_, _, _) => (self.coord.0 - 1, self.coord.1),
        }
    }
}

fn get_start_pos(start: (usize, usize), grid: &[Vec<char>]) -> (char, Pipe, Pipe) {
    let left = grid[start.1][start.0 - 1];
    let right = grid[start.1][start.0 + 1];
    let up = grid[start.1 - 1][start.0];
    let down = grid[start.1 + 1][start.0];
    match (left, right, up, down) {
        (_, _, u, d) if ['L', '|', 'J'].contains(&d) && ['|', '7', 'F'].contains(&u) => (
            '|',
            Pipe::new(u, (start.0, start.1 - 1), start),
            Pipe::new(d, (start.0, start.1 + 1), start),
        ),
        (l, r, _, _) if ['L', 'F', '-'].contains(&l) && ['7', 'J', '-'].contains(&r) => (
            '-',
            Pipe::new(l, (start.0 - 1, start.1), start),
            Pipe::new(r, (start.0 + 1, start.1), start),
        ),
        (l, _, u, _) if ['L', 'F', '-'].contains(&l) && ['7', 'F', '|'].contains(&u) => (
            'J',
            Pipe::new(l, (start.0 - 1, start.1), start),
            Pipe::new(u, (start.0, start.1 - 1), start),
        ),
        (l, _, _, d) if ['L', 'F', '-'].contains(&l) && ['L', 'J', '|'].contains(&d) => (
            '7',
            Pipe::new(l, (start.0 - 1, start.1), start),
            Pipe::new(d, (start.0, start.1 + 1), start),
        ),
        (_, r, u, _) if ['7', 'J', '-'].contains(&r) && ['7', 'F', '|'].contains(&u) => (
            'L',
            Pipe::new(r, (start.0 + 1, start.1), start),
            Pipe::new(u, (start.0, start.1 - 1), start),
        ),
        (_, r, _, d) => (
            'F',
            Pipe::new(r, (start.0 + 1, start.1), start),
            Pipe::new(d, (start.0, start.1 + 1), start),
        ),
    }
}

fn parse_grid(data: &[String]) -> ((usize, usize), Vec<Vec<char>>) {
    let mut start = (0, 0);
    let grid = data
        .iter()
        .enumerate()
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
    let (_, next1, next2) = get_start_pos(start, &grid);

    (1..)
        .scan((next1, next2), |(n1, n2), _| {
            let next1 = n1.next();
            let next2 = n2.next();
            *n1 = Pipe::new(grid[next1.1][next1.0], next1, n1.coord);
            *n2 = Pipe::new(grid[next2.1][next2.0], next2, n2.coord);
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
    let (start, mut grid) = parse_grid(data);
    let (s, next1, _) = get_start_pos(start, &grid);
    grid[start.1][start.0] = s;
    let mut contour = [start].into_iter().collect::<HS<_>>();
    let mut current = next1;
    while current.coord != start {
        contour.insert(current.coord);
        let (x, y) = current.next();
        current = Pipe::new(grid[y][x], (x, y), current.coord);
    }

    grid.iter()
        .enumerate()
        .flat_map(|(y, _)| (0..grid[y].len()).map(move |x| (x, y)))
        .filter(|(x, y)| !contour.contains(&(*x, *y)))
        .filter(|(x, y)| {
            (x + 1..grid[*y].len())
                .filter(|xx| {
                    contour.contains(&(*xx, *y)) && ['L', 'J', '|'].contains(&grid[*y][*xx])
                })
                .count()
                % 2
                == 1
        })
        .count()
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

        let data = [
            "....................".to_string(),
            "FF7FSF7F7F7F7F7F---7".to_string(),
            "L|LJ||||||||||||F--J".to_string(),
            "FL-7LJLJ||||||LJL-77".to_string(),
            "F--JF--7||LJLJ7F7FJ-".to_string(),
            "L---JF-JLJ.||-FJLJJ7".to_string(),
            "|F|F-JF---7F7-L7L|7|".to_string(),
            "|FFJF7L7F-JF7|JL---7".to_string(),
            "7-L-JL7||F7|L7F-7F7|".to_string(),
            "L.L7LFJ|||||FJL7||LJ".to_string(),
            "L7JLJL-JLJLJL--JLJ.L".to_string(),
        ];
        assert_eq!(day_10_2(&data), 10);
    }
}
