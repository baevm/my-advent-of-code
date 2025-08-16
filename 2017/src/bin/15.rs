use itertools::Itertools;

advent_of_code::solution!(15);

const A_FACTOR: i64 = 16807;
const B_FACTOR: i64 = 48271;
const DIVIDER: i64 = 2147483647;
const LAST_16_BITS: i64 = 0xFFFF;

pub fn part_one(input: &str) -> Option<i64> {
    let (mut a, mut b): (i64, i64) = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|last_num| last_num.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut result = 0;

    for _ in 0..40_000_000 {
        a = (a * A_FACTOR) % DIVIDER;
        b = (b * B_FACTOR) % DIVIDER;

        if (a & LAST_16_BITS) == (b & LAST_16_BITS) {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (mut a, mut b): (i64, i64) = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|last_num| last_num.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut result = 0;

    for _ in 0..5_000_000 {
        loop {
            a = (a * A_FACTOR) % DIVIDER;

            if a % 4 == 0 {
                break;
            }
        }

        loop {
            b = (b * B_FACTOR) % DIVIDER;

            if b % 8 == 0 {
                break;
            }
        }

        if (a & LAST_16_BITS) == (b & LAST_16_BITS) {
            result += 1;
        }
    }

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
