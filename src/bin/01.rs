advent_of_code::solution!(1);
use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b) = parse_input(input);

    a.sort_unstable();
    b.sort_unstable();

    Some(a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut a, mut b) = parse_input(input);

    a.sort_unstable();
    b.sort_unstable();

    let b_counts = b.iter().counts();

    Some(
        a.iter()
            .map(|element| element * *b_counts.get(element).unwrap_or(&0) as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
