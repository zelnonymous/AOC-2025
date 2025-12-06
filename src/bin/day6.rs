use aoc_2025::{Solver, read_input};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Addition,
    Multiplication,
    Unknown,
}
impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Addition => write!(f, "+"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Unknown => write!(f, "?"),
        }
    }
}
struct Day6 {}
#[derive(Debug)]
struct Problem {
    inputs: Vec<u64>,
    operator: Operator,
}
impl Problem {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            operator: Operator::Unknown,
        }
    }
}
impl Solver for Day6 {
    type Parsed = (Vec<Problem>, Vec<Problem>);
    type Part1 = u64;
    type Part2 = u64;
    fn parse_input(&self, input: &str) -> Self::Parsed {
        let mut problems: Vec<Problem> = Vec::new();
        let mut problems_corrected: Vec<Problem> = Vec::new();
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut current_problem = Problem::new();
        if matrix.is_empty() {
            panic!("Input is unexpectedly empty!")
        }
        let max_x = matrix.iter().map(|row| row.len() - 1).max().unwrap();
        for (x, &_) in matrix[0].iter().enumerate() {
            if matrix.iter().all(|c| c[x] == ' ') {
                if !current_problem.inputs.is_empty() {
                    problems_corrected.push(current_problem);
                    current_problem = Problem::new();
                }
                continue;
            }
            let mut current_number = String::from("");
            for row in matrix.iter() {
                match row[x].to_digit(10) {
                    Some(_) => current_number.push(row[x]),
                    None => match row[x] {
                        '*' => current_problem.operator = Operator::Multiplication,
                        '+' => current_problem.operator = Operator::Addition,
                        _ => continue,
                    },
                }
            }
            current_problem
                .inputs
                .push(current_number.parse::<u64>().unwrap());
            if x == max_x {
                problems_corrected.push(current_problem);
                current_problem = Problem::new();
            }
        }
        for line in input.lines() {
            for (idx, token) in line.split_whitespace().enumerate() {
                if problems.is_empty() || problems.len() - 1 < idx {
                    problems.push(Problem::new());
                }
                match token.parse::<u64>() {
                    Ok(input) => problems[idx].inputs.push(input),
                    Err(_) => match token {
                        "*" => problems[idx].operator = Operator::Multiplication,
                        "+" => problems[idx].operator = Operator::Addition,
                        other => println!("Unexpected token: {}", other),
                    },
                }
            }
        }
        (problems, problems_corrected)
    }
    fn solve_part1(&self, data: &Self::Parsed, debug: bool) -> Self::Part1 {
        let (input, _) = data;
        self.solve_problems(input, debug)
    }
    fn solve_part2(&self, data: &Self::Parsed, debug: bool) -> Self::Part2 {
        let (_, corrected) = data;
        self.solve_problems(corrected, debug)
    }
}
impl Day6 {
    fn solve_problems(&self, data: &Vec<Problem>, debug: bool) -> u64 {
        let mut total = 0;
        for problem in data {
            let start = problem.inputs[0];
            let result = problem.inputs[1..]
                .iter()
                .fold(start, |acc, i| match problem.operator {
                    Operator::Addition => acc + i,
                    Operator::Multiplication => acc * i,
                    Operator::Unknown => acc,
                });
            if debug {
                let problemstr = problem
                    .inputs
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(&format!(" {} ", problem.operator));
                println!("{} = {}", problemstr, result);
            }
            total += result;
        }
        total
    }
}
fn main() {
    let input = read_input("day06").unwrap();
    let puzzle = Day6 {};
    let data = puzzle.parse_input(&input);
    let part1 = puzzle.solve_part1(&data, false);
    println!("Part 1 Solution: {}", part1);
    let part2 = puzzle.solve_part2(&data, false);
    println!("Part 2 Solution: {}", part2);
}
#[cfg(test)]
#[test]
fn test_part1() {
    let input = read_input("day06-example").unwrap();
    let puzzle = Day6 {};
    let (data, corrected) = puzzle.parse_input(&input);
    assert_eq!(data.len(), 4);
    assert_eq!(data[0].operator, Operator::Multiplication);
    assert_eq!(data[0].inputs.len(), 3);
    let part1 = puzzle.solve_part1(&(data, corrected), true);
    assert_eq!(part1, 4277556);
}
#[test]
fn test_part2() {
    let input = read_input("day06-example").unwrap();
    let puzzle = Day6 {};
    let data = puzzle.parse_input(&input);
    let part2 = puzzle.solve_part2(&data, true);
    assert_eq!(part2, 3263827);
}
