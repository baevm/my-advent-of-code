use std::{collections::HashMap, i64, path::Path};

use crate::helpers::file::read_lines;

// https://adventofcode.com/2021/day/7
pub fn solve() {
    let path = Path::new("src/data/07.txt");

    let lines = read_lines(path.to_str().unwrap());

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut counter: HashMap<i64, i64> = HashMap::new();

    let puzzle_input: Vec<_> = lines
        .first()
        .unwrap()
        .split(",")
        .map(|x| (x).parse::<i64>().unwrap())
        .collect();

    for number in &puzzle_input {
        counter.entry(*number).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut cheapest_outcome = i64::MAX;

    for (expected_position, _) in counter {
        cheapest_outcome =
            check_fuel_outcome(&puzzle_input, expected_position).min(cheapest_outcome);
    }

    println!("Part 1: {}", cheapest_outcome);
}

fn part2(lines: &Vec<String>) {
    let puzzle_input: Vec<_> = lines
        .first()
        .unwrap()
        .split(",")
        .map(|x| (x).parse::<i64>().unwrap())
        .collect();

    let mut min_pos = i64::MAX;
    let mut max_pos = i64::MIN;

    for number in &puzzle_input {
        min_pos = min_pos.min(*number);
        max_pos = max_pos.max(*number);
    }

    let mut cheapest_outcome = i64::MAX;

    for expected_position in min_pos..=max_pos {
        cheapest_outcome =
            check_fuel_sequence(&puzzle_input, expected_position).min(cheapest_outcome);
    }

    println!("Part 2: {}", cheapest_outcome);
}

fn check_fuel_outcome(positions: &Vec<i64>, expected_position: i64) -> i64 {
    let mut res = 0;

    for pos in positions {
        res += (pos - expected_position).abs();
    }

    return res;
}

fn check_fuel_sequence(positions: &Vec<i64>, expected_position: i64) -> i64 {
    let mut res = 0;

    for pos in positions {
        let diff = (expected_position - pos).abs();

        // increasing distance 1..n
        let dist = (diff * (diff + 1)) / 2;

        res += dist;
    }

    return res;
}
