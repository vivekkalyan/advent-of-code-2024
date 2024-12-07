advent_of_code::solution!(6);
use std::collections::HashSet;
use std::hash::Hash;

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new(input, Direction { dx: 0, dy: -1 });
    map.walk(false);
    Some(map.visited.len() as u32)
}

struct Map {
    current_position: Position,
    current_direction: Direction,
    bounds: (i32, i32),
    obstacles: Vec<Position>,
    visited: HashSet<Position>,
    taken_path: HashSet<(Position, Direction)>,
    obstacles_that_cause_loop: HashSet<Position>,
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Direction {
    dx: i32,
    dy: i32,
}

enum WalkResult {
    Okay(Position),
    OutOfBounds,
    Obstacle,
}

impl Map {
    fn new(input: &str, direction: Direction) -> Self {
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
        let mut taken_path = HashSet::new();
        taken_path.insert((start_position.clone(), direction.clone()));

        Map {
            current_position: start_position,
            current_direction: direction,
            bounds: (width, height),
            obstacles,
            visited,
            taken_path,
            obstacles_that_cause_loop: HashSet::new(),
        }
    }

    fn walk_result(&self, position: &Position, direction: &Direction) -> WalkResult {
        let next_position = position.one_step(direction);
        if self.obstacles.contains(&next_position) {
            return WalkResult::Obstacle;
        }
        if !self.is_within_bounds(&next_position) {
            return WalkResult::OutOfBounds;
        }
        WalkResult::Okay(next_position)
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.x < self.bounds.0
            && position.y < self.bounds.1
    }

    fn walk(&mut self, check_hypothetical_loops: bool) {
        loop {
            match self.walk_result(&self.current_position, &self.current_direction) {
                WalkResult::Okay(next_position) => {
                    // potential optimization: only check loops which hit an obstacles
                    // potential optimization: get full taken_path first, then run this in parallel
                    if check_hypothetical_loops && (!self.visited.contains(&next_position)) {
                        self.obstacles.push(next_position.clone());
                        if self.does_hypothetical_walk_create_loop(
                            &self.current_position,
                            &self.current_direction,
                        ) {
                            self.obstacles_that_cause_loop.insert(next_position.clone());
                        }
                        self.obstacles.pop();
                    }
                    self.current_position = next_position;
                    self.visited.insert(self.current_position.clone());
                    self.taken_path.insert((
                        self.current_position.clone(),
                        self.current_direction.clone(),
                    ));
                }
                WalkResult::Obstacle => {
                    self.current_direction = Direction {
                        dx: -self.current_direction.dy,
                        dy: self.current_direction.dx,
                    };
                }
                WalkResult::OutOfBounds => {
                    return;
                }
            }
        }
    }

    fn does_hypothetical_walk_create_loop(
        &self,
        position: &Position,
        direction: &Direction,
    ) -> bool {
        let mut new_position = position.clone();
        let mut new_direction = Direction {
            dx: -direction.dy,
            dy: direction.dx,
        };
        let mut new_taken_path = self.taken_path.clone();
        loop {
            new_taken_path.insert((new_position.clone(), new_direction.clone()));
            match self.walk_result(&new_position, &new_direction) {
                WalkResult::Okay(next_position) => {
                    new_position = next_position;
                }
                WalkResult::Obstacle => {
                    new_direction = Direction {
                        dx: -new_direction.dy,
                        dy: new_direction.dx,
                    };
                    if new_taken_path.contains(&(new_position.clone(), new_direction.clone())) {
                        return true;
                    };
                }
                WalkResult::OutOfBounds => {
                    return false;
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::new(input, Direction { dx: 0, dy: -1 });
    map.walk(true);
    Some(map.obstacles_that_cause_loop.len() as u32)
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
        assert_eq!(result, Some(6));
    }
}
