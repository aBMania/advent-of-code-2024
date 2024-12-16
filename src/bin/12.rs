use advent_of_code::grid::{input_to_grid, EAST, NORTH, SOUTH, WEST};
use fnv::FnvHashSet;
use itertools::enumerate;
use smallvec::{smallvec, SmallVec};
use std::collections::HashSet;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input_to_grid::<char>(input).unwrap();

    let mut total_price = 0;
    let mut visited = FnvHashSet::<(usize, usize)>::default();
    let mut to_visit = Vec::new();
    let mut to_visit_block = Vec::new();

    to_visit.push((0, 0));

    while let Some(visiting) = to_visit.pop() {
        if visited.contains(&visiting) {
            continue;
        }

        let visiting_value = *map.get(visiting.0, visiting.1).unwrap();

        let mut area = 0;
        let mut perimeter = 0;

        to_visit_block.push(visiting);

        while let Some(visiting_block) = to_visit_block.pop() {
            if visited.contains(&visiting_block) {
                continue;
            }
            visited.insert(visiting_block);

            area += 1;

            if visiting_block.0 == 0 || visiting_block.0 == map.rows() - 1 {
                perimeter += 1;
            }

            if visiting_block.1 == 0 || visiting_block.1 == map.cols() - 1 {
                perimeter += 1;
            }

            for (neighbor_pos, &neighbor_value) in
                map.iter_neighbors(visiting_block.0, visiting_block.1)
            {
                if neighbor_value == visiting_value {
                    if !visited.contains(&neighbor_pos) {
                        to_visit_block.push(neighbor_pos);
                    }
                } else {
                    if !visited.contains(&neighbor_pos) {
                        to_visit.push(neighbor_pos);
                    }
                    perimeter += 1;
                }
            }
        }

        total_price += area * perimeter;
    }

    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input_to_grid::<char>(input).unwrap();

    let mut total_price = 0;
    let mut visited = FnvHashSet::<(usize, usize)>::default();
    let mut to_visit = Vec::new();
    let mut to_visit_block = Vec::new();

    to_visit.push((0, 0));

    while let Some(visiting) = to_visit.pop() {
        if visited.contains(&visiting) {
            continue;
        }

        let visiting_value = *map.get(visiting.0, visiting.1).unwrap();

        let mut area = 0;
        let mut n_corners = 0;

        to_visit_block.push(visiting);

        while let Some(visiting_block) = to_visit_block.pop() {
            if visited.contains(&visiting_block) {
                continue;
            }
            visited.insert(visiting_block);

            area += 1;

            let mut same_value_neighbors = [false, false, false, false];

            for (i, (neighbor_pos, &neighbor_value)) in [NORTH, EAST, SOUTH, WEST]
                .iter()
                .map(move |(col_offset, row_offset)| {
                    (
                        (visiting_block.0 as isize + row_offset),
                        (visiting_block.1 as isize + col_offset),
                    )
                })
                .enumerate()
                .filter_map(|(i, (row, col))| {
                    if col.is_negative() || row.is_negative() {
                        None
                    } else {
                        map.get(row as usize, col as usize)
                            .map(|val| (i, ((row as usize, col as usize), val)))
                    }
                })
            {
                same_value_neighbors[i] = neighbor_value == visiting_value;

                if neighbor_value == visiting_value {
                    if !visited.contains(&neighbor_pos) {
                        to_visit_block.push(neighbor_pos);
                    }
                } else {
                    if !visited.contains(&neighbor_pos) {
                        to_visit.push(neighbor_pos);
                    }
                }
            }

            n_corners += match same_value_neighbors {
                a @ _ if a.iter().map(|&a| a as u8).sum::<u8>() == 0 => 4,
                a @ _ if a.iter().map(|&a| a as u8).sum::<u8>() == 1 => 2,
                [true, true, false, false] => {
                    match *map
                        .get(
                            visiting_block.0 as isize + NORTH.1 + EAST.1,
                            visiting_block.1 as isize + NORTH.0 + EAST.0,
                        )
                        .unwrap()
                        == visiting_value
                    {
                        true => 1,
                        false => 2,
                    }
                }
                [false, true, true, false] => {
                    match *map
                        .get(
                            visiting_block.0 as isize + SOUTH.1 + EAST.1,
                            visiting_block.1 as isize + SOUTH.0 + EAST.0,
                        )
                        .unwrap()
                        == visiting_value
                    {
                        true => 1,
                        false => 2,
                    }
                }
                [false, false, true, true] => {
                    match *map
                        .get(
                            visiting_block.0 as isize + SOUTH.1 + WEST.1,
                            visiting_block.1 as isize + SOUTH.0 + WEST.0,
                        )
                        .unwrap()
                        == visiting_value
                    {
                        true => 1,
                        false => 2,
                    }
                }
                [true, false, false, true] => {
                    match *map
                        .get(
                            visiting_block.0 as isize + NORTH.1 + WEST.1,
                            visiting_block.1 as isize + NORTH.0 + WEST.0,
                        )
                        .unwrap()
                        == visiting_value
                    {
                        true => 1,
                        false => 2,
                    }
                }
                _ => 0,
            };
        }

        total_price += area * n_corners;
        println!("Area {visiting_value} has an area of {area} and {n_corners} corners");
    }

    Some(total_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(80));
    }
    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(236));
    }
    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(368));
    }
}
