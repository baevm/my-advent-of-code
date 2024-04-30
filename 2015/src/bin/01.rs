advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let mut result = 0;

    for parenths in input.chars() {
        match parenths {
            '(' => result += 1,
            ')' => result -= 1,
            _ => unreachable!(),
        }
    }

    return Some(result);
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result: i64 = 0;

    for (i, parenths) in input.chars().enumerate() {
        match parenths {
            '(' => result += 1,
            ')' => result -= 1,
            _ => unreachable!(),
        }

        if result == -1 {
            return Some(i + 1);
        }
    }

    return None;
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
