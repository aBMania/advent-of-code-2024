use crate::Direction::{East, North, South, West};
use advent_of_code::grid::{input_to_grid, CustomGrid};
use rayon::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate_right(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

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

pub fn visit_map(map: &CustomGrid<char>, mut guard_position: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
    let mut visited: HashSet<(usize, usize)> = HashSet::with_capacity(map.rows() * map.cols());

    let mut guard_orientation = North;

    while let Some(_) = map.get(guard_position.0, guard_position.1) {
        visited.insert(guard_position);

        let new_guard_position = match guard_orientation.apply(guard_position) {
            None => break,
            Some(p) => p,
        };

        let at_new_guard_position = map.get(new_guard_position.0, new_guard_position.1);

        if let Some('#') = at_new_guard_position {
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
    let map: CustomGrid<char> = input_to_grid(input).unwrap();

    let (guard_position, _) = map.indexed_iter().find(|(_, &c)| c == '^').unwrap();

    let visited = visit_map(&map, guard_position).unwrap();

    Some(visited.len() as u32)
}

pub fn is_loop(map: &CustomGrid<char>, mut guard_position: (usize, usize)) -> bool {
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::with_capacity(map.rows() * map.cols());

    let mut guard_orientation = North;

    while let Some(_) = map.get(guard_position.0, guard_position.1) {
        if visited.contains(&(guard_position, guard_orientation)) {
            return true;
        }
        visited.insert((guard_position, guard_orientation));

        let new_guard_position = match guard_orientation.apply(guard_position) {
            None => break,
            Some(p) => p,
        };
        let mut at_new_guard_position = map.get(new_guard_position.0, new_guard_position.1);

        while let Some('#') = at_new_guard_position {
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
    let map: CustomGrid<char> = input_to_grid(input).unwrap();

    let (guard_position, _) = map.indexed_iter().find(|(_, &c)| c == '^').unwrap();
    let visited = visit_map(&map, guard_position).unwrap();
    let solution = visited
        .par_iter()
        .filter(|&&(row, col)| {
            if *map.get(row, col).unwrap() != '.' {
                return false;
            }
            let mut map = map.clone();
            map[(row, col)] = '#';
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
