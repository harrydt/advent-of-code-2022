pub fn part_one(input: &str) -> Option<u32> {
    input.split("\n\n").map(sum_block).max()
}

pub fn sum_block(input: &str) -> u32 {
    // Split string by line
    // then parse to u32
    // then calculate sum
    input.lines().map(|w| w.parse::<u32>().unwrap()).sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut block_sums: Vec<u32> = input.split("\n\n").map(sum_block).collect();
    if block_sums.len() < 3 {
        None
    } else {
        block_sums.sort();
        Some(
            block_sums[block_sums.len() - 1]
                + block_sums[block_sums.len() - 2]
                + block_sums[block_sums.len() - 3],
        )
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_sum_block() {
        let input = "7000\n8000\n9000\n";
        assert_eq!(sum_block(input), 24000);
    }

    #[test]
    #[should_panic]
    fn test_sum_block_panic() {
        // A block should not have any empty line
        let input = "7000\n8000\n\n9000\n";
        sum_block(input);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
