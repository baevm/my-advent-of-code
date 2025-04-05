use std::i64;

use itertools::Itertools;

advent_of_code::solution!(6);

fn convert_coords(coords: &str) -> (u16, u16) {
    coords
        .split(",")
        .map(|c| c.parse::<u16>().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<i64> {
    let lines = input.lines();
    let mut grid: [[bool; 1000]; 1000] = [[false; 1000]; 1000];

    for line in lines {
        //  0    1   2    3       4
        // turn on 0,0 through 999,999
        //  0       1      2         3
        // toggle 756,965 through 812,992
        let words: Vec<_> = line.split_whitespace().collect();

        let command = *words.first().unwrap();

        match command {
            "toggle" => {
                let start = convert_coords(words.get(1).expect("wtf"));
                let end = convert_coords(words.get(3).expect("wtf"));

                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        grid[i as usize][j as usize] = !grid[i as usize][j as usize];
                    }
                }
            }
            "turn" => {
                let start = convert_coords(words.get(2).expect("wtf"));
                let end = convert_coords(words.get(4).expect("wtf"));
                let is_turn_on_command = match *words.get(1).expect("wtf") {
                    "on" => true,
                    "off" => false,
                    _ => unreachable!(),
                };

                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        grid[i as usize][j as usize] = is_turn_on_command;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let mut lit_lights = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] {
                lit_lights += 1;
            }
        }
    }

    Some(lit_lights)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut grid: [[u64; 1000]; 1000] = [[0; 1000]; 1000];

    for line in lines {
        //  0    1   2    3       4
        // turn on 0,0 through 999,999
        //  0       1      2         3
        // toggle 756,965 through 812,992
        let words: Vec<_> = line.split_whitespace().collect();

        let command = *words.first().unwrap();

        match command {
            "toggle" => {
                let start = convert_coords(words.get(1).expect("wtf"));
                let end = convert_coords(words.get(3).expect("wtf"));

                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        grid[i as usize][j as usize] += 2;
                    }
                }
            }
            "turn" => {
                let start = convert_coords(words.get(2).expect("wtf"));
                let end = convert_coords(words.get(4).expect("wtf"));
                let is_turn_on_command = match *words.get(1).expect("wtf") {
                    "on" => true,
                    "off" => false,
                    _ => unreachable!(),
                };

                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        if is_turn_on_command {
                            grid[i as usize][j as usize] += 1;
                        } else if grid[i as usize][j as usize] > 0 {
                            grid[i as usize][j as usize] -= 1;
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let mut brightness = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            brightness += grid[i][j];
        }
    }

    Some(brightness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
