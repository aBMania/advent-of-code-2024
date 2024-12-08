use advent_of_code::grid::input_to_grid;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::iproduct;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input_to_grid::<char>(input).unwrap();

    let antipodes = map
        .indexed_iter()
        .filter_map(|((row, col), &c)| match c {
            '.' => None,
            _ => Some((c, (row, col))),
        })
        .fold(FnvHashMap::default(), |mut acc, (c, (row, col))| {
            acc.entry(c).or_insert_with(Vec::new).push((row, col));
            acc
        })
        .iter()
        .fold(FnvHashSet::default(), |mut antipodes, (_, antennas)| {
            iproduct!(antennas, antennas)
                .filter(|(a_1, a_2)| a_1 != a_2)
                .for_each(|(&(row_a, col_a), &(row_b, col_b))| {
                    let row_a = row_a as isize;
                    let row_b = row_b as isize;
                    let col_a = col_a as isize;
                    let col_b = col_b as isize;
                    let row_diff = row_a - row_b;
                    let col_diff = col_a - col_b;

                    if row_a + row_diff < map.rows() as isize
                        && row_a + row_diff >= 0
                        && col_a + col_diff < map.cols() as isize
                        && col_a + col_diff >= 0
                    {
                        antipodes.insert((row_a + row_diff, col_a + col_diff));
                    }
                    if row_b - row_diff < map.rows() as isize
                        && row_b - row_diff >= 0
                        && col_b - col_diff < map.cols() as isize
                        && col_b - col_diff >= 0
                    {
                        antipodes.insert((row_b - row_diff, col_b - col_diff));
                    }
                });

            antipodes
        });

    Some(antipodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input_to_grid::<char>(input).unwrap();

    let antipodes = map
        .indexed_iter()
        .filter_map(|((row, col), &c)| match c {
            '.' => None,
            _ => Some((c, (row, col))),
        })
        .fold(FnvHashMap::default(), |mut acc, (c, (row, col))| {
            acc.entry(c).or_insert_with(Vec::new).push((row, col));
            acc
        })
        .iter()
        .fold(FnvHashSet::default(), |mut antipodes, (_, antennas)| {
            iproduct!(antennas, antennas)
                .filter(|(a_1, a_2)| a_1 != a_2)
                .for_each(|(&(row_a, col_a), &(row_b, col_b))| {
                    let row_a = row_a as isize;
                    let row_b = row_b as isize;
                    let col_a = col_a as isize;
                    let col_b = col_b as isize;
                    let row_diff = row_a - row_b;
                    let col_diff = col_a - col_b;

                    let mut k = 0;
                    while row_a + row_diff * k < map.rows() as isize
                        && row_a + row_diff * k >= 0
                        && col_a + col_diff * k < map.cols() as isize
                        && col_a + col_diff * k >= 0
                    {
                        antipodes.insert((row_a + row_diff * k, col_a + col_diff * k));
                        k += 1;
                    }
                });

            antipodes
        });

    // for (row, col) in &antipodes {
    //     map[(*row as usize, *col as usize)] = '#';
    // }
    //
    // println!("{:?}", map);

    Some(antipodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
