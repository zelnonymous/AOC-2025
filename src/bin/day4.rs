use aoc_2025::{Solver, read_input};

#[derive(Debug)]
struct Day4 {}

impl Day4 {
    fn is_safe_roll(
        &self,
        data: &<Day4 as Solver>::Parsed,
        adjacent_count: usize,
        max_adjacent_rolls: usize,
        x: usize,
        y: usize,
    ) -> bool {
        let row_count = data.len();
        let col_count = data[y].len();
        let min_row = y.saturating_sub(adjacent_count);
        let max_row = (y + adjacent_count).min(row_count - 1);
        let min_col = x.saturating_sub(adjacent_count);
        let max_col = (x + adjacent_count).min(col_count - 1);
        let mut adjacent_rolls = 0;
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                if row == y && col == x {
                    continue;
                }
                if data[row][col] == '@' {
                    adjacent_rolls += 1;
                }
            }
        }
        adjacent_rolls < max_adjacent_rolls
    }
    fn remove_safe_rolls(
        &self,
        data: &mut Vec<Vec<char>>,
        adjacent_count: usize,
        max_adjacent_rolls: usize,
    ) -> i32 {
        let mut safe_rolls = 0;
        let mut removals: Vec<(usize, usize)> = Vec::new();
        for (y, row) in data.iter().enumerate() {
            for (x, &_) in row.iter().enumerate() {
                if row[x] != '@' {
                    continue;
                }
                if self.is_safe_roll(data, adjacent_count, max_adjacent_rolls, x, y) {
                    safe_rolls += 1;
                    removals.push((y, x));
                }
            }
        }
        for (ry, rx) in removals {
            data[ry][rx] = '.';
        }
        safe_rolls
    }
}

impl Solver for Day4 {
    type Parsed = Vec<Vec<char>>;
    type Part1 = i32;
    type Part2 = i32;
    fn parse_input(&self, input: &str) -> Self::Parsed {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect()
    }
    fn solve_part1(&self, data: &Self::Parsed, debug: bool) -> Self::Part1 {
        let mut editable = data.clone();
        let removals = self.remove_safe_rolls(&mut editable, 1, 4);
        if debug {
            println!("Part 1: {} Removals", removals)
        }
        removals
    }
    fn solve_part2(&self, data: &Self::Parsed, debug: bool) -> Self::Part2 {
        let mut editable = data.clone();
        let mut total_removals = 0;
        loop {
            let removals = self.remove_safe_rolls(&mut editable, 1, 4);
            if debug {
                println!("Part 2: {} Removals", removals)
            }
            if removals == 0 {
                break;
            }
            total_removals += removals
        }
        total_removals
    }
}
fn main() {
    let input = read_input("day04").unwrap();
    let puzzle = Day4 {};
    let parsed = puzzle.parse_input(&input);
    let part1 = puzzle.solve_part1(&parsed, false);
    println!("Part 1 Solution: {}", part1);
    let part2 = puzzle.solve_part2(&parsed, false);
    println!("Part 2 Solution: {}", part2);
}
#[cfg(test)]
#[test]
fn test_part1() {
    let input = read_input("day04-example").unwrap();
    let puzzle = Day4 {};
    let parsed = puzzle.parse_input(&input);
    assert_eq!(parsed[1][1], '@');
    let part1 = puzzle.solve_part1(&parsed, true);
    assert_eq!(part1, 13)
}
#[test]
fn test_part2() {
    let input = read_input("day04-example").unwrap();
    let puzzle = Day4 {};
    let parsed = puzzle.parse_input(&input);
    let part2 = puzzle.solve_part2(&parsed, true);
    assert_eq!(part2, 43)
}
