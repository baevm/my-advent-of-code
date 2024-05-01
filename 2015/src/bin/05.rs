use std::collections::{HashMap, HashSet};
use std::str;

// https://adventofcode.com/2015/day/5
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut nice_count = 0;

    let forbidden: HashSet<String> = HashSet::from([
        ("ab").to_owned(),
        ("cd").to_owned(),
        ("pq").to_owned(),
        ("xy").to_owned(),
    ]);
    let vowels: HashSet<char> = HashSet::from_iter("aeiou".chars());

    'outer: for line in lines {
        let mut vowels_count = 0;
        let mut is_double_found = false;
        let mut last_ch: char = '\0';

        for ch in line.chars() {
            let last_plus_ch = last_ch.to_string() + &ch.to_string();
            if forbidden.contains(&last_plus_ch) {
                continue 'outer;
            }

            if vowels.contains(&ch) {
                vowels_count += 1;
            }

            if last_ch == ch {
                is_double_found = true;
            }

            last_ch = ch;
        }

        if vowels_count >= 3 && is_double_found {
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

        let mut is_triplet_found = false;
        let mut is_pair_found = false;
        let mut triplet = "".to_string();

        let mut last_ch = '\0';

        for (i, ch) in line.chars().enumerate() {
            let last_plus_ch = last_ch.to_string() + &ch.to_string();

            let exist_pair = pairs.get(&last_plus_ch);

            match exist_pair {
                Some(val) => {
                    if i - 1 != *val {
                        is_pair_found = true;
                    }
                }
                None => {
                    pairs.insert(last_plus_ch, i);
                }
            }

            if triplet.len() < 3 {
                triplet += &ch.to_string();
            }

            if triplet.len() == 3 {
                let first = triplet.get(..=0).unwrap();
                let third = triplet.get(2..).unwrap();

                if first == third {
                    is_triplet_found = true;
                }

                triplet.remove(0);
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
