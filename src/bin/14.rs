use advent_of_code::grid::CustomGrid;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

advent_of_code::solution!(14);

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

#[derive(Debug)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    const PATTERN: [char; 4] = [' ', ',', '=', '\n'];

    fn position_after_seconds(&self, n: i64) -> (i64, i64) {
        (self.px + self.vx * n, self.py + self.vy * n)
    }
}

#[derive(Debug)]
struct ParseProblemError;
impl FromStr for Robot {
    type Err = ParseProblemError;
    fn from_str(problem_str: &str) -> Result<Self, <Self as FromStr>::Err> {
        if let Some((px, py, vx, vy)) = problem_str
            .split(&Self::PATTERN)
            .skip(1)
            .filter_map(|s| s.parse::<i64>().ok())
            .next_tuple()
        {
            Ok(Self { px, py, vx, vy })
        } else {
            Err(ParseProblemError)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    _part_one(input, WIDTH, HEIGHT)
}

pub fn _part_one(input: &str, width: i64, height: i64) -> Option<u32> {
    let (top_left, top_right, bottom_left, bottom_right) = input
        .lines()
        .map(|s| s.parse::<Robot>().unwrap())
        .map(|robot| robot.position_after_seconds(100))
        .map(|(x, y)| (x.rem_euclid(width), y.rem_euclid(height)))
        .fold(
            (0, 0, 0, 0),
            |(top_left, top_right, bottom_left, bottom_right), (x, y)| match (x, y) {
                (x, y) if x > width / 2 && y > height / 2 => {
                    (top_left, top_right, bottom_left, bottom_right + 1)
                }
                (x, y) if x < width / 2 && y > height / 2 => {
                    (top_left, top_right + 1, bottom_left, bottom_right)
                }
                (x, y) if x > width / 2 && y < height / 2 => {
                    (top_left, top_right, bottom_left + 1, bottom_right)
                }
                (_, _) if x < width / 2 && y < height / 2 => {
                    (top_left + 1, top_right, bottom_left, bottom_right)
                }
                (_, _) => (top_left, top_right, bottom_left, bottom_right),
            },
        );

    // println!("{top_left}, {top_right}, {bottom_left}, {bottom_right}");
    Some(top_left * top_right * bottom_left * bottom_right)
}

pub fn part_two(input: &str) -> Option<u32> {
    _part_two(input, WIDTH, HEIGHT)
}

pub fn _part_two(input: &str, width: i64, height: i64) -> Option<u32> {
    let robots = input
        .lines()
        .map(|s| s.parse::<Robot>().unwrap())
        .collect_vec();

    for i in (0..10000) {
        let iteration_robots = robots
            .iter()
            .map(|robot| robot.position_after_seconds(i))
            .map(|(x, y)| (x.rem_euclid(width), y.rem_euclid(height)))
            .collect::<HashSet<_>>();

        if iteration_robots
            .iter()
            .filter(|&&(x, y)| iteration_robots.contains(&(WIDTH - x, y)))
            .count()
            > 80
        {
            let mut grid = CustomGrid::from_value((WIDTH as usize, HEIGHT as usize), ' ');

            // for &(x, y) in &iteration_robots {
            //     let robot_square = grid.get_mut(y.rem_euclid(height), x.rem_euclid(width));
            //     if let Some(robot_square) = robot_square {
            //         *robot_square = '#';
            //     };
            // }

            //             println!("{}", i);
            //             grid.print();
            return Some(i as u32);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7861));
    }
}
