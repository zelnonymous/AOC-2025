use aoc_2025::{Solver, read_input};
use std::io::Error;
use std::str::{FromStr, from_utf8};

#[derive(Debug)]
struct Day2 {}
#[derive(Debug)]
struct Range {
    lower: u64,
    upper: u64,
}
impl Range {
    pub fn new(low: u64, high: u64) -> Self {
        Range {
            lower: low,
            upper: high,
        }
    }
}
impl FromStr for Range {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-").take(2);
        let lower = parts.next().unwrap().parse::<u64>().unwrap();
        let upper = parts.next().unwrap().parse::<u64>().unwrap();
        Ok(Range::new(lower, upper))
    }
}
impl Solver for Day2 {
    type Parsed = Vec<Range>;
    type Part1 = u64;
    type Part2 = u64;
    fn parse_input(&self, input: &str) -> Self::Parsed {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .flat_map(|line| line.split(","))
            .map(|range| range.parse().unwrap())
            .collect()
    }
    fn solve_part1(&self, data: &Self::Parsed, debug: bool) -> Self::Part1 {
        let mut invalid_ids: Vec<u64> = Vec::new();
        for range in data {
            for id in range.lower..=range.upper {
                let str_id = id.to_string();
                let id_len = str_id.len();
                if id_len % 2 != 0 {
                    if debug {
                        println!("{} had an odd number of chars. Skipping.", id)
                    }
                    continue;
                }
                let (left, right) = str_id.split_at(str_id.len() / 2);
                if left == right {
                    if debug {
                        println!(
                            "{} had left {} and right {}, so it's invalid!",
                            id, left, right
                        );
                    }
                    invalid_ids.push(id);
                }
            }
        }
        invalid_ids.iter().sum()
    }
    fn solve_part2(&self, data: &Self::Parsed, debug: bool) -> Self::Part2 {
        let mut invalid_ids: Vec<u64> = Vec::new();
        for range in data {
            for id in range.lower..=range.upper {
                let str_id = id.to_string();
                for start in 1..str_id.len() {
                    let (left, right) = str_id.split_at(start);
                    let compcnt = left.len();
                    if compcnt == 0 {
                        continue;
                    }
                    if right.len() % compcnt != 0 {
                        continue;
                    }
                    let is_repeated = right
                        .as_bytes()
                        .chunks(start)
                        .map(|chunk| from_utf8(chunk).unwrap())
                        .all(|chunk_str| chunk_str == left);
                    if is_repeated {
                        if debug {
                            println!("Found a repeating id: {}", id)
                        }
                        invalid_ids.push(id);
                        break;
                    }
                }
            }
        }
        invalid_ids.iter().sum()
    }
}

fn main() {
    let input = read_input("day02").unwrap();
    let puzzle = Day2 {};
    let parsed = puzzle.parse_input(&input);
    let part1 = puzzle.solve_part1(&parsed, false);
    let part2 = puzzle.solve_part2(&parsed, false);
    println!("Part 1 Solution: {}", part1);
    println!("Part 2 Solution: {}", part2);
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = read_input("day02-example").unwrap();
    let puzzle = Day2 {};
    let parsed = puzzle.parse_input(&input);
    assert_eq!(parsed[2].lower, 998);
    let part1 = puzzle.solve_part1(&parsed, true);
    assert_eq!(part1, 1227775554);
}
#[test]
fn test_part2() {
    let input = read_input("day02-example").unwrap();
    let puzzle = Day2 {};
    let parsed = puzzle.parse_input(&input);
    let part2 = puzzle.solve_part2(&parsed, true);
    assert_eq!(part2, 4174379265);
}
