use std::collections::HashSet;

// https://adventofcode.com/2015/day/3
advent_of_code::solution!(3);

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Coordinates(i32, i32);

impl Coordinates {
    fn update(&mut self, dir: char) {
        match dir {
            // Y
            '^' => self.1 += 1,
            'v' => self.1 -= 1,
            // X
            '>' => self.0 += 1,
            '<' => self.0 -= 1,
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // visited (x, y) coordinates
    let mut visited: HashSet<Coordinates> = HashSet::new();
    visited.insert(Coordinates(0, 0));

    let mut xy = Coordinates(0, 0);

    for dir in input.chars() {
        xy.update(dir);

        visited.insert(xy);
    }

    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut visited: HashSet<Coordinates> = HashSet::new();
    visited.insert(Coordinates(0, 0));

    let mut santa_coords = Coordinates(0, 0);
    let mut robot_coords = Coordinates(0, 0);

    // Iterate by 2, 1st char - santa dir, 2nd char - robot dir
    for i in (0..input.len()).step_by(2) {
        let mut dir = input.get(i..i + 2).unwrap().chars();
        let santa_dir = dir.next().unwrap();
        let robot_dir = dir.next().unwrap();

        santa_coords.update(santa_dir);
        robot_coords.update(robot_dir);

        visited.insert(santa_coords);
        visited.insert(robot_coords);
    }

    Some(visited.len())
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
        assert_eq!(result, Some(11));
    }
}
