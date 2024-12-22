use advent_of_code::grid::{
    input_to_grid, CustomGrid, EAST, NORTH, NORTH_EAST, NORTH_WEST, SOUTH, SOUTH_EAST, SOUTH_WEST,
    WEST,
};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid(input).unwrap();
    let mut counter = 0;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            for (col_offset, row_offset) in [
                NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
            ] {
                let test: Option<String> = (0isize..4isize)
                    .map(|i| {
                        (
                            (row as isize + i * row_offset),
                            (col as isize + i * col_offset),
                        )
                    })
                    .map(|(row, col)| {
                        if col.is_negative() || row.is_negative() {
                            None
                        } else {
                            grid.get(row as usize, col as usize).map(|c| *c)
                        }
                    })
                    .collect();

                if Some("XMAS") == test.as_deref() {
                    counter += 1;
                }
            }
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid(input).unwrap();
    let mut counter = 0;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let diagonal_1: Option<String> = [NORTH_EAST, (0, 0), SOUTH_WEST]
                .iter()
                .map(|(col_offset, row_offset)| {
                    ((col as isize + col_offset), (row as isize + row_offset))
                })
                .map(|(col, row)| {
                    if col.is_negative() || row.is_negative() {
                        None
                    } else {
                        grid.get(row as usize, col as usize).map(|c| *c)
                    }
                })
                .collect();

            let diagonal_2: Option<String> = [SOUTH_EAST, (0, 0), NORTH_WEST]
                .iter()
                .map(|(col_offset, row_offset)| {
                    ((col as isize + col_offset), (row as isize + row_offset))
                })
                .map(|(col, row)| {
                    if col.is_negative() || row.is_negative() {
                        None
                    } else {
                        grid.get(row as usize, col as usize).map(|c| *c)
                    }
                })
                .collect();

            match (diagonal_1.as_deref(), diagonal_2.as_deref()) {
                (Some("SAM"), Some("MAS"))
                | (Some("MAS"), Some("SAM"))
                | (Some("MAS"), Some("MAS"))
                | (Some("SAM"), Some("SAM")) => counter += 1,
                (_, _) => {}
            }
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
