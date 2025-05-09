use std::collections::HashMap;

use fancy_regex::Regex;

advent_of_code::solution!(16);

fn box_fn<F>(f: F) -> Box<dyn Fn(i64) -> bool>
where
    F: Fn(i64) -> bool + 'static,
{
    Box::new(f)
}

pub fn part_one(input: &str) -> Option<String> {
    return solve(input, false);
}

pub fn part_two(input: &str) -> Option<String> {
    return solve(input, true);
}

fn solve(input: &str, is_part_2: bool) -> Option<String> {
    let lines = input.lines();

    let checks = if !is_part_2 {
        HashMap::from([
            ("children".to_string(), box_fn(|n: i64| n == 3)),
            ("cats".to_string(), box_fn(|n: i64| n == 7)),
            ("samoyeds".to_string(), box_fn(|n: i64| n == 5)),
            ("pomeranians".to_string(), box_fn(|n: i64| n == 3)),
            ("akitas".to_string(), box_fn(|n: i64| n == 0)),
            ("vizslas".to_string(), box_fn(|n: i64| n == 0)),
            ("goldfish".to_string(), box_fn(|n: i64| n == 5)),
            ("trees".to_string(), box_fn(|n: i64| n == 3)),
            ("cars".to_string(), box_fn(|n: i64| n == 2)),
            ("perfumes".to_string(), box_fn(|n: i64| n == 1)),
        ])
    } else {
        HashMap::from([
            ("children".to_string(), box_fn(|n: i64| n == 3)),
            ("cats".to_string(), box_fn(|n: i64| n > 7)),
            ("samoyeds".to_string(), box_fn(|n: i64| n == 5)),
            ("pomeranians".to_string(), box_fn(|n: i64| n < 3)),
            ("akitas".to_string(), box_fn(|n: i64| n == 0)),
            ("vizslas".to_string(), box_fn(|n: i64| n == 0)),
            ("goldfish".to_string(), box_fn(|n: i64| n < 5)),
            ("trees".to_string(), box_fn(|n: i64| n > 3)),
            ("cars".to_string(), box_fn(|n: i64| n == 2)),
            ("perfumes".to_string(), box_fn(|n: i64| n == 1)),
        ])
    };

    let item_regex = Regex::new(r"(\w+): (\d+)").unwrap();
    let sue_num = Regex::new(r"(?<=Sue )\d+").unwrap();

    // Sue 462: pomeranians: 6, cats: 2, perfumes: 6
    for line in lines {
        let mut params = item_regex.captures_iter(line);

        let mut is_found = true;

        while let Some(Ok(captures)) = params.next() {
            if captures.len() < 3 {
                continue;
            }

            let prop_name = captures.get(1).expect("property name not found").as_str();
            let prop_value: i64 = captures
                .get(2)
                .expect("property value not found")
                .as_str()
                .parse()
                .expect("failed to parse property value");

            let check_fn = checks.get(prop_name).expect("fn for property not found");

            if !check_fn(prop_value) {
                is_found = false;
                break;
            }
        }

        if is_found {
            let found_sue_num = sue_num
                .captures(line)
                .expect("failed to exec regex")
                .expect("sue number not found")
                .get(0)
                .unwrap()
                .as_str()
                .to_string();

            return Some(found_sue_num);
        }
    }

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
