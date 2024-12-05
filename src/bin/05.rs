use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    input
        .lines()
        .filter(|line| line.contains('|'))
        .for_each(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            let key: u32 = parts[0].trim().parse().unwrap();
            let value: u32 = parts[1].trim().parse().unwrap();

            rules.entry(key).or_default().push(value);
        });

    let updates: Vec<Vec<u32>> = input
        .lines()
        .filter(|line| line.contains(','))
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let correct_updates: Vec<Vec<u32>> = updates
        .into_iter()
        .filter(|update| {
            update
                .windows(2)
                .all(|w| rules.entry(w[0]).or_default().contains(&w[1]))
        })
        .collect();

    let correct_updates_middle_sum: u32 = correct_updates
        .into_iter()
        .map(|update| update[(update.len() - 1) / 2])
        .sum();

    Some(correct_updates_middle_sum)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
