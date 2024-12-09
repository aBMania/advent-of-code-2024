use itertools::Itertools;
use smallvec::{smallvec, SmallVec};
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;

advent_of_code::solution!(5);

#[derive(PartialEq, Eq, Debug, Ord)]
struct Page {
    number: u32,
    must_be_before: SmallVec<[u32; 10]>,
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.must_be_before.contains(&other.number) {
            Some(Ordering::Less)
        } else if other.must_be_before.contains(&self.number) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Page>> {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules: Vec<(u32, u32)> = rules
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("|").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    let mut rules_map: HashMap<u32, SmallVec<[u32; 10]>> = HashMap::new();

    for rule in rules {
        rules_map.entry(rule.0).or_insert(smallvec![]).push(rule.1);
    }

    let pages = pages
        .lines()
        .map(|l| {
            l.split(",")
                .map(|page| {
                    let page_number = page.parse().unwrap();
                    Page {
                        number: page_number,
                        must_be_before: rules_map
                            .get(&page_number)
                            .map(|sm| sm.clone())
                            .unwrap_or_default(),
                    }
                })
                .collect()
        })
        .collect();

    pages
}

pub fn part_one(input: &str) -> Option<u32> {
    let pages = parse_input(input);

    let solution = pages
        .iter()
        .filter(|&pages| pages.windows(2).all(|w| w[0] <= w[1]))
        .map(|pages| pages.get(pages.len() / 2).unwrap().number)
        .sum();

    Some(solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pages = parse_input(input);

    let solution = pages
        .iter()
        .filter(|&pages| !pages.windows(2).all(|w| w[0] <= w[1]))
        .map(|pages| pages.iter().sorted().collect::<Vec<&Page>>())
        .map(|pages| pages.get(pages.len() / 2).unwrap().number)
        .sum();

    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_gt() {
        let page_1 = Page {
            number: 1,
            must_be_before: smallvec![2, 3],
        };
        let page_2 = Page {
            number: 2,
            must_be_before: smallvec![],
        };

        assert_eq!(page_1 < page_2, true);
        assert_eq!(page_2 > page_1, true);
        assert_eq!(page_2 > page_1, true);
        assert_eq!(page_1 < page_2, true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
