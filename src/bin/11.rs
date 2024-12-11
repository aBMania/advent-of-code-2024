use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = input.split_whitespace().map(|stone|stone.parse::<u64>().unwrap()).collect_vec();
    let mut new_stones = vec![];

    for _ in 0..25 {

        for stone in stones.iter_mut() {
            if *stone == 0 {
                *stone = 1;
                continue;
            }
            let stone_log_10 = stone.ilog10() + 1;
            if stone_log_10 % 2 == 0 {
                let corresponding_power_of_ten = 10u64.pow(stone_log_10 / 2);
                let new_stone = *stone % corresponding_power_of_ten;
                *stone /= corresponding_power_of_ten;
                new_stones.push(new_stone);
                continue;
            }

            *stone *= 2024;
        }

        stones.append(&mut new_stones);
        // println!("{:?}", stones);
    }

    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ilog10() {
        let stone = 123456u32;

        let stone_log_10 = stone.ilog10() + 1;

        println!("{:?}", stone_log_10);
        let corresponding_power_of_ten = 10u32.pow(stone_log_10 / 2);

        println!("{:?}", corresponding_power_of_ten);
        println!("{:?}", stone / corresponding_power_of_ten);
        println!("{:?}", stone % corresponding_power_of_ten);

        assert_eq!(stone_log_10, 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
