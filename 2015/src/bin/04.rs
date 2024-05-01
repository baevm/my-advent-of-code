use crypto::{digest::Digest, md5::Md5};

// https://adventofcode.com/2015/day/4
advent_of_code::solution!(4);

fn find_hash_with_zeroes(input: &str, lead_zeroes: &str) -> Option<i64> {
    // найти минимально возможное число, чтобы hexadecimal md5 hash для заданной строки + числа начинался с заданного числа нулей
    let mut md5_hasher = Md5::new();

    let mut i: i64 = 0;
    loop {
        let curr_str = input.to_owned() + &i.to_string();
        md5_hasher.input_str(&curr_str);

        let result_hash = md5_hasher.result_str();

        if result_hash.starts_with(lead_zeroes) {
            return Some(i);
        }

        if i == i64::MAX {
            break;
        }

        md5_hasher.reset();
        i += 1;
    }

    None
}

pub fn part_one(input: &str) -> Option<i64> {
    return find_hash_with_zeroes(input, "00000");
}

pub fn part_two(input: &str) -> Option<i64> {
    return find_hash_with_zeroes(input, "000000");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
