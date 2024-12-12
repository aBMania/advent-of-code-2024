use fnv::FnvHashSet;
use advent_of_code::grid::input_to_grid;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input_to_grid::<char>(input).unwrap();
    
    let mut total_price = 0;
    let mut visited = FnvHashSet::<(usize, usize)>::default();
    let mut to_visit = Vec::new();
    let mut to_visit_block = Vec::new();
    
    to_visit.push((0, 0));
    
    while let Some(visiting) = to_visit.pop() {
        if visited.contains(&visiting) {
            continue;
        }
        
        let visiting_value = *map.get(visiting.0, visiting.1).unwrap();
        
        let mut area = 0;
        let mut perimeter = 0;

        to_visit_block.push(visiting);
        
        while let Some(visiting_block) = to_visit_block.pop() {
            if visited.contains(&visiting_block) {
                continue;
            }
            visited.insert(visiting_block);
            
            area += 1;
            
            if visiting_block.0 == 0 || visiting_block.0 == map.rows() - 1 {
                perimeter += 1;
            }
            
            if visiting_block.1 == 0 || visiting_block.1 == map.cols() - 1 {
                perimeter += 1;
            }
            
            for (neighbor_pos, &neighbor_value) in map.iter_neighbors(visiting_block.0, visiting_block.1) {
                // println!("Found neighbor for block {visiting_value} at {neighbor_pos:?}");
                if neighbor_value == visiting_value {
                    if !visited.contains(&neighbor_pos) {
                        to_visit_block.push(neighbor_pos);   
                    }
                } else {
                    if !visited.contains(&neighbor_pos) {
                        to_visit.push(neighbor_pos);
                    }
                    perimeter += 1;
                }
            }
            
        }

        total_price += area * perimeter;
        // println!("Area {visiting_value} has an area of {area} and a perimeter of {perimeter}");
    }
    
    Some(total_price)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
