advent_of_code::solution!(9);

#[inline]
fn sum_from_to(from: u64, to: u64) -> u64 {
    if from == 0 {
        to * (to - 1) / 2
    } else {
        (to * (to - 1) - from * (from - 1)) / 2
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, mut files, frees) = input
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

    // println!("{:?}", files);
    let (mut file_index, mut count, mut file_content) = files.pop().unwrap();

    let mut checksum = 0;
    'outer: for (mut free_index, space) in frees {
        let mut blocks_to_move = space;
        while blocks_to_move > 0 {
            if file_index < free_index {
                break 'outer;
            }

            let moved_blocks = blocks_to_move.min(count);
            blocks_to_move -= moved_blocks;
            count -= moved_blocks;

            // println!("moving {moved_blocks} from {file_content} ({file_index}) zone to {free_index} zone");
            let moved_file_checksum =
                (file_content as u64) * sum_from_to(free_index, free_index + moved_blocks);
            // println!(
            //     "{file_content} * sum_from_to({free_index}, {free_index} + {moved_blocks})"
            // );
            // println!("Adding {moved_file_checksum} to checksum");
            checksum += moved_file_checksum;
            free_index += moved_blocks;
            if count == 0 {
                (file_index, count, file_content) = files.pop().unwrap_or((0, 0, 0));
            }
        }
    }

    // println!("({file_index}, {count}, {file_content})");
    // println!(
    //     "{}",
    //     (file_content as u64) * sum_from_to(file_index, file_index + count)
    // );
    checksum += (file_content as u64) * sum_from_to(file_index, file_index + count);

    checksum += files
        .into_iter()
        .map(|(file_index, count, file_content)| {
            (file_content as u64) * sum_from_to(file_index, file_index + count)
        })
        .sum::<u64>();

    // println!("{files:?}");
    // println!("{checksum:?}");

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
