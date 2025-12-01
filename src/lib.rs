use std::io::Result;

pub fn read_input(path: &str) -> Result<String> {
    std::fs::read_to_string(format!("inputs/{}.txt", &path))
}

pub trait Solver {
    type Parsed;
    type Part1;
    type Part2;

    fn parse_input(&self, input: &str) -> Self::Parsed;
    fn solve_part1(&self, data: &Self::Parsed, debug: bool) -> Self::Part1;
    fn solve_part2(&self, data: &Self::Parsed, debug: bool) -> Self::Part2;
}
