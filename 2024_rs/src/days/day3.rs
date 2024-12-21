use regex::Regex;

use crate::helpers::{file::read_file_as_str, runner::run_with_timing};

// https://adventofcode.com/2024/day/3
pub fn solve() {
    let line = read_file_as_str("./src/data/03.txt");

    run_with_timing(|| part1(&line), 1);
    run_with_timing(|| part2(&line), 2);
}

fn part1(line: &str) {
    let mut res = 0;

    // matches mul(11,8)
    let mul_regex = Regex::new(r"mul\(([0-9]{1,},[0-9]{1,})\)").unwrap();

    // matches 11,8
    let instruction_regex = Regex::new(r"[0-9]*,[0-9]*").unwrap();

    let mul_strs: Vec<&str> = mul_regex.find_iter(line).map(|c| c.as_str()).collect();

    for mul_str in mul_strs {
        let instructions: Vec<i64> = instruction_regex
            .find_iter(mul_str)
            .flat_map(|c| c.as_str().split(",").map(|s| s.parse::<i64>().unwrap()))
            .collect();

        res += instructions[0] * instructions[1];
    }

    println!("Part 1: {}", res);
}

fn part2(line: &str) {
    let mut res = 0;

    // matches don't() | mul(11,8) | do()
    let mul_regex = Regex::new(r"don't\(\)|do\(\)|mul\(([0-9]{1,},[0-9]{1,})\)").unwrap();

    // matches 11,8
    let instruction_regex = Regex::new(r"[0-9]*,[0-9]*").unwrap();

    let mul_strs: Vec<&str> = mul_regex.find_iter(line).map(|c| c.as_str()).collect();

    let mut is_disabled = false;

    for mul_str in mul_strs {
        match mul_str {
            "don't()" => {
                is_disabled = true;
            }
            "do()" => {
                is_disabled = false;
            }
            instruction if !is_disabled => {
                let instructions: Vec<i64> = instruction_regex
                    .find_iter(instruction)
                    .flat_map(|c| c.as_str().split(",").map(|s| s.parse::<i64>().unwrap()))
                    .collect();

                res += instructions[0] * instructions[1];
            }
            _ => continue,
        }
    }

    println!("Part 2: {}", res);
}
