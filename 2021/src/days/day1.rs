use std::path::Path;

use crate::helpers::file::read_lines;

// https://adventofcode.com/2021/day/1
pub fn solve() {
    let path = Path::new("src/data/01.txt");

    let lines = read_lines(path.to_str().unwrap());

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut res = 0;

    let ints: Vec<i64> = lines
        .iter()
        .map(|item| item.parse::<i64>().unwrap())
        .collect();

    ints.iter().zip(ints.iter().skip(1)).for_each(|(a, b)| {
        if b > a {
            res += 1
        }
    });

    println!("Part 1: {}", res);
}

fn part2(lines: &Vec<String>) {
    let mut res = 0;

    let window3_sums: Vec<i64> = lines
        .iter()
        .map(|item| item.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .windows(3)
        .map(|x| x.to_vec().iter().sum())
        .collect();

    window3_sums
        .iter()
        .zip(window3_sums.iter().skip(1))
        .for_each(|(a, b)| {
            if b > a {
                res += 1
            }
        });

    println!("Part 2: {}", res);
}
