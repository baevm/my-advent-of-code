use std::path::Path;

use crate::helpers::{data_structs::Point, file::read_lines};

// https://adventofcode.com/2021/day/2
pub fn solve() {
    let path = Path::new("src/data/02.txt");

    let lines = read_lines(path.to_str().unwrap());

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut pos = Point::default();

    lines
        .iter()
        .map(|line| line.split_whitespace())
        .for_each(|mut command| {
            let course = command.next().unwrap();
            let units: i64 = command.next().unwrap().parse().unwrap();

            match course {
                "forward" => pos.x += units,
                "up" => pos.y += units,
                "down" => pos.y -= units,
                _ => unreachable!(),
            }

            ()
        });

    let total = (pos.x * pos.y).abs();

    println!("Part 1: {}", total);
}

fn part2(lines: &Vec<String>) {
    let mut pos = PointWithAim::default();

    lines
        .iter()
        .map(|line| line.split_whitespace())
        .for_each(|mut command| {
            let course = command.next().unwrap();
            let units: i64 = command.next().unwrap().parse().unwrap();

            match course {
                "forward" => {
                    pos.point.x += units;
                    pos.point.y += pos.aim * units;
                }
                "up" => pos.aim -= units,
                "down" => pos.aim += units,
                _ => unreachable!(),
            }

            ()
        });

    let total = (pos.point.x * pos.point.y).abs();

    println!("Part 2: {}", total);
}

#[derive(Default, Debug)]
struct PointWithAim {
    point: Point,
    aim: i64,
}
