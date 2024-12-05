use smallvec::{smallvec, SmallVec};
use std::collections::{HashMap, HashSet};
use std::vec;
use itertools::Itertools;

advent_of_code::solution!(5);

pub fn parse_input(input: &str) -> (HashMap<u32, SmallVec<[u32; 6]>>, Vec<SmallVec<[u32; 6]>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules: Vec<(u32, u32)> = rules
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("|").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();
    
    let mut rules_map: HashMap<u32, SmallVec<[u32; 6]>> = HashMap::new();
    
    for rule in rules {
        rules_map.entry(rule.0).or_insert(smallvec![]).push(rule.1);
    }

    let pages = pages
        .lines()
        .map(|l| l.split(",").map(|page| page.parse().unwrap()).collect())
        .collect();

    (rules_map, pages)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input);

    println!("{rules:?}");

    
    let solution = pages.iter().filter(|&pages| {
        println!("{pages:?}");
        
        for (i, page) in pages.iter().enumerate() {
            if let Some(&ref pages_to_encounter) = rules.get(page) {
                for page_to_encounter in pages_to_encounter {
                    if pages[0..i].contains(&page_to_encounter) {
                        println!("{page_to_encounter} is already seen before {page}");
                        return false;
                    }
                }
            }
        }
        
        true
    }).map(|pages| pages.get(pages.len() / 2).unwrap()).sum();
    
    
    Some(solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
