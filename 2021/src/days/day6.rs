use std::{collections::VecDeque, path::Path};

use crate::helpers::file::read_lines;

// https://adventofcode.com/2021/day/6
pub fn solve() {
    let path = Path::new("src/data/06.txt");
    let lines = read_lines(path.to_str().unwrap());

    let res1 = part1(&lines);
    println!("Part 1: {:?}", res1);

    let res2 = part2(&lines);
    println!("Part 2: {:?}", res2);
}

fn part1(lines: &Vec<String>) -> Option<usize> {
    calculate_fishes(lines.first().unwrap().to_string(), 80)
}

fn part2(lines: &Vec<String>) -> Option<usize> {
    calculate_fishes(lines.first().unwrap().to_string(), 256)
}

fn calculate_fishes(line: String, days: i64) -> Option<usize> {
    let lanternfishes: Vec<usize> = line
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut fish_data: VecDeque<usize> = vec![0; 9]
        .iter()
        .enumerate()
        .map(|(idx, _)| lanternfishes.iter().filter(|fish| **fish == idx).count())
        .collect();

    for _ in 0..days {
        let new_fish = fish_data.pop_front().unwrap();
        fish_data.push_back(new_fish);
        fish_data[6] += new_fish;
    }

    let res = fish_data.iter().sum();

    return Some(res);
}
