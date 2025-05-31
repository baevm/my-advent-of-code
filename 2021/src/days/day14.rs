use std::{collections::HashMap, path::Path};

use itertools::Itertools;

use crate::helpers::file::read_lines;

// https://adventofcode.com/2021/day/14
pub fn solve() {
    let path = Path::new("src/data/14.txt");
    let lines = read_lines(path.to_str().unwrap());

    let res1 = part1(&lines);
    println!("Part 1: {:?}", res1);

    let res2 = part2(&lines);
    println!("Part 2: {:?}", res2);
}

fn part1(lines: &Vec<String>) -> Option<i64> {
    calculate(lines, 10)
}

fn part2(lines: &Vec<String>) -> Option<i64> {
    calculate(lines, 40)
}

fn calculate(lines: &Vec<String>, steps: i64) -> Option<i64> {
    let mut polymers: Vec<char> = lines.first().unwrap().chars().collect();

    let mut rules: HashMap<String, char> = HashMap::new();

    for line in lines.iter().skip(2) {
        let mut rule = line.split(" -> ");
        let combination = rule.next().unwrap().to_string();
        let new_char = rule.next().unwrap().chars().next().unwrap();

        rules.insert(combination, new_char);
    }

    for step in 0..steps {
        let mut new_polymers: Vec<char> = vec![];
        let polymers_len = polymers.len();

        for (idx, (first, second)) in polymers
            .into_iter()
            .tuple_windows::<(char, char)>()
            .enumerate()
        {
            let combo = format!("{}{}", first, second);
            let new_char = rules[&combo];

            new_polymers.push(first);
            new_polymers.push(new_char);

            let t = polymers_len - 2;

            if idx == t {
                new_polymers.push(second);
            }
        }

        polymers = new_polymers;
    }

    let mut counter: HashMap<char, i64> = HashMap::new();

    for polymer in polymers {
        *counter.entry(polymer).or_insert_with(|| 0) += 1;
    }

    let most_common = counter.values().max()?;
    let least_common = counter.values().min()?;

    Some(most_common - least_common)
}
