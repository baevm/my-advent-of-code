use std::path::Path;

use crate::helpers::file::read_lines;

// https://adventofcode.com/2021/day/9
pub fn solve() {
    let path = Path::new("src/data/09.txt");

    let lines = read_lines(path.to_str().unwrap());

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let heightmap = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut low_points: Vec<i64> = vec![];

    for (i, row) in heightmap.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            let top = match i > 0 {
                true => heightmap.get(i - 1).and_then(|row| Some(row[j])),
                false => None,
            };

            let right = match j + 1 < row.len() {
                true => heightmap.get(i).and_then(|row| Some(row[j + 1])),
                false => None,
            };

            let bottom = match i + 1 < heightmap.len() {
                true => heightmap.get(i + 1).and_then(|row| Some(row[j])),
                false => None,
            };

            let left = match j > 0 {
                true => heightmap.get(i).and_then(|row| Some(row[j - 1])),
                false => None,
            };

            let mut is_lowest = true;

            if top.is_some() {
                if top.unwrap() <= *num {
                    is_lowest = false;
                }
            }

            if right.is_some() {
                if right.unwrap() <= *num {
                    is_lowest = false;
                }
            }

            if bottom.is_some() {
                if bottom.unwrap() <= *num {
                    is_lowest = false;
                }
            }

            if left.is_some() {
                if left.unwrap() <= *num {
                    is_lowest = false;
                }
            }

            if is_lowest {
                low_points.push(*num + 1);
            }
        }
    }

    let risk_level: i64 = low_points.iter().sum();

    println!("Part 1: {}", risk_level);
}

fn part2(lines: &Vec<String>) {
    let mut res = 0;

    println!("Part 2: {}", res);
}
