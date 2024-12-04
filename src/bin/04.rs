advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    // direction: dx, dy
    const DIRECTIONS: [(i32, i32); 8] = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
    ];

    let count = (0..grid.len())
        .flat_map(|i| (0..grid[0].len()).map(move |j| (i, j)))
        .flat_map(|(i, j)| DIRECTIONS.iter().map(move |(dx, dy)| (i, j, dx, dy)))
        .filter(|(i, j, dx, dy)| check_xmas(&grid, i, j, dx, dy))
        .count() as u32;

    Some(count)
}

fn check_xmas(grid: &[Vec<char>], i: &usize, j: &usize, dx: &i32, dy: &i32) -> bool {
    let xmas = ['X', 'M', 'A', 'S'];
    let num_rows = grid.len() as i32;
    let num_cols = grid[0].len() as i32;

    xmas.iter().enumerate().all(|(num, &ch)| {
        let x = *i as i32 + num as i32 * dx;
        let y = *j as i32 + num as i32 * dy;
        if x < 0 || x >= num_rows || y < 0 || y >= num_cols {
            false
        } else {
            //println!("i: {}, j: {}, num: {}, dx: {}, dy: {}", i, j, num, dx, dy);
            //println!("x: {}, rows: {}, y: {}, cols: {}", x, num_rows, y, num_cols);
            ch == grid[x as usize][y as usize]
        }
    })
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
