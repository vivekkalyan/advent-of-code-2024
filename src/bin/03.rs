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
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();

    let do_positions: Vec<usize> = do_pattern.find_iter(input).map(|m| m.start()).collect();
    let dont_positions: Vec<usize> = dont_pattern.find_iter(input).map(|m| m.start()).collect();
    let result_positions: Vec<(usize, u32)> = mul_pattern
        .find_iter(input)
        .zip(mul_pattern.captures_iter(input))
        .map(|(m, c)| {
            let first: u32 = c.get(1).unwrap().as_str().parse().unwrap();
            let second: u32 = c.get(2).unwrap().as_str().parse().unwrap();
            (m.start(), first * second)
        })
        .collect();

    let mut toggles: Vec<(usize, bool)> = Vec::new();
    toggles.extend(do_positions.iter().map(|&pos| (pos, true)));
    toggles.extend(dont_positions.iter().map(|&pos| (pos, false)));
    toggles.sort_by_key(|&(pos, _)| pos);

    let mut filtered_result: u32 = 0;
    let mut is_active = true;

    for (pos, value) in result_positions {
        for &(toggle_pos, toggle_state) in &toggles {
            if toggle_pos <= pos {
                is_active = toggle_state;
            } else {
                break;
            }
        }

        if is_active {
            filtered_result += value;
        }
    }
    Some(filtered_result)
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
        assert_eq!(result, Some(48));
    }
}
