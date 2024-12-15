use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Problem {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    x: i64,
    y: i64,
}

impl Problem {
    fn solve(&self) -> Option<(i64, i64)> {
        let divisor = self.a_x * self.b_y - self.a_y * self.b_x;
        let dividend = self.a_x * self.y - self.a_y * self.x;

        if dividend % divisor != 0 {
            return None;
        }

        let b = dividend / divisor;

        let divisor = self.a_x;
        let dividend = self.x - self.b_x * b;

        if dividend % divisor != 0 {
            return None;
        }

        let a = dividend / divisor;

        Some((a, b))
    }
}

impl Problem {
    const PATTERN: [char; 4] = ['+', ',', '=', '\n'];
}

#[derive(Debug)]
struct ParseProblemError;
impl FromStr for Problem {
    type Err = ParseProblemError;
    fn from_str(problem_str: &str) -> Result<Self, <Self as FromStr>::Err> {
        // Thanks to https://github.com/ndunnett/aoc/blob/ea2e0abf0e4a97aed5a2c55976c54e9de6f819e5/rust/2024/src/bin/day13.rs#L17 for parsing
        if let Some((a_x, a_y, b_x, b_y, x, y)) = problem_str
            .split(&Self::PATTERN)
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<i64>().expect("failed to parse number"))
            .next_tuple()
        {
            Ok(Self {
                a_x,
                a_y,
                b_x,
                b_y,
                x,
                y,
            })
        } else {
            Err(ParseProblemError)
        }
    }
}
pub fn part_one(input: &str) -> Option<i64> {
    let total_cost = input
        .split("\n\n")
        .map(|block| block.parse::<Problem>().unwrap())
        .filter_map(|problem| problem.solve())
        .filter(|&(a, b)| a <= 100 && b <= 100)
        .map(|(a, b)| a * 3 + b)
        .sum();

    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<i64> {
    let total_cost = input
        .split("\n\n")
        .map(|block| {
            let mut problem = block.parse::<Problem>().unwrap();
            problem.x += 10000000000000;
            problem.y += 10000000000000;
            problem
        })
        .filter_map(|problem| problem.solve())
        .map(|(a, b)| a * 3 + b)
        .sum();

    Some(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
