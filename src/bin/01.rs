use std::collections::HashMap;

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
    let mut right_list: HashMap<u32, u32> = HashMap::new();
    let left_list: Vec<u32> = input
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
                (Some(left), Some(right)) => {
                    right_list.entry(right).and_modify(|c| *c += 1).or_insert(1);
                    Some(left)
                }
                _ => None, // Skip lines that don't have exactly two parts
            }
        })
        .collect();
    let score = left_list
        .iter()
        .map(|x| x * right_list.get(x).unwrap_or(&0))
        .sum();
    Some(score)
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
        assert_eq!(result, Some(31));
    }
}
