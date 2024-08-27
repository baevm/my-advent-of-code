use std::{collections::HashMap, path::Path};

use crate::helpers::file::read_lines;

// https://adventofcode.com/2021/day/3
pub fn solve() {
    let path = Path::new("src/data/03.txt");

    let lines = read_lines(path.to_str().unwrap());

    part1(&lines);

    part2(&lines);
}

#[derive(Debug)]
struct PositionCounter {
    zeroes: i64,
    ones: i64,
}

fn part1(lines: &Vec<String>) {
    let hmap = position_bit_count(&lines);

    let n = lines.first().unwrap().len();

    let mut gamma_str = vec![""; n];
    let mut epsilon_str = vec![""; n];

    for (pos, counter) in hmap {
        match counter.ones > counter.zeroes {
            true => {
                gamma_str[pos] = "1";
                epsilon_str[pos] = "0"
            }
            false => {
                gamma_str[pos] = "0";
                epsilon_str[pos] = "1"
            }
        }
    }

    let gamma_num = i64::from_str_radix(&gamma_str.join(""), 2).expect("not a binary num");
    let epsilon_num = i64::from_str_radix(&epsilon_str.join(""), 2).expect("not a binary num");

    let res = gamma_num * epsilon_num;

    println!("Part 1: {}", res);
}

fn part2(lines: &Vec<String>) {
    let hmap = position_bit_count(&lines);

    let n = lines.first().unwrap().len();

    let mut oxygen_rating: Vec<String> = vec![];
    let mut scrubber_rating: Vec<String> = vec![];

    let first_pos_num = hmap.get(&0).unwrap();

    let starter_num = match first_pos_num.ones > first_pos_num.zeroes {
        true => '1',
        false => '0',
    };

    lines.iter().for_each(|line| {
        match line.starts_with(starter_num) {
            true => oxygen_rating.push(line.to_string()),
            false => scrubber_rating.push(line.to_string()),
        };
    });

    let mut hmap_oxygen = position_bit_count(&oxygen_rating);
    let mut hmap_scrubber = position_bit_count(&scrubber_rating);

    for i in 1..n {
        let num_pos_oxygen = hmap_oxygen.get(&i).expect("not found position oxygen");

        let oxygen_pos_bit = match num_pos_oxygen.ones > num_pos_oxygen.zeroes
            || num_pos_oxygen.ones == num_pos_oxygen.zeroes
        {
            true => '1',
            false => '0',
        };

        if oxygen_rating.len() > 1 {
            oxygen_rating.retain(|num| {
                let is_ok = num.chars().nth(i).unwrap() == oxygen_pos_bit;
                is_ok
            });

            hmap_oxygen = position_bit_count(&oxygen_rating);
        }

        let num_pos_scrubber = hmap_scrubber.get(&i).expect("not found position scrubber");

        let scrubber_pos_bit = match num_pos_scrubber.ones > num_pos_scrubber.zeroes
            || num_pos_scrubber.ones == num_pos_scrubber.zeroes
        {
            true => '0',
            false => '1',
        };

        if scrubber_rating.len() > 1 {
            scrubber_rating.retain(|num| {
                let is_ok = num.chars().nth(i).unwrap() == scrubber_pos_bit;
                is_ok
            });

            hmap_scrubber = position_bit_count(&scrubber_rating);
        }
    }

    let oxygen_rating_num =
        i64::from_str_radix(&oxygen_rating.join(""), 2).expect("not a binary num");
    let scrubber_rating_num =
        i64::from_str_radix(&scrubber_rating.join(""), 2).expect("not a binary num");

    let res = oxygen_rating_num * scrubber_rating_num;

    println!("Part 2: {}", res);
}

fn position_bit_count(lines: &Vec<String>) -> HashMap<usize, PositionCounter> {
    let mut hmap: HashMap<usize, PositionCounter> = HashMap::new();

    let n = lines.first().unwrap().len();

    for i in 0..n {
        hmap.insert(i, PositionCounter { zeroes: 0, ones: 0 });
    }

    for line in lines {
        for (pos, ch) in line.chars().enumerate() {
            match ch {
                '1' => {
                    hmap.entry(pos).and_modify(|counter| counter.ones += 1);
                }
                '0' => {
                    hmap.entry(pos).and_modify(|counter| counter.zeroes += 1);
                }
                _ => unreachable!(),
            }
        }
    }

    hmap
}
