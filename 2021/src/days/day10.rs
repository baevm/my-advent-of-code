use std::{collections::HashMap, path::Path};

use crate::helpers::file::read_lines;

// https://adventofcode.com/2021/day/10
pub fn solve() {
    let path = Path::new("src/data/10.txt");

    let lines = read_lines(path.to_str().unwrap());

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let mut invalids: Vec<char> = vec![];

    'outer: for line in lines {
        let mut stack: Vec<char> = vec![];

        for ch in line.chars() {
            if pairs.contains_key(&ch) {
                stack.push(ch);
            } else if stack.len() > 0 && pairs.get(stack.last().unwrap()).unwrap() == &ch {
                stack.pop();
            } else {
                invalids.push(ch);
                continue 'outer;
            }
        }
    }

    let mut score = 0;
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    for pr in invalids {
        score += scores.get(&pr).unwrap();
    }

    println!("Part 1: {}", score);
}

fn part2(lines: &Vec<String>) {
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let mut incompletes: Vec<Vec<char>> = vec![];

    'outer: for line in lines {
        let mut stack: Vec<char> = vec![];

        for ch in line.chars() {
            if pairs.contains_key(&ch) {
                stack.push(ch);
            } else if stack.len() > 0 && pairs.get(stack.last().unwrap()).unwrap() == &ch {
                stack.pop();
            } else {
                continue 'outer;
            }
        }

        incompletes.push(stack)
    }

    let scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut total_scores: Vec<i64> = vec![];

    for incomplete in incompletes {
        let mut score = 0;

        for ch in incomplete.iter().rev() {
            let missing_pair = pairs.get(&ch).unwrap();
            let score_for_parens = scores.get(missing_pair).unwrap();

            score = (score * 5) + score_for_parens
        }

        total_scores.push(score);
    }

    total_scores.sort();

    let result_score = total_scores[total_scores.len() / 2];

    println!("Part 2: {}", result_score);
}
