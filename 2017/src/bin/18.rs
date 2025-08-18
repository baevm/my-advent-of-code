use std::collections::HashMap;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i64> {
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut last_played_freq: i64 = 0;

    let mut idx = 0;
    let lines: Vec<Vec<String>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.to_string())
                .collect()
        })
        .collect();

    while idx < lines.len() {
        let parts = &lines[idx];

        let command = parts[0].as_str();
        let register = parts[1].clone();

        match command {
            "snd" => {
                last_played_freq = *registers.get(&register).unwrap();
                idx += 1;
            }
            "set" => {
                let value: i64 = parts[2]
                    .parse()
                    .unwrap_or_else(|_| *registers.get(&parts[2]).unwrap());

                registers.insert(register, value);
                idx += 1;
            }
            "add" | "mul" | "mod" => {
                let value: i64 = parts[2]
                    .parse()
                    .unwrap_or_else(|_| *registers.get(&parts[2]).unwrap());

                registers
                    .entry(register)
                    .and_modify(|x| match command {
                        "add" => *x += value,
                        "mul" => *x *= value,
                        "mod" => *x %= value,
                        _ => unreachable!(),
                    })
                    .or_insert(0);

                idx += 1;
            }
            "rcv" => {
                let register_value = *registers.get(&register).unwrap();

                if register_value != 0 {
                    return Some(last_played_freq);
                }
                idx += 1;
            }
            "jgz" => {
                let value: i64 = parts[2]
                    .parse()
                    .unwrap_or_else(|_| *registers.get(&parts[2]).unwrap());

                let possible_digit_register = register.parse::<i64>();

                let register_value = if possible_digit_register.is_ok() {
                    possible_digit_register.unwrap()
                } else {
                    *registers.get(&register).unwrap()
                };

                idx = if register_value > 0 {
                    safe_subtract(idx, value).expect("undeflow")
                } else {
                    idx + 1
                }
            }
            _ => unreachable!(),
        }
    }

    None
}

fn safe_subtract(index: usize, offset: i64) -> Option<usize> {
    if offset < 0 {
        index.checked_sub((-offset) as usize)
    } else {
        index.checked_add(offset as usize)
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    None
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
