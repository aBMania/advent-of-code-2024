use itertools::Itertools;

advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (total, operands) = line.split_once(": ").unwrap();

            (
                total.parse().unwrap(),
                operands
                    .split_whitespace()
                    .map(|o| o.parse().unwrap())
                    .collect(),
            )
        })
        .collect_vec()
}

fn is_solvable(total: u64, operands: &[u64]) -> bool {
    let n = operands.len();
    if n == 1 {
        return total == operands[0];
    }

    if operands.iter().sum::<u64>() == total {
        return true;
    }

    if operands.iter().product::<u64>() == total {
        return true;
    }

    if n == 2 {
        return false;
    }

    if operands[n - 1] < total && is_solvable(total - operands[n - 1], &operands[0..n - 1]) {
        return true;
    }

    if total % operands[n - 1] == 0 && is_solvable(total / operands[n - 1], &operands[0..n - 1]) {
        return true;
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let solution = input
        .into_iter()
        .map(|(total, operands)| match is_solvable(total, &operands) {
            true => total,
            false => 0,
        })
        .sum();

    Some(solution)
}

fn is_solvable_part_2(total: u64, operands: &[u64]) -> bool {
    let n = operands.len();
    if n == 1 {
        return total == operands[0];
    }

    if operands.iter().sum::<u64>() == total {
        return true;
    }

    if operands.iter().product::<u64>() == total {
        return true;
    }

    if operands[n - 1] < total && is_solvable_part_2(total - operands[n - 1], &operands[0..n - 1]) {
        return true;
    }

    if total % operands[n - 1] == 0
        && is_solvable_part_2(total / operands[n - 1], &operands[0..n - 1])
    {
        return true;
    }

    let log10 = operands[n - 1].ilog10() + 1;
    if total % 10u64.pow(log10) == operands[n - 1]
        && is_solvable_part_2(total / 10u64.pow(log10), &operands[0..n - 1])
    {
        return true;
    }

    false
}
pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let solution = input
        .into_iter()
        .map(
            |(total, operands)| match is_solvable_part_2(total, &operands) {
                true => total,
                false => 0,
            },
        )
        .sum();

    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solvable() {
        assert_eq!(is_solvable(10, &[1, 2, 3, 4]), true);
        assert_eq!(is_solvable(10, &[5, 2]), true);
        assert_eq!(is_solvable(10, &[2, 2, 6]), true);
        assert_eq!(is_solvable(3267, &[81, 40, 27]), true);
        assert_eq!(is_solvable(292, &[11, 6, 16, 20]), true);
        assert_eq!(is_solvable(272, &[11, 6, 16]), true);
    }

    #[test]
    fn test_is_solvable_part_2() {
        assert_eq!(is_solvable_part_2(156, &[15, 6]), true);
        assert_eq!(is_solvable_part_2(7290, &[6, 8, 6, 15]), true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
