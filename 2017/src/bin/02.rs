advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i64> {
    let lines = input.lines();

    let result = lines
        .map(|line| {
            line.split_whitespace()
                .map(|ch| ch.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .fold(0, |acc, val| {
            let max_num = val.iter().max().unwrap();
            let min_num = val.iter().min().unwrap();

            acc + max_num - min_num
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines = input.lines();

    let mut result = 0;

    lines
        .map(|line| {
            line.split_whitespace()
                .map(|ch| ch.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .for_each(|digit_arr| {
            digit_arr.iter().for_each(|num| {
                let even_divider = digit_arr
                    .iter()
                    .find(|div_num| num % *div_num == 0 && num != *div_num);

                if let Some(even_divider) = even_divider {
                    let max = num.max(even_divider);
                    let min = num.min(even_divider);

                    result += max / min;
                }
            });
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
