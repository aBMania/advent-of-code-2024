use advent_of_code::grid::input_to_grid;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    _part_one(input, 100)
}
pub fn _part_one(input: &str, threshold: usize) -> Option<u32> {
    let map = input_to_grid::<char>(input).expect("Parse error");

    let start = map
        .indexed_iter()
        .find(|(_, v)| **v == 'S')
        .map(|(pos, _)| pos)
        .expect("No start found");

    let mut path_index = FnvHashMap::<_, _>::default();

    path_index.insert(start, 0);
    let mut previous = start;

    while let Some((current, v)) = map
        .iter_neighbors(previous.0, previous.1)
        .find(|(pos, v)| (**v != '#') && !path_index.contains_key(pos))
    {
        path_index.insert(current, path_index.len());
        previous = current;
    }

    Some(
        map.indexed_iter()
            .filter(|(pos, _)| {
                if let Some((p1, p2)) = map
                    .iter_neighbors(pos.0, pos.1)
                    .filter(|(_, v)| **v != '#')
                    .map(|(p, _)| path_index[&p])
                    .collect_tuple()
                {
                    return p1.abs_diff(p2) - 2 >= threshold;
                }
                return false;
            })
            .count() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(&advent_of_code::template::read_file("examples", DAY), 30);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
