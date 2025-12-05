use aoc_2025::{Solver, read_input};

struct Day5 {}
#[derive(Clone)]
struct FreshRange {
    lower: u64,
    upper: u64,
}
struct DB {
    fresh_ranges: Vec<FreshRange>,
    available: Vec<u64>,
}
impl DB {
    fn new() -> Self {
        DB {
            fresh_ranges: Vec::new(),
            available: Vec::new(),
        }
    }
}
impl FreshRange {
    fn new(low: u64, high: u64) -> Self {
        FreshRange {
            lower: low,
            upper: high,
        }
    }
}
impl Solver for Day5 {
    type Parsed = DB;
    type Part1 = u64;
    type Part2 = u64;
    fn parse_input(&self, input: &str) -> Self::Parsed {
        let mut db = DB::new();
        for line in input.lines() {
            match line.split_once('-') {
                Some((low, high)) => match (low.parse::<u64>(), high.parse::<u64>()) {
                    (Ok(low), Ok(high)) => {
                        db.fresh_ranges.push(FreshRange::new(low, high));
                    }
                    _ => {
                        continue;
                    }
                },
                None => match line.parse::<u64>() {
                    Ok(ingredient_id) => db.available.push(ingredient_id),
                    Err(_) => continue,
                },
            }
        }
        db
    }
    fn solve_part1(&self, data: &Self::Parsed, debug: bool) -> Self::Part1 {
        let fresh: Vec<_> = data
            .available
            .iter()
            .filter(|ingredient| {
                data.fresh_ranges
                    .iter()
                    .any(|fresh| (fresh.lower..=fresh.upper).contains(ingredient))
            })
            .collect();
        if debug {
            for ingredient in &fresh {
                println!("{:?} is fresh", ingredient);
            }
        }
        fresh.len() as u64
    }
    fn solve_part2(&self, data: &Self::Parsed, _debug: bool) -> Self::Part2 {
        // I thought about going back and updating the parser to combine ranges so that
        // all ranges contain distinct IDs from the start, but that was tricky without
        // the ranges first being sorted.  Instead, I opted to copy fresh ranges and
        // sort them by the lower bound, then iterate over them and combine ranges where
        // there is overlap.
        let mut ranges = data.fresh_ranges.clone();
        ranges.sort_by_key(|range| range.lower);
        let mut merged_ranges: Vec<FreshRange> = Vec::new();
        for range in ranges {
            if let Some(last) = merged_ranges.last_mut()
                && range.lower <= last.upper + 1
            {
                last.upper = last.upper.max(range.upper);
                continue;
            }
            merged_ranges.push(range);
        }
        merged_ranges
            .iter()
            .map(|range| range.upper - range.lower + 1)
            .sum()
    }
}

fn main() {
    let input = read_input("day05").unwrap();
    let puzzle = Day5 {};
    let parsed = puzzle.parse_input(&input);
    let part1 = puzzle.solve_part1(&parsed, false);
    let part2 = puzzle.solve_part2(&parsed, false);
    println!("Part 1 Solution: {}", part1);
    println!("Part 2 Solution: {}", part2);
}
#[cfg(test)]
#[test]
fn test_part1() {
    let input = read_input("day05-example").unwrap();
    let puzzle = Day5 {};
    let parsed = puzzle.parse_input(&input);
    assert_eq!(parsed.fresh_ranges[2].lower, 16);
    let part1 = puzzle.solve_part1(&parsed, true);
    assert_eq!(part1, 3);
}
#[test]
fn test_part2() {
    let input = read_input("day05-example").unwrap();
    let puzzle = Day5 {};
    let parsed = puzzle.parse_input(&input);
    let part2 = puzzle.solve_part2(&parsed, true);
    assert_eq!(part2, 14);
}
