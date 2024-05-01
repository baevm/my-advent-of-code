use std::collections::HashMap;
use std::str;

use regex::Regex;

// https://adventofcode.com/2015/day/5
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut nice_count = 0;

    let forbidden = Regex::new(r"ab|cd|pq|xy").unwrap();
    let vowels = Regex::new(r"[aeiou]").unwrap();

    for line in lines {
        let vowels_count = vowels.find_iter(line).count();
        let is_forbidden = forbidden.is_match(line);
        let is_doubles = line.as_bytes().windows(2).any(|x| x[0] == x[1]);

        if vowels_count > 2 && !is_forbidden && is_doubles {
            nice_count += 1;
        }
    }

    Some(nice_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut nice_count = 0;

    for line in lines {
        // key: pair, value: ending index
        let mut pairs: HashMap<String, usize> = HashMap::new();

        let is_triplet_found = line.as_bytes().windows(3).any(|x| x[0] == x[2]);
        let mut is_pair_found = false;

        let mut last_ch = '\0';

        for (i, ch) in line.chars().enumerate() {
            let last_plus_ch = last_ch.to_string() + &ch.to_string();
            let exist_pair = pairs.get(&last_plus_ch);

            match exist_pair {
                Some(val) => {
                    if i - 1 != *val {
                        is_pair_found = true;
                        break;
                    }
                }
                None => {
                    pairs.insert(last_plus_ch, i);
                }
            }

            last_ch = ch;
        }

        if is_pair_found && is_triplet_found {
            nice_count += 1;
        }
    }

    return Some(nice_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
