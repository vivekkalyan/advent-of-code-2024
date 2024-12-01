advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first_list, mut second_list): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split("   ");
            match (
                parts
                    .next()
                    .map(str::trim)
                    .and_then(|s| s.parse::<u32>().ok()),
                parts
                    .next()
                    .map(str::trim)
                    .and_then(|s| s.parse::<u32>().ok()),
            ) {
                (Some(first), Some(second)) => Some((first, second)),
                _ => None, // Skip lines that don't have exactly two parts
            }
        })
        .unzip();
    first_list.sort();
    second_list.sort();
    Some(
        first_list
            .iter()
            .zip(second_list.iter())
            .map(|(x, y)| ((*x as i32) - (*y as i32)).unsigned_abs())
            .sum(),
    )
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
