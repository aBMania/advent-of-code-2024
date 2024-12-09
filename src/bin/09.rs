advent_of_code::solution!(9);

/// Compute integer sum from a to b (excluded)
///
/// # Examples
///
///
/// ```
/// assert_eq!(sum_from_to(0, 4), 1 + 2 + 3);
/// assert_eq!(sum_from_to(2, 4), 2 + 3);
/// ```
#[inline]
fn sum_from_to(from: u64, to: u64) -> u64 {
    if from == 0 {
        to * (to - 1) / 2
    } else {
        (to * (to - 1) - from * (from - 1)) / 2
    }
}

/// Returns parsed files and free spaces
///
///
/// files: `Vec<index, size, ID>`
///
/// free: `Vec<index, size>`
/// ```
fn parse_input(input: &str) -> (Vec<(u64, u64, usize)>, Vec<(u64, u64)>) {
    let (_, files, frees) = input
        .trim()
        .char_indices()
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as u64))
        .fold(
            (0, Vec::new(), Vec::new()),
            |(sum, mut files, mut frees), (i, c)| {
                if c == 0 {
                    return (sum, files, frees);
                }
                match i % 2 {
                    0 => files.push((sum, c, files.len())),
                    1 => frees.push((sum, c)),
                    _ => unreachable!(),
                }

                (sum + c, files, frees)
            },
        );

    (files, frees)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut files, frees) = parse_input(input);

    // Checksum accumulator
    let mut checksum = 0;

    // First file to move, starting from rightmost
    let (mut file_index, mut file_size_remaining, mut file_id) = files.pop().unwrap();

    'outer: for (mut free_index, free_size) in frees {
        let mut free_size_remaining = free_size;
        while free_size_remaining > 0 {
            // Stop when file index is lower than free index, meaning that there is no free space between files
            if file_index < free_index {
                break 'outer;
            }

            // Check how many blocks we can move
            // The whole file if we can, if we cannot, just move part of the file that fit the free space
            let moved_blocks = free_size_remaining.min(file_size_remaining);

            // Compute checksum of moved blocks
            checksum += (file_id as u64) * sum_from_to(free_index, free_index + moved_blocks);
            
            // Keep track of how many blocks remains in the free space
            free_size_remaining -= moved_blocks;
            free_index += moved_blocks;

            // Keep track of how many blocks remains in the file space
            file_size_remaining -= moved_blocks;
            
            // If the whole file has moved, read the next one
            if file_size_remaining == 0 {
                (file_index, file_size_remaining, file_id) = files.pop().unwrap_or((0, 0, 0));
            }
        }
    }

    // Add the checksum of the rest of the current file
    checksum += (file_id as u64) * sum_from_to(file_index, file_index + file_size_remaining);

    // Add the checksum of all the initial files
    checksum += files
        .into_iter()
        .map(|(file_index, count, file_content)| {
            (file_content as u64) * sum_from_to(file_index, file_index + count)
        })
        .sum::<u64>();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut files, frees) = parse_input(input);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_from_to() {
        assert_eq!(sum_from_to(0, 4), 1 + 2 + 3);
        assert_eq!(sum_from_to(1, 4), 1 + 2 + 3);
        assert_eq!(sum_from_to(9, 10), 9);
        assert_eq!(sum_from_to(9, 11), 9 + 10);
        assert_eq!(sum_from_to(9, 12), 9 + 10 + 11);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
