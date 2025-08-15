use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(16);

pub fn solve(input: &str, amount: u64) -> String {
    let mut program: VecDeque<char> = "abcdefghijklmnop".chars().collect();

    let instructions: Vec<&str> = input.split(',').collect();

    let mut seen: HashMap<String, u64> = HashMap::new();
    let mut history: Vec<String> = vec![];

    for i in 0..amount {
        let curr_state: String = program.iter().collect();

        if let Some(first_occur) = seen.get(&curr_state) {
            let cycle_len = i - first_occur;
            let remaining_iters = amount - i;
            let final_state = first_occur + (remaining_iters % cycle_len);

            return history[final_state as usize].clone();
        }

        seen.insert(curr_state.clone(), i);
        history.push(curr_state);

        instructions.iter().for_each(|instruction| {
            let mut chars = instruction.chars();

            match chars.next().unwrap() {
                's' => {
                    let spin_size: usize = chars.as_str().parse().unwrap();
                    program.rotate_right(spin_size);
                }
                'x' => {
                    let (a, b): (usize, usize) = chars
                        .as_str()
                        .split('/')
                        .map(|item| item.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    program.swap(a, b);
                }
                'p' => {
                    let (a, b): (char, char) = chars
                        .as_str()
                        .split('/')
                        .map(|p| p.chars().next().unwrap())
                        .collect_tuple()
                        .unwrap();

                    let a_idx = program.iter().position(|x| *x == a).unwrap();
                    let b_idx = program.iter().position(|x| *x == b).unwrap();

                    program.swap(a_idx, b_idx);
                }
                _ => unreachable!(),
            };
        });
    }

    let result: String = program.into_iter().collect();

    result
}

pub fn part_one(input: &str) -> Option<String> {
    Some(solve(input, 1))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(solve(input, 1_000_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
