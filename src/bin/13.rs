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

#[derive(Debug)]
struct ParseProblemError;
impl FromStr for Problem {
    type Err = ParseProblemError;
    fn from_str(problem_str: &str) -> Result<Self, <Self as FromStr>::Err> {
        let regex = regex::Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .map_err(|_| ParseProblemError)?;
        let captures = regex.captures(problem_str).ok_or(ParseProblemError)?;

        Ok(Self {
            a_x: captures
                .get(1)
                .ok_or(ParseProblemError)?
                .as_str()
                .parse()
                .map_err(|_| ParseProblemError)?,
            a_y: captures
                .get(2)
                .ok_or(ParseProblemError)?
                .as_str()
                .parse()
                .map_err(|_| ParseProblemError)?,
            b_x: captures
                .get(3)
                .ok_or(ParseProblemError)?
                .as_str()
                .parse()
                .map_err(|_| ParseProblemError)?,
            b_y: captures
                .get(4)
                .ok_or(ParseProblemError)?
                .as_str()
                .parse()
                .map_err(|_| ParseProblemError)?,
            x: captures
                .get(5)
                .ok_or(ParseProblemError)?
                .as_str()
                .parse()
                .map_err(|_| ParseProblemError)?,
            y: captures
                .get(6)
                .ok_or(ParseProblemError)?
                .as_str()
                .parse()
                .map_err(|_| ParseProblemError)?,
        })
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
