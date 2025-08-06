use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    let lines = input.lines();

    lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .for_each(|line| {
            let mut line_sum = 0;

            line.iter()
                .circular_tuple_windows()
                .for_each(|(prev, curr)| {
                    if prev == curr {
                        line_sum += curr.to_digit(10).expect("failed to parse char as int");
                    }
                });

            result += line_sum;
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    let lines = input.lines();

    lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .for_each(|line| {
            let mut line_sum = 0;
            let skip_count = line.len() / 2;

            line.iter().enumerate().for_each(|(idx, curr)| {
                let idx_to_check = if idx + skip_count > line.len() - 1 {
                    idx - skip_count
                } else {
                    idx + skip_count
                };

                if line.get(idx_to_check).is_some_and(|v| v == curr) {
                    line_sum += curr.to_digit(10).expect("failed to parse char as int");
                }
            });

            result += line_sum;
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
