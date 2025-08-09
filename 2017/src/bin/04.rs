use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<i64> {
    let mut result: i64 = 0;

    let lines = input.lines();

    lines.for_each(|passphrase| {
        let mut uniq: HashSet<&str> = HashSet::new();

        for word in passphrase.split_whitespace() {
            if uniq.contains(word) {
                return;
            }

            uniq.insert(word);
        }

        result += 1;
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut result: i64 = 0;

    let lines = input.lines();

    lines.for_each(|passphrase| {
        let mut uniq: HashSet<String> = HashSet::new();

        for word in passphrase
            .split_whitespace()
            .map(|word| word.chars().sorted().collect::<String>())
        {
            if uniq.contains(&word) {
                return;
            }

            uniq.insert(word);
        }

        result += 1;
    });

    Some(result)
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
