advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    let diff_reports: Vec<Vec<i32>> = reports
        .into_iter()
        .map(|report| report.windows(2).map(|w| w[1] - w[0]).collect())
        .collect();
    let valid_reports: Vec<Vec<i32>> = diff_reports.into_iter().filter_map(|report| {
        let value_range_valid = report.iter().all(|&x| (1..=3).contains(&x.abs()));
        let same_sign_valid = report.iter().all(|&x| x.signum() == report[0].signum());
        if value_range_valid && same_sign_valid {
            Some(report)
        } else {
            None
        }
    }).collect();
    Some(valid_reports.len() as u32)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
