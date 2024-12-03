use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let solution = regex
        .captures_iter(input)
        .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
        .sum();
    Some(solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(do)\(\)|(don't)\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let (solution, _) = regex
        .captures_iter(input)
        .fold((0, true), |(acc, enabled), cap| {
            let enable = cap.get(1);
            let disable = cap.get(2);
            let mul_left = cap.get(3);
            let mul_right = cap.get(4);
            
            if enable.is_some() {
                return (acc, true)
            }
            
            if !enabled || disable.is_some() {
                return (acc, false)
            }

            let mul_left = mul_left.unwrap().as_str().parse::<u32>().unwrap();
            let mul_right = mul_right.unwrap().as_str().parse::<u32>().unwrap();

            (acc + mul_left * mul_right, enabled)
        });
    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
