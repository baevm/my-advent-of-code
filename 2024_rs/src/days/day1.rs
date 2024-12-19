use std::{
    collections::{BinaryHeap, HashMap},
    path::Path,
};

use itertools::Itertools;

use crate::helpers::file::read_lines;

// https://adventofcode.com/2024/day/1
pub fn solve() {
    let path = Path::new("src/data/01.txt");
    let lines = read_lines(path.to_str().unwrap());

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut bin_heap_lefts: BinaryHeap<i64> = BinaryHeap::new();
    let mut bin_heap_rights: BinaryHeap<i64> = BinaryHeap::new();

    for line in lines {
        let nums: (i64, i64) = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        bin_heap_lefts.push(nums.0);
        bin_heap_rights.push(nums.1);
    }

    let mut res = 0;

    while bin_heap_lefts.len() > 0 {
        let min_from_left = bin_heap_lefts.pop().unwrap();
        let min_from_right = bin_heap_rights.pop().unwrap();

        let dist = (min_from_left - min_from_right).abs();
        res += dist;
    }

    println!("Part 1: {}", res);
}

fn part2(lines: &Vec<String>) {
    let mut nums_in_left: Vec<i64> = Vec::new();
    let mut count_in_right: HashMap<i64, i64> = HashMap::new();

    for line in lines {
        let nums: (i64, i64) = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        nums_in_left.push(nums.0);

        count_in_right
            .entry(nums.1)
            .and_modify(|prev| *prev += 1)
            .or_insert(1);
    }

    let mut res = 0;

    for num_left in nums_in_left {
        if let Some(val) = count_in_right.get(&num_left) {
            let similarity_score = num_left * val;
            res += similarity_score;
        }
    }

    println!("Part 2: {}", res);
}
