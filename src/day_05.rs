//! # Advent of Code 2023 - Day 5
//!
//! This module contains the solution of the [fifth day's challenges](https://adventofcode.com/2023/day/5).
type Map = Vec<(u64, u64, u64)>;

fn parse_input(data: &[String]) -> (Vec<u64>, Vec<Map>) {
    let seed = data[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();
    let mut maps = vec![];
    data.iter()
        .enumerate()
        .skip(2)
        .filter(|(_, line)| line != &"")
        .fold(vec![], |mut state, (idx, line)| {
            if !line.ends_with("map:") {
                let split = line
                    .split(' ')
                    .filter_map(|x| x.parse::<u64>().ok())
                    .collect::<Vec<_>>();
                state.push((split[1], split[0], split[2]));
            }
            if line.ends_with("map:") && !state.is_empty() || idx == data.len() - 1 {
                maps.push(state);
                state = vec![]
            }
            state
        });

    (seed, maps)
}

/// The solution to task 1 of day 5.
pub fn day_05_1(data: &[String]) -> u64 {
    let (seeds, maps) = parse_input(data);
    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |acc, map| {
                if let Some((src, dst, _)) = map.iter().find(|(s, _, n)| s <= &acc && acc <= s + n)
                {
                    dst + acc - src
                } else {
                    acc
                }
            })
        })
        .min()
        .unwrap()
}

/// The solution to task 2 of day 5.
pub fn day_05_2(data: &[String]) -> u64 {
    let (seeds, maps) = parse_input(data);
    (0..)
        .find(|&loc| {
            let first = maps.iter().rev().fold(loc, |state, level| {
                if let Some((s, d, _)) =
                    level.iter().find(|(_, d, n)| d <= &state && state <= d + n)
                {
                    s + state - d
                } else {
                    state
                }
            });
            (0..seeds.len())
                .step_by(2)
                //.any(|(start, end)| start <= first && first <= end)
                .any(|x| seeds[x] <= first && first <= seeds[x] + seeds[x + 1])
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_05_1() {
        let data = [
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ];
        let (seed, maps) = parse_input(&data);
        assert_eq!(seed, vec![79, 14, 55, 13]);
        assert_eq!(maps[0], vec![(98, 50, 2), (50, 52, 48)]);
        assert_eq!(
            maps.into_iter().last().unwrap(),
            vec![(56, 60, 37), (93, 56, 4)]
        );
        assert_eq!(day_05_1(&data), 35);
    }

    #[test]
    fn test_day_05_2() {
        let data = [
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ];
        assert_eq!(day_05_2(&data), 46);
    }
}
