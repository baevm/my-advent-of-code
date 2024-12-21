use crate::helpers::{file::read_file_lines, runner::run_with_timing};

// https://adventofcode.com/2024/day/2
pub fn solve() {
    let lines = read_file_lines("./src/data/02.txt");

    run_with_timing(|| part1(&lines), 1);
    run_with_timing(|| part2(&lines), 2);
}

fn part1(lines: &Vec<String>) {
    let mut safe_reports = 0;

    for line in lines {
        let levels: Vec<i64> = line
            .split_whitespace()
            .map(|val| val.parse::<i64>().unwrap())
            .collect();

        if is_levels_safe(levels) {
            safe_reports += 1;
        }
    }

    println!("Part 1: {}", safe_reports);
}

fn is_levels_safe(levels: Vec<i64>) -> bool {
    let is_increasing = levels[0] < levels[1];

    for (prev, curr) in levels.iter().zip(levels.iter().skip(1)) {
        if (prev - curr).abs() > 3
            || prev == curr
            || (is_increasing && curr < prev)
            || (!is_increasing && curr > prev)
        {
            return false;
        }
    }

    return true;
}

fn part2(lines: &Vec<String>) {
    let mut safe_reports = 0;

    for line in lines {
        let mut tolerable = false;

        let levels: Vec<i64> = line
            .split_whitespace()
            .map(|val| val.parse::<i64>().unwrap())
            .collect();

        for i in 0..levels.len() {
            let mut new_levels = levels.clone();
            new_levels.remove(i);

            if is_levels_safe(new_levels) {
                tolerable = true;
                break;
            }
        }

        if is_levels_safe(levels) || tolerable {
            safe_reports += 1;
        }
    }

    println!("Part 2: {}", safe_reports);
}
