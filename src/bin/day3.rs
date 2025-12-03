use aoc_2025::{Solver, read_input};

#[derive(Debug)]
struct Day3 {}

impl Day3 {
    fn max_subsequence_of_len(&self, s: &str, k: usize) -> i64 {
        let mut stack: Vec<char> = Vec::new();
        let mut to_remove = s.len().saturating_sub(k);

        for c in s.chars() {
            while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < c {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(c);
        }

        stack.truncate(k);

        stack.iter().collect::<String>().parse::<i64>().unwrap()
    }
}
impl Solver for Day3 {
    type Parsed = Vec<String>;
    type Part1 = i64;
    type Part2 = i64;
    fn parse_input(&self, input: &str) -> Self::Parsed {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect()
    }
    fn solve_part1(&self, data: &Self::Parsed, debug: bool) -> Self::Part1 {
        let mut result = 0;
        for batteries in data {
            let max_joltage = self.max_subsequence_of_len(batteries, 2);
            if debug {
                println!("Max joltage for {}: {}", batteries, max_joltage);
            }
            result += max_joltage;
        }
        result
    }
    fn solve_part2(&self, data: &Self::Parsed, debug: bool) -> Self::Part2 {
        let mut result = 0;
        for batteries in data {
            let max_joltage = self.max_subsequence_of_len(batteries, 12);
            if debug {
                println!("Max joltage for {}: {}", batteries, max_joltage);
            }
            result += max_joltage;
        }
        result
    }
}
fn main() {
    let input = read_input("day03").unwrap();
    let puzzle = Day3 {};
    let parsed = puzzle.parse_input(&input);
    let part1 = puzzle.solve_part1(&parsed, false);
    let part2 = puzzle.solve_part2(&parsed, false);
    println!("Part 1 Solution: {}", part1);
    println!("Part 2 Solution: {}", part2);
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = read_input("day03-example").unwrap();
    let puzzle = Day3 {};
    let parsed = puzzle.parse_input(&input);
    assert_eq!(parsed[2], "234234234234278");
    let part1 = puzzle.solve_part1(&parsed, true);
    assert_eq!(part1, 357);
}
#[test]
fn test_part2() {
    let input = read_input("day03-example").unwrap();
    let puzzle = Day3 {};
    let parsed = puzzle.parse_input(&input);
    let part2 = puzzle.solve_part2(&parsed, true);
    assert_eq!(part2, 3121910778619);
}
