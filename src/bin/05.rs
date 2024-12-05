use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules = read_rules(input);
    let updates = read_updates(input);
    let correct_updates: Vec<Vec<u32>> = updates
        .into_iter()
        .filter(|update| check_update_order(update, &rules))
        .collect();

    let correct_updates_middle_sum = sum_middle_values(&correct_updates);

    Some(correct_updates_middle_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules = read_rules(input);
    let updates = read_updates(input);

    let incorrect_updates = updates
        .into_iter()
        .filter(|update| !check_update_order(update, &rules));

    let corrected_updates: Vec<Vec<u32>> = incorrect_updates
        .map(|update| {
            // get each pages rules and filter just those that appear in this update
            let mut page_rules: Vec<(u32, usize)> = update
                .iter()
                .map(|&page| {
                    (
                        page,
                        rules
                            .get(&page)
                            .unwrap_or(&Vec::new())
                            .iter()
                            .filter(|&&p| update.contains(&p))
                            .count(),
                    )
                })
                .collect();
            // sort the rules by count -> that should be the new corrected order
            page_rules.sort_by(|a, b| a.1.cmp(&b.1));
            let corrected_update: Vec<u32> = page_rules.into_iter().map(|(a, _)| a).collect();

            corrected_update
        })
        .collect();

    let correct_updates_middle_sum = sum_middle_values(&corrected_updates);

    Some(correct_updates_middle_sum)
}

fn read_rules(input: &str) -> HashMap<u32, Vec<u32>> {
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
    rules
}

fn read_updates(input: &str) -> Vec<Vec<u32>> {
    let updates: Vec<Vec<u32>> = input
        .lines()
        .filter(|line| line.contains(','))
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    updates
}

fn check_update_order(update: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    update
        .windows(2)
        .all(|w| rules.get(&w[0]).unwrap_or(&Vec::new()).contains(&w[1]))
}

fn sum_middle_values(updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .map(|update| update[(update.len() - 1) / 2])
        .sum()
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
        assert_eq!(result, Some(123));
    }
}
