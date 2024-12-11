use std::mem;
use fnv::FnvHashMap;

advent_of_code::solution!(11);

fn count_stones_after_n_blinks(input: &str, n: usize) -> u64 {
    let mut stones = input
        .split_whitespace()
        .map(|stone| (stone.parse::<u64>().unwrap(), 1))
        .collect::<FnvHashMap<u64, u64>>();

    let mut new_stones: FnvHashMap<u64, u64> = Default::default();

    for _ in 0..n {
        for (stone, n) in stones.iter_mut() {
            if *stone == 0 {
                *new_stones.entry(1).or_insert(0) += *n;
                *n = 0;
                continue;
            }
            let stone_log_10 = stone.ilog10() + 1;
            if stone_log_10 % 2 == 0 {
                let corresponding_power_of_ten = 10u64.pow(stone_log_10 / 2);
                let right_stone = stone % corresponding_power_of_ten;
                let left_stone = stone / corresponding_power_of_ten;

                *new_stones.entry(right_stone).or_insert(0) += *n;
                *new_stones.entry(left_stone).or_insert(0) += *n;
                *n = 0;
                continue;
            }

            *new_stones.entry(*stone * 2024).or_insert(0) += *n;
            *n = 0;
        }

        mem::swap(&mut new_stones, &mut stones);
    }

    stones.values().sum()

}

pub fn part_one(input: &str) -> Option<u64> {
    Some(count_stones_after_n_blinks(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(count_stones_after_n_blinks(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
