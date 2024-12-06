advent_of_code::solution!(6);
use std::collections::HashSet;
use std::hash::Hash;

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new(input);
    map.walk(Direction{dx: 0, dy: -1});
    Some(map.visited.len() as u32)
}

struct Map {
    current_position: Position,
    bounds: (i32, i32),
    obstacles: Vec<Position>,
    visited: HashSet<Position>,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn one_step(&self, direction: &Direction) -> Self {
        Position { x: self.x + direction.dx, y: self.y + direction.dy }
    }
}

#[derive(Debug)]
struct Direction {
    dx: i32,
    dy: i32,
}

enum WalkResult {
    Okay,
    OutOfBounds,
    Obstacle
}

impl Map {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;

        let obstacles: Vec<Position> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| Position{x: x as i32, y: y as i32})
            })
            .collect();

        let start_position: Position = lines
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .position(|c| c == '^')
                    .map(|x| Position{x: x as i32, y: y as i32})
            })
            .expect("Starting position '^' not found");
        let mut visited = HashSet::new();
        visited.insert(start_position.clone());

        Map {
            current_position: start_position,
            bounds: (width, height),
            obstacles,
            visited,
        }
    }

    fn walk_result(&mut self, direction: &Direction) -> WalkResult {
        let new_position = self.current_position.one_step(direction);
        if self.obstacles.contains(&new_position) {
            return WalkResult::Obstacle
        }
        if new_position.x >= self.bounds.0 || new_position.y >= self.bounds.1 || new_position.x < 0 || new_position.y < 0 {
            return WalkResult::OutOfBounds
        }
        self.current_position = new_position;
        WalkResult::Okay
    }

    fn walk(&mut self, direction: Direction) {
        let mut current_direction = direction;
        loop {
            match self.walk_result(&current_direction) {
                WalkResult::Okay => {
                    self.visited.insert(self.current_position.clone());
                }
                WalkResult::Obstacle => {
                    current_direction = Direction {
                        dx: -current_direction.dy,
                        dy: current_direction.dx
                    };
                }
                WalkResult::OutOfBounds => {
                    return;
                }
            }
        }
    }
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
