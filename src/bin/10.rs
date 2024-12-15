use advent_of_code::grid::{input_to_grid, CustomGrid};
use fnv::FnvHashSet;
use itertools::Itertools;

advent_of_code::solution!(10);

fn get_attainable(map: &CustomGrid<u8>, summit: (usize, usize)) -> u32 {
    let mut n_attainable = 0;
    let mut visited: FnvHashSet<(usize, usize)> = Default::default();
    let mut to_visit = vec![summit];

    while let Some((row, col)) = to_visit.pop() {
        let value = *map.get(row, col).unwrap();
        visited.insert((row, col));
        map.iter_neighbors(row, col)
            .filter(|(neighbor_pos, &neighbor)| {
                neighbor + 1 == value && !visited.contains(neighbor_pos)
            })
            .for_each(|(neighbor_pos, &neighbor)| {
                to_visit.push(neighbor_pos);
                if neighbor == 0 {
                    n_attainable += 1;
                };
            });
    }

    n_attainable
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input_to_grid::<u8>(input).unwrap();

    let starts = map
        .indexed_iter()
        .filter_map(|(index, &altitude)| match altitude {
            9 => Some(index),
            _ => None,
        })
        .map(|summit| get_attainable(&map, summit))
        .sum();

    Some(starts)
}

fn count_trails_from_trailhead(map: &CustomGrid<u8>, (row, col): (usize, usize)) -> u32 {
    let current_position = *map.get(row, col).unwrap();
    if current_position == 9 {
        1
    } else {
        map.iter_neighbors(row, col)
            .filter(|(neighbor_pos, &neighbor)| {
                neighbor == current_position + 1
            })
            .map(|(neighbor_pos, &neighbor)| count_trails_from_trailhead(&map, neighbor_pos))
            .sum()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input_to_grid::<u8>(input).unwrap();

    let trail_count = map
        .indexed_iter()
        .filter_map(|(index, &altitude)| match altitude {
            0 => Some(index),
            _ => None,
        })
        .map(|summit| count_trails_from_trailhead(&map, summit))
        .sum();

    Some(trail_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
