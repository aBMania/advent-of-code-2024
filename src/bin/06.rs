use crate::Direction::{East, North, South, West};
use advent_of_code::grid::CustomGrid;
use fnv::FnvHashSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Tile::Obstacle),
            &_ => Ok(Tile::Empty),
        }
    }
}

impl Direction {
    #[inline]
    pub fn rotate_right(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    #[inline]

    pub fn apply(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        match (self, row, col) {
            (North, 0, _) | (West, _, 0) => None,
            (North, _, _) => Some((row - 1, col)),
            (East, _, _) => Some((row, col + 1)),
            (South, _, _) => Some((row + 1, col)),
            (West, _, _) => Some((row, col - 1)),
        }
    }
}

fn parse_input(input: &str) -> (CustomGrid<Tile>, (usize, usize)) {
    let mut guard_position = (0, 0);
    let mut cols = 0;
    let lines = input.lines().map(|line| line.trim());

    let grid_data: Vec<Tile> = lines
        .enumerate()
        .flat_map(|(row, line)| {
            cols = line.len();
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => Tile::Obstacle,
                    '.' => Tile::Empty,
                    '^' => {
                        guard_position = (row, col);
                        Tile::Empty
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect();

    let map = CustomGrid::from_vec(grid_data, cols);

    (map, guard_position)
}

fn visit_map(
    map: &CustomGrid<Tile>,
    mut guard_position: (usize, usize),
) -> Option<FnvHashSet<(usize, usize)>> {
    let mut visited: FnvHashSet<(usize, usize)> =
        FnvHashSet::with_capacity_and_hasher(map.rows() * map.cols(), Default::default());

    let mut guard_orientation = North;

    while let Some(_) = map.get(guard_position.0, guard_position.1) {
        visited.insert(guard_position);

        let new_guard_position = match guard_orientation.apply(guard_position) {
            None => break,
            Some(p) => p,
        };

        let at_new_guard_position = map.get(new_guard_position.0, new_guard_position.1);

        if let Some(Tile::Obstacle) = at_new_guard_position {
            guard_orientation = guard_orientation.rotate_right();
        }

        guard_position = match guard_orientation.apply(guard_position) {
            None => break,
            Some(p) => p,
        };
    }

    Some(visited)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, guard_position) = parse_input(input);

    let visited = visit_map(&map, guard_position).unwrap();

    Some(visited.len() as u32)
}

fn is_loop(map: &CustomGrid<Tile>, mut guard_position: (usize, usize)) -> bool {
    let mut visited: FnvHashSet<((usize, usize), Direction)> =
        FnvHashSet::with_capacity_and_hasher(map.rows() * map.cols(), Default::default());

    let mut guard_orientation = North;

    while let Some(_) = map.get(guard_position.0, guard_position.1) {
        if visited.contains(&(guard_position, guard_orientation)) {
            return true;
        }

        let new_guard_position = match guard_orientation.apply(guard_position) {
            None => break,
            Some(p) => p,
        };
        let mut at_new_guard_position = map.get(new_guard_position.0, new_guard_position.1);

        while let Some(Tile::Obstacle) = at_new_guard_position {
            visited.insert((guard_position, guard_orientation));
            guard_orientation = guard_orientation.rotate_right();
            let new_guard_position = match guard_orientation.apply(guard_position) {
                None => break,
                Some(p) => p,
            };
            at_new_guard_position = map.get(new_guard_position.0, new_guard_position.1);
        }

        guard_position = match guard_orientation.apply(guard_position) {
            None => return false,
            Some(p) => p,
        }
    }

    false
}

// 1864 too low
pub fn part_two(input: &str) -> Option<u32> {
    let (map, guard_position) = parse_input(input);

    let visited = visit_map(&map, guard_position).unwrap();

    let solution = visited
        .par_iter()
        .filter(|&&(row, col)| {
            if *map.get(row, col).unwrap() != Tile::Empty {
                return false;
            }
            let mut map = map.clone();
            map[(row, col)] = Tile::Obstacle;
            is_loop(&map, guard_position)
        })
        .count();

    Some(solution as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
