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
    let valid_reports_count = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .filter_map(|report: Vec<i32>| {
            if is_valid_report(&report) {
                Some(report)
            } else {
                // reattempt by removing each item in report
                for i in 0..report.len() {
                    let subset: Vec<i32> = [&report[..i], &report[i + 1..]].concat();
                    if is_valid_report(&subset) {
                        return Some(report);
                    }
                }
                None
            }
        })
        .count() as u32;
    Some(valid_reports_count)
}

fn is_valid_report(report: &[i32]) -> bool {
    let diff_report: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();
    let value_range_valid = diff_report.iter().all(|&x| (1..=3).contains(&x.abs()));
    let same_sign_valid = diff_report
        .iter()
        .all(|&x| x.signum() == diff_report[0].signum());
    value_range_valid && same_sign_valid
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
        assert_eq!(result, Some(4));
    }
}
