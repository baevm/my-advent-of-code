use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
enum Action {
    Inc,
    Dec,
}

#[derive(Debug)]
enum Operator {
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

#[derive(Debug)]
struct Register {
    action: Action,
    change_amount: i64,
    if_target: String,
    if_operator: Operator,
    if_comparer: i64,
}

impl Register {
    fn new(parts: &[String]) -> Self {
        let action = match parts[1].as_str() {
            "inc" => Action::Inc,
            "dec" => Action::Dec,
            _ => unreachable!(),
        };
        let amount = parts[2].parse::<i64>().unwrap();

        let if_target = parts[4].clone();

        let if_operator = match parts[5].as_str() {
            "==" => Operator::Eq,
            "!=" => Operator::NotEq,
            ">" => Operator::Gt,
            "<" => Operator::Lt,
            ">=" => Operator::GtEq,
            "<=" => Operator::LtEq,
            _ => unreachable!(),
        };

        let if_comparer = parts[6].parse::<i64>().unwrap();

        Register {
            action,
            change_amount: amount,
            if_target,
            if_operator,
            if_comparer,
        }
    }
}

pub fn solve(input: &str, is_part_2: bool) -> Option<i64> {
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut max_alltime_amount = 0;

    let lines: Vec<Vec<String>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.to_string())
                .collect()
        })
        .collect();

    lines.iter().for_each(|parts| {
        let target = parts[0].clone();
        registers.entry(target).or_insert(0);
    });

    lines.iter().for_each(|parts| {
        let target = parts[0].clone();

        let register = Register::new(parts);

        let to_compare = *registers.get(&register.if_target).unwrap();

        let is_ok = match register.if_operator {
            Operator::Eq => to_compare == register.if_comparer,
            Operator::NotEq => to_compare != register.if_comparer,
            Operator::Lt => to_compare < register.if_comparer,
            Operator::Gt => to_compare > register.if_comparer,
            Operator::LtEq => to_compare <= register.if_comparer,
            Operator::GtEq => to_compare >= register.if_comparer,
        };

        if is_ok {
            registers.entry(target.clone()).and_modify(|prev| {
                match register.action {
                    Action::Inc => *prev += register.change_amount,
                    Action::Dec => *prev -= register.change_amount,
                }

                max_alltime_amount = *prev.max(&mut max_alltime_amount)
            });
        }
    });

    let max_register_amount = *registers.values().max().unwrap();

    if !is_part_2 {
        Some(max_register_amount)
    } else {
        Some(max_alltime_amount)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve(input, true)
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
