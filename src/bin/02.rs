advent_of_code::solution!(2);

fn is_safe_report(report: &Vec<u32>) -> bool {
    let ascending = report[1] > report[0];
    let mut report_iterator = report.iter();
    let mut prev = *report_iterator.next().unwrap();

    while let Some(&next) = report_iterator.next() {
        let valid = is_valid_pairwise(ascending, prev, next);
        if !valid {
            return false;
        }
        prev = next;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect::<_>();

    let safe_report_count = reports
        .iter()
        .filter(|&report| is_safe_report(report))
        .count() as u32;
    Some(safe_report_count)
}
pub fn is_valid_pairwise(ascending: bool, prev: u32, next: u32) -> bool {
    let diff = prev.abs_diff(next);
    if diff == 0 || diff > 3 {
        return false;
    }
    match (ascending, next > prev) {
        (true, true) | (false, false) => true,
        _ => false,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect::<_>();

    let safe_report_count = reports
        .iter()
        .filter(|&report| {
            if is_safe_report(report) {
                return true;
            }

            for i_skip in 0..report.len() {
                let report_with_skip = &report
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != i_skip)
                    .map(|(_, &v)| v)
                    .collect();
                if is_safe_report(report_with_skip) {
                    return true;
                }
            }

            return false;
        })
        .count() as u32;
    Some(safe_report_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
