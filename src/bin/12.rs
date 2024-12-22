use advent_of_code::grid::{input_to_grid, EAST, NORTH, SOUTH, WEST};
use fnv::FnvHashSet;

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

            for (neighbor_pos, &neighbor_value) in
                map.iter_neighbors(visiting_block.0, visiting_block.1)
            {
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
    }

    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
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
        let mut corners = 0;
        let mut left_borders_to_visit = FnvHashSet::<(usize, usize)>::default();

        to_visit_block.push(visiting);

        while let Some(visiting_block) = to_visit_block.pop() {
            if visited.contains(&visiting_block) {
                continue;
            }
            visited.insert(visiting_block);

            area += 1;

            let left = map.left(visiting_block.0, visiting_block.1);

            // Store left borders
            if left.is_none() || left != Some(&visiting_value) {
                left_borders_to_visit.insert(visiting_block);
            }

            for (neighbor_pos, &neighbor_value) in
                map.iter_neighbors(visiting_block.0, visiting_block.1)
            {
                if neighbor_value == visiting_value {
                    if !visited.contains(&neighbor_pos) {
                        to_visit_block.push(neighbor_pos);
                    }
                } else {
                    if !visited.contains(&neighbor_pos) {
                        to_visit.push(neighbor_pos);
                    }
                }
            }
        }

        // println!("{visiting_value}: found {} left border(s)", left_borders_to_visit.len());

        // Visit all the borders
        while let Some(initial_left_border) = left_borders_to_visit.iter().next().cloned() {
            left_borders_to_visit.remove(&initial_left_border);

            // Walk the border from this position
            let mut walker_position = initial_left_border;
            let mut walker_orientation = NORTH;
            let mut started = false;

            while !started || walker_position != initial_left_border || walker_orientation != NORTH {
                started = true;


                // println!("Inner loop start: ({walker_position:?}, {walker_orientation:?}). Remaining: {:?}", left_borders_to_visit);

                // Remove the current position from left border list
                // This may be optional, try removing it after solving
                if walker_orientation == NORTH {
                    // We are still on the border
                    left_borders_to_visit.remove(&walker_position);
                }


                // Position at the left of the walker (based on orientation)
                let walker_left_position = match walker_orientation {
                    NORTH => (walker_position.0 as isize + WEST.0, walker_position.1 as isize + WEST.1),
                    WEST => (walker_position.0 as isize + SOUTH.0, walker_position.1 as isize + SOUTH.1),
                    SOUTH => (walker_position.0 as isize + EAST.0, walker_position.1 as isize + EAST.1),
                    EAST => (walker_position.0 as isize + NORTH.0, walker_position.1 as isize + NORTH.1),
                    _ => unreachable!(),
                };

                // Value at the left of the walker (based on orientation)
                let walker_left = map.get(walker_left_position.0, walker_left_position.1);

                // Handle left hand turn (aka a corner)
                // XXX
                // OOX <- we are on this kind of situation
                // XOX
                if walker_left == Some(&visiting_value) {
                    // Go past the corner
                    walker_position = (walker_left_position.0 as usize, walker_left_position.1 as usize);

                    // println!("{visiting_value}: rotate left on {walker_position:?}");

                    // Rotate left
                    corners += 1;
                    walker_orientation = match walker_orientation {
                        NORTH => WEST,
                        WEST => SOUTH,
                        SOUTH => EAST,
                        EAST => NORTH,
                        _ => unreachable!()
                    };

                    // println!("Inner loop end: ({walker_position:?}, {walker_orientation:?})");
                    // println!("Remaining borders ({}): {:?}", left_borders_to_visit.len(), left_borders_to_visit);
                    continue;
                }

                // Walker next position, based on current orientation
                let walker_next_position = (walker_position.0 as isize + walker_orientation.0, walker_position.1 as isize + walker_orientation.1);
                let walker_next = map.get(walker_next_position.0, walker_next_position.1);

                (walker_position, walker_orientation) = match walker_next {
                    // XOX
                    // XOX  <- we are on this kind of situation
                    // XOX
                    Some(c) if c == &visiting_value => {
                        ((walker_next_position.0 as usize, walker_next_position.1 as usize), walker_orientation)
                    }
                    None | Some(_) => {
                        // XXX
                        // XOO  <- we are on this kind of situation
                        // XOX

                        corners += 1;

                        // println!("{visiting_value}: rotate right on {walker_position:?}");

                        // Rotate right, do not move
                        (walker_position, match walker_orientation {
                            NORTH => EAST,
                            EAST => SOUTH,
                            SOUTH => WEST,
                            WEST => NORTH,
                            _ => unreachable!()
                        })
                    }
                };
            }
            // println!("Inner loop end: ({walker_position:?}, {walker_orientation:?})");
            // println!("Remaining borders ({}): {:?}", left_borders_to_visit.len(), left_borders_to_visit);
        }

        // Add to the total price
        total_price += area * corners;

        // println!("{visiting_value}: area {area}, corners: {corners}")
    }

    Some(total_price)
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
        assert_eq!(result, Some(1206));
    }
    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(80));
    }
    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(236));
    }
    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(368));
    }
}
