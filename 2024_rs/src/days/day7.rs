use crate::helpers::{file::read_file_lines, runner::run_with_timing};

// https://adventofcode.com/2024/day/7
pub fn solve() {
    let lines = read_file_lines("./src/data/07.txt");

    run_with_timing(|| part1(&lines), 1);
    run_with_timing(|| part2(&lines), 2);
}

fn part1(lines: &Vec<String>) {
    let mut good_vals: Vec<i64> = Vec::new();

    for line in lines {
        let task: Vec<&str> = line.split(": ").collect();

        let expected = task.first().unwrap().parse::<i64>().unwrap();
        let nums: Vec<i64> = task
            .last()
            .unwrap()
            .split(" ")
            .map(|val| val.parse::<i64>().unwrap())
            .collect();

        let is_found = backtrack_part1(0, expected, &nums, 0);

        if is_found {
            good_vals.push(expected);
        }
    }

    let res: i64 = good_vals.iter().sum();

    println!("Part 1: {}", res);
}

fn backtrack_part1(curr_val: i64, expected_val: i64, nums: &Vec<i64>, idx: usize) -> bool {
    if idx >= nums.len() {
        return curr_val == expected_val;
    }

    let sum_res = curr_val + nums[idx];

    if sum_res <= expected_val {
        if backtrack_part1(sum_res, expected_val, nums, idx + 1) {
            return true;
        }
    }

    let multiply_res = curr_val * nums[idx];

    if multiply_res <= expected_val {
        if backtrack_part1(multiply_res, expected_val, nums, idx + 1) {
            return true;
        }
    }

    return false;
}

fn part2(lines: &Vec<String>) {
    let mut good_vals: Vec<i64> = Vec::new();

    for line in lines {
        let task: Vec<&str> = line.split(": ").collect();

        let expected = task.first().unwrap().parse::<i64>().unwrap();
        let nums: Vec<i64> = task
            .last()
            .unwrap()
            .split(" ")
            .map(|val| val.parse::<i64>().unwrap())
            .collect();

        let is_found = backtrack_part2(0, expected, &nums, 0);

        if is_found {
            good_vals.push(expected);
        }
    }

    let res: i64 = good_vals.iter().sum();

    println!("Part 2: {}", res);
}

fn backtrack_part2(curr_val: i64, expected_val: i64, nums: &Vec<i64>, idx: usize) -> bool {
    if idx >= nums.len() {
        return curr_val == expected_val;
    }

    let concat_res = format!("{}{}", curr_val, nums[idx]).parse::<i64>().unwrap();

    if concat_res <= expected_val {
        if backtrack_part2(concat_res, expected_val, nums, idx + 1) {
            return true;
        }
    }

    let sum_res = curr_val + nums[idx];

    if sum_res <= expected_val {
        if backtrack_part2(sum_res, expected_val, nums, idx + 1) {
            return true;
        }
    }

    let multiply_res = curr_val * nums[idx];

    if multiply_res <= expected_val {
        if backtrack_part2(multiply_res, expected_val, nums, idx + 1) {
            return true;
        }
    }

    return false;
}
