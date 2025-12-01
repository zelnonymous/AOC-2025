use aoc_2025::{Solver, read_input};
use std::io::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Day1;
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
struct DialInput {
    direction: Direction,
    clicks: i32,
}
#[derive(Debug)]
struct State {
    position: i32,
    ended_on_zero: i32,
    passed_zero: i32,
}
impl FromStr for DialInput {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_ch, clicks_str) = s.split_at(1);
        let direction = match dir_ch {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(Error::other("Invalid direction")),
        };
        let clicks: i32 = clicks_str
            .parse()
            .map_err(|_| Error::other("Invalid clicks"))?;
        Ok(DialInput { direction, clicks })
    }
}
impl State {
    fn turn_dial(&mut self, dial_input: &DialInput, debug: bool) {
        let movement = match dial_input.direction {
            Direction::Left => -dial_input.clicks,
            Direction::Right => dial_input.clicks,
        };
        self.position += movement;
        let mut zeroes = 0;
        while self.position < 0 {
            self.position += 100;
            if self.position != 0 {
                zeroes += 1;
            }
        }
        while self.position > 99 {
            self.position -= 100;
            if self.position != 0 {
                zeroes += 1;
            }
        }
        if debug {
            println!(
                "Moved {:?} {} to {}",
                dial_input.direction, dial_input.clicks, self.position
            );
        }
        if self.position == 0 {
            self.ended_on_zero += 1;
            zeroes += 1;
        }
        self.passed_zero += zeroes;
        if debug && zeroes != 0 {
            println!("New zeroes count: {}", self.passed_zero)
        }
    }
}
impl Day1 {
    pub fn execute_turns(&self, data: &[DialInput], debug: bool) -> State {
        if debug {
            println!("Starting at 50");
        }
        let end_state = data.iter().fold(
            State {
                position: 50,
                ended_on_zero: 0,
                passed_zero: 0,
            },
            |mut state, dial_input| {
                state.turn_dial(dial_input, debug);
                state
            },
        );
        if debug {
            dbg!(&end_state);
        }
        end_state
    }
}
impl Solver for Day1 {
    type Parsed = Vec<DialInput>;
    type Part1 = i32;
    type Part2 = i32;
    fn parse_input(&self, input: &str) -> Self::Parsed {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
    }
    fn solve_part1(&self, data: &Self::Parsed, debug: bool) -> Self::Part1 {
        let end_state = self.execute_turns(data, debug);
        end_state.ended_on_zero
    }
    fn solve_part2(&self, data: &Self::Parsed, debug: bool) -> Self::Part2 {
        let end_state = self.execute_turns(data, debug);
        end_state.passed_zero
    }
}

fn main() {
    let input = read_input("day01").unwrap();
    let puzzle = Day1 {};
    let parsed = puzzle.parse_input(&input);
    let part1 = puzzle.solve_part1(&parsed, false);
    let part2 = puzzle.solve_part2(&parsed, false);
    println!("Part 1 Solution: {}", part1);
    println!("Part 2 Solution: {}", part2);
}
#[cfg(test)]
#[test]
fn test_part1() {
    let input = read_input("day01-example").unwrap();
    let puzzle = Day1 {};
    let data = puzzle.parse_input(&input);
    assert_eq!(data.len(), 10);
    assert_eq!(data[0].direction, Direction::Left);
    let part1 = puzzle.solve_part1(&data, true);
    assert_eq!(part1, 3);
}
#[test]
fn test_part2() {
    let input = read_input("day01-example").unwrap();
    let puzzle = Day1 {};
    let data = puzzle.parse_input(&input);
    let part2 = puzzle.solve_part2(&data, true);
    assert_eq!(part2, 6)
}
