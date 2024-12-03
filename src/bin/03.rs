advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result: u32 = pattern.captures_iter(input)
        .map(|c| {
            let first: u32 = c.get(1).unwrap().as_str().parse().unwrap();
            let second: u32 = c.get(2).unwrap().as_str().parse().unwrap();
            first * second
        })
        .sum();
    Some(result)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
